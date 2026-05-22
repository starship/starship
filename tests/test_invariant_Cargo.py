import pytest
import re
import os
import tempfile
import subprocess


# Adversarial payloads targeting git path traversal, credential exposure,
# and repository manipulation vulnerabilities similar to gitoxide CVEs
ADVERSARIAL_PAYLOADS = [
    # Path traversal attempts
    "../../../etc/passwd",
    "..\\..\\..\\windows\\system32\\config\\sam",
    ".git/../../../etc/shadow",
    "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd",
    "....//....//....//etc/passwd",
    # Null byte injection
    "valid_ref\x00malicious",
    "refs/heads/main\x00../../etc/passwd",
    # Git ref injection
    "refs/heads/../../../etc/passwd",
    "refs/heads/HEAD\nref: refs/heads/malicious",
    # Unicode/encoding tricks
    "\u202e\u0041\u0042\u0043",  # Right-to-left override
    "refs/heads/\u0000evil",
    # Shell injection in ref names
    "refs/heads/$(cat /etc/passwd)",
    "refs/heads/`id`",
    "refs/heads/main; rm -rf /",
    # Credential/config injection
    "[core]\n\tgitProxy=malicious_command",
    "url = https://evil.com\n[credential]\n\thelper=malicious",
    # Extremely long inputs
    "A" * 10000,
    "refs/heads/" + "A" * 5000,
    # Special git references
    "HEAD",
    "FETCH_HEAD",
    "ORIG_HEAD",
    "MERGE_HEAD",
    "refs/heads/-malicious",
    # Symlink attacks
    ".git/config",
    ".git/hooks/pre-commit",
    # CRLF injection
    "refs/heads/main\r\nmalicious: header",
    "valid\r\n\r\nmalicious_body",
    # Empty and whitespace
    "",
    "   ",
    "\t\n\r",
    # Cargo.lock specific - package name injection
    'name = "malicious"\nversion = "0.0.1"\n[dependencies]\nevil = "*"',
    # Version string attacks
    "0.0.0-alpha.1; DROP TABLE packages;--",
    "999999999.999999999.999999999",
    # URL injection in Cargo.lock
    'https://evil.com/malicious#abc123',
    'file:///etc/passwd',
    'ssh://evil.com/repo.git',
]


def is_safe_ref_name(ref_name: str) -> bool:
    """
    Validate that a git reference name is safe.
    Implements git ref validation rules to prevent path traversal and injection.
    """
    if not ref_name or not ref_name.strip():
        return False
    
    # Must not contain null bytes
    if '\x00' in ref_name:
        return False
    
    # Must not contain control characters
    if any(ord(c) < 32 for c in ref_name):
        return False
    
    # Must not contain path traversal sequences
    if '..' in ref_name:
        return False
    
    # Must not start with a dash
    if ref_name.startswith('-'):
        return False
    
    # Must not contain shell metacharacters
    dangerous_chars = set('`$;|&><(){}[]\\')
    if any(c in dangerous_chars for c in ref_name):
        return False
    
    return True


def is_safe_package_name(name: str) -> bool:
    """Validate that a Cargo package name is safe (alphanumeric, hyphens, underscores)."""
    if not name:
        return False
    return bool(re.match(r'^[a-zA-Z0-9_-]+$', name))


def is_safe_version(version: str) -> bool:
    """Validate that a version string is a safe semver."""
    if not version:
        return False
    return bool(re.match(r'^\d+\.\d+\.\d+([.-][a-zA-Z0-9.+]+)?$', version))


def is_safe_url(url: str) -> bool:
    """Validate that a URL is safe (no file://, no injection)."""
    if not url:
        return True  # Empty URL is acceptable
    
    # Must not use file:// protocol
    if url.lower().startswith('file://'):
        return False
    
    # Must not contain null bytes or control characters
    if any(ord(c) < 32 for c in url):
        return False
    
    # Must not contain path traversal
    if '..' in url:
        return False
    
    return True


def sanitize_input(payload: str) -> str:
    """Sanitize input by removing dangerous characters."""
    # Remove null bytes
    sanitized = payload.replace('\x00', '')
    # Remove control characters
    sanitized = re.sub(r'[\x00-\x1f\x7f]', '', sanitized)
    # Remove path traversal
    sanitized = sanitized.replace('..', '')
    return sanitized


@pytest.mark.parametrize("payload", ADVERSARIAL_PAYLOADS)
def test_git_ref_security_boundary(payload):
    """Invariant: Adversarial inputs must never be treated as valid git references
    that could enable path traversal, injection, or unauthorized access."""
    
    # If the payload contains path traversal or injection patterns,
    # it must be rejected by the validator
    dangerous_patterns = [
        r'\.\.',           # Path traversal
        r'\x00',           # Null byte
        r'[\x00-\x1f]',   # Control characters
        r'[`$;|&><]',     # Shell metacharacters
    ]
    
    is_dangerous = any(
        re.search(pattern, payload) 
        for pattern in dangerous_patterns
    )
    
    if is_dangerous:
        # Dangerous payloads MUST be rejected
        assert not is_safe_ref_name(payload), (
            f"Security violation: Dangerous payload was accepted as valid ref: {repr(payload)}"
        )
    
    # Sanitized version must never contain dangerous sequences
    sanitized = sanitize_input(payload)
    assert '\x00' not in sanitized, "Null bytes must be removed during sanitization"
    assert not re.search(r'[\x00-\x1f]', sanitized), "Control chars must be removed"


@pytest.mark.parametrize("payload", ADVERSARIAL_PAYLOADS)
def test_cargo_lock_package_name_security(payload):
    """Invariant: Package names from Cargo.lock must always be validated
    to prevent injection attacks through dependency names."""
    
    # Package names with special characters must be rejected
    if re.search(r'[^a-zA-Z0-9_-]', payload) or not payload.strip():
        assert not is_safe_package_name(payload), (
            f"Security violation: Invalid package name accepted: {repr(payload)}"
        )


@pytest.mark.parametrize("payload", ADVERSARIAL_PAYLOADS)
def test_version_string_security(payload):
    """Invariant: Version strings must conform to semver and never contain
    injection payloads that could be executed or cause path traversal."""
    
    # Payloads with non-semver characters must be rejected
    if re.search(r'[^0-9a-zA-Z.\-+]', payload) or not payload.strip():
        assert not is_safe_version(payload), (
            f"Security violation: Malicious version string accepted: {repr(payload)}"
        )


@pytest.mark.parametrize("payload", ADVERSARIAL_PAYLOADS)
def test_url_security_boundary(payload):
    """Invariant: URLs in Cargo.lock must never use file:// protocol
    or contain path traversal sequences that could access local files."""
    
    if payload.lower().startswith('file://'):
        assert not is_safe_url(payload), (
            f"Security violation: file:// URL was accepted: {repr(payload)}"
        )
    
    if '..' in payload and ('/' in payload or '\\' in payload):
        # Path traversal in URLs must be rejected
        assert not is_safe_url(payload), (
            f"Security violation: Path traversal URL was accepted: {repr(payload)}"
        )


@pytest.mark.parametrize("payload", ADVERSARIAL_PAYLOADS)
def test_no_command_injection_in_git_operations(payload):
    """Invariant: Git-related inputs must never allow command injection
    regardless of the payload content."""
    
    # Shell metacharacters in any git-related input must be sanitized
    shell_metacharacters = set('`$;|&><(){}[]\\')
    
    sanitized = sanitize_input(payload)
    
    # After sanitization, dangerous shell chars should be handled
    # The key invariant: null bytes are always removed
    assert '\x00' not in sanitized, (
        f"Null byte injection not prevented for payload: {repr(payload)}"
    )
    
    # Control characters are always removed
    assert not any(ord(c) < 32 for c in sanitized), (
        f"Control character injection not prevented for payload: {repr(payload)}"
    )


@pytest.mark.parametrize("payload", [
    "../../../etc/passwd",
    "..\\..\\..\\windows\\system32",
    ".git/../../../etc/shadow",
    "refs/heads/../../../sensitive",
    "%2e%2e%2f%2e%2e%2f",
])
def test_path_traversal_always_blocked(payload):
    """Invariant: Path traversal sequences must ALWAYS be detected and blocked
    in git reference names and file paths."""
    
    # Path traversal must always be detected
    assert not is_safe_ref_name(payload), (
        f"CRITICAL: Path traversal not blocked: {repr(payload)}"
    )
    
    # Verify the traversal pattern is actually present
    assert '..' in payload or '%2e%2e' in payload.lower(), (
        f"Test payload doesn't contain traversal: {repr(payload)}"
    )


@pytest.mark.parametrize("payload", [
    "valid-package-name",
    "my_crate",
    "gitoxide",
    "serde-json",
    "tokio",
])
def test_legitimate_inputs_accepted(payload):
    """Invariant: Legitimate package names must always be accepted
    to ensure the security controls don't break normal functionality."""
    
    assert is_safe_package_name(payload), (
        f"Legitimate package name incorrectly rejected: {repr(payload)}"
    )


@pytest.mark.parametrize("version", [
    "0.21.1",
    "1.0.0",
    "2.3.4-alpha.1",
    "0.0.1",
    "10.20.30",
])
def test_legitimate_versions_accepted(version):
    """Invariant: Legitimate semver strings must always be accepted."""
    
    assert is_safe_version(version), (
        f"Legitimate version incorrectly rejected: {repr(version)}"
    )
use crate::shadow;
use crate::utils::{self, exec_cmd};
use nu_ansi_term::Style;

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub fn create() {
    println!("{}\n", shadow::VERSION.trim());
    let os_info = os_info::get();

    let environment = Environment {
        os_type: os_info.os_type(),
        os_version: os_info.version().clone(),
        shell_info: get_shell_info(),
        terminal_info: get_terminal_info(),
        starship_config: get_starship_config(),
    };

    let issue_body = get_github_issue_body(&environment);

    println!(
        "{}\n{issue_body}\n\n",
        Style::new().bold().paint("Generated bug report:")
    );
    println!("Forward the pre-filled report above to GitHub in your browser?");
    println!("{} To avoid any sensitive data from being exposed, please review the included information before proceeding. Data forwarded to GitHub is subject to GitHub's privacy policy.", Style::new().bold().paint("Warning:"));
    println!(
        "Enter `{}` to accept, or anything else to decline, and `{}` to confirm your choice:\n",
        Style::new().bold().paint("y"),
        Style::new().bold().paint("Enter key")
    );

    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    if input.trim().to_lowercase() == "y" {
        let link = make_github_issue_link(&issue_body);
        if let Err(e) = open::that(&link) {
            println!("Failed to open issue report in your browser: {e}");
            println!("Please copy the above report and open an issue manually, or try opening the following link:\n{link}");
        }
    } else {
        println!("Will not open an issue in your browser! Please copy the above report and open an issue manually.");
    }
    println!("Thanks for using the Starship bug report tool!");
}

const UNKNOWN_SHELL: &str = "<unknown shell>";
const UNKNOWN_TERMINAL: &str = "<unknown terminal>";
const UNKNOWN_VERSION: &str = "<unknown version>";
const UNKNOWN_CONFIG: &str = "<unknown config>";
const GITHUB_CHAR_LIMIT: usize = 8100; // Magic number accepted by Github

struct Environment {
    os_type: os_info::Type,
    os_version: os_info::Version,
    shell_info: ShellInfo,
    terminal_info: TerminalInfo,
    starship_config: String,
}

fn get_pkg_branch_tag() -> &'static str {
    if !shadow::TAG.is_empty() {
        return shadow::TAG;
    }
    shadow::BRANCH
}

fn get_github_issue_body(environment: &Environment) -> String {
    let shell_syntax = match environment.shell_info.name.as_ref() {
        "powershell" | "pwsh" => "pwsh",
        "fish" => "fish",
        "cmd" => "lua",
        // GitHub does not seem to support elvish syntax highlighting.
        "elvish" => "bash",
        _ => "bash",
    };

    format!("#### Current Behavior
<!-- A clear and concise description of the behavior. -->

#### Expected Behavior
<!-- A clear and concise description of what you expected to happen. -->

#### Additional context/Screenshots
<!-- Add any other context about the problem here. If applicable, add screenshots to help explain. -->

#### Possible Solution
<!--- Only if you have suggestions on a fix for the bug -->

#### Environment
- Starship version: {starship_version}
- {shell_name} version: {shell_version}
- Operating system: {os_name} {os_version}
- Terminal emulator: {terminal_name} {terminal_version}
- Git Commit Hash: {git_commit_hash}
- Branch/Tag: {pkg_branch_tag}
- Rust Version: {rust_version}
- Rust channel: {rust_channel} {build_rust_channel}
- Build Time: {build_time}
#### Relevant Shell Configuration

```{shell_syntax}
{shell_config}
```

#### Starship Configuration

```toml
{starship_config}
```",
        starship_version = shadow::PKG_VERSION,
        shell_name = environment.shell_info.name,
        shell_version = environment.shell_info.version,
        terminal_name = environment.terminal_info.name,
        terminal_version = environment.terminal_info.version,
        os_name = environment.os_type,
        os_version = environment.os_version,
        shell_config = environment.shell_info.config,
        starship_config = environment.starship_config,
        git_commit_hash =  shadow::SHORT_COMMIT,
        pkg_branch_tag =  get_pkg_branch_tag(),
        rust_version =  shadow::RUST_VERSION,
        rust_channel =  shadow::RUST_CHANNEL,
        build_rust_channel =  shadow::BUILD_RUST_CHANNEL,
        build_time =  shadow::BUILD_TIME,
        shell_syntax = shell_syntax,
    )
}

fn make_github_issue_link(body: &str) -> String {
    let escaped = urlencoding::encode(body).replace("%20", "+");

    format!(
        "https://github.com/starship/starship/issues/new?template={}&body={}",
        urlencoding::encode("Bug_report.md"),
        escaped
    )
    .chars()
    .take(GITHUB_CHAR_LIMIT)
    .collect()
}

#[derive(Debug)]
struct ShellInfo {
    name: String,
    version: String,
    config: String,
}

fn get_shell_info() -> ShellInfo {
    let shell = std::env::var("STARSHIP_SHELL");
    if shell.is_err() {
        return ShellInfo {
            name: UNKNOWN_SHELL.to_string(),
            version: UNKNOWN_VERSION.to_string(),
            config: UNKNOWN_CONFIG.to_string(),
        };
    }

    let shell = shell.unwrap();

    let version = get_shell_version(&shell);

    let config = get_config_path(&shell)
        .and_then(|config_path| fs::read_to_string(config_path).ok())
        .map_or_else(
            || UNKNOWN_CONFIG.to_string(),
            |config| config.trim().to_string(),
        );

    ShellInfo {
        name: shell,
        version,
        config,
    }
}

#[derive(Debug)]
struct TerminalInfo {
    name: String,
    version: String,
}

fn get_terminal_info() -> TerminalInfo {
    let terminal = std::env::var("TERM_PROGRAM")
        .or_else(|_| std::env::var("LC_TERMINAL"))
        .unwrap_or_else(|_| UNKNOWN_TERMINAL.to_string());

    let version = std::env::var("TERM_PROGRAM_VERSION")
        .or_else(|_| std::env::var("LC_TERMINAL_VERSION"))
        .unwrap_or_else(|_| UNKNOWN_VERSION.to_string());

    TerminalInfo {
        name: terminal,
        version,
    }
}

fn get_config_path(shell: &str) -> Option<PathBuf> {
    if shell == "nu" {
        return dirs_next::config_dir()
            .map(|config_dir| config_dir.join("nushell").join("config.nu"));
    }

    utils::home_dir().and_then(|home_dir| {
        match shell {
            "bash" => Some(".bashrc"),
            "fish" => Some(".config/fish/config.fish"),
            "ion" => Some(".config/ion/initrc"),
            "powershell" | "pwsh" => {
                if cfg!(windows) {
                    Some("Documents/PowerShell/Microsoft.PowerShell_profile.ps1")
                } else {
                    Some(".config/powershell/Microsoft.PowerShell_profile.ps1")
                }
            }
            "zsh" => Some(".zshrc"),
            "elvish" => Some(".elvish/rc.elv"),
            "tcsh" => Some(".tcshrc"),
            "xonsh" => Some(".xonshrc"),
            "cmd" => Some("AppData/Local/clink/starship.lua"),
            _ => None,
        }
        .map(|path| home_dir.join(path))
    })
}

fn get_starship_config() -> String {
    std::env::var("STARSHIP_CONFIG")
        .map(PathBuf::from)
        .ok()
        .or_else(|| {
            utils::home_dir().map(|mut home_dir| {
                home_dir.push(".config/starship.toml");
                home_dir
            })
        })
        .and_then(|config_path| fs::read_to_string(config_path).ok())
        .unwrap_or_else(|| UNKNOWN_CONFIG.to_string())
}

fn get_shell_version(shell: &str) -> String {
    let time_limit = Duration::from_millis(500);
    match shell {
        "powershell" => exec_cmd(
            shell,
            &["(Get-Host | Select Version | Format-Table -HideTableHeaders | Out-String).trim()"],
            time_limit,
        ),
        _ => exec_cmd(shell, &["--version"], time_limit),
    }
    .map_or_else(
        || UNKNOWN_VERSION.to_string(),
        |output| output.stdout.trim().to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_make_github_link() {
        let environment = Environment {
            os_type: os_info::Type::Linux,
            os_version: os_info::Version::Semantic(1, 2, 3),
            shell_info: ShellInfo {
                name: "test_shell".to_string(),
                version: "2.3.4".to_string(),
                config: "No config".to_string(),
            },
            terminal_info: TerminalInfo {
                name: "test_terminal".to_string(),
                version: "5.6.7".to_string(),
            },
            starship_config: "No Starship config".to_string(),
        };

        let body = get_github_issue_body(&environment);
        let link = make_github_issue_link(&body);

        assert!(link.contains(clap::crate_version!()));
        assert!(link.contains("Linux"));
        assert!(link.contains("1.2.3"));
        assert!(link.contains("test_shell"));
        assert!(link.contains("2.3.4"));
        assert!(link.contains("No+config"));
        assert!(link.contains("No+Starship+config"));
    }

    #[test]
    #[cfg(not(windows))]
    fn test_get_config_path() {
        let config_path = get_config_path("bash");
        assert_eq!(
            utils::home_dir().unwrap().join(".bashrc"),
            config_path.unwrap()
        );
    }
}

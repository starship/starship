# AI Usage Policy

This policy supplements our [Contributing Guide](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md).

This project is maintained by volunteers.
This policy exists to keep review burden reasonable.
The policy is inspired by the [Ghostty](https://github.com/ghostty-org/ghostty/blob/main/AI_POLICY.md) and [LLVM](https://llvm.org/docs/AIToolPolicy.html) AI policies.

Contributions that violate this policy may be closed without further notice.

## Mandatory Disclosure

Every Pull Request that utilizes AI-assisted tooling (including but not limited to Claude Code, Cursor, GitHub Copilot, ChatGPT, or local LLMs) must disclose its usage.

### PR Description

You must complete the **AI-Assistance** section in our Pull Request Template.

## Human-in-the-Loop

Contributors must fully understand all submitted contributions.

### Contributions

- You must be able to explain what your changes do and defend your implementation choices.
- You are expected to have read and understood every line of code you submit.
- If your response to a maintainer's question is an unedited copy-paste from an LLM, or if you cannot explain the mechanics of your PR, the PR will be closed.

### Issue Triage and Discussions

You are not allowed to reply to user issues or discussions with unverified or raw AI-generated information.

## "Good First Issue" Protections

You may not submit contributions to close a `🌱 good first issue` if they were authored with substantial AI assistance.
These issues are intentionally triaged as learning opportunities for new developers navigating the codebase for the first time.

## Low-Effort Contributions & Prohibition of Autonomous Agents

- Contributions that are overly verbose, contain unsupported or hallucinated claims, or otherwise show the hallmarks of low-effort LLM usage may be closed without further notice.
- Contributions via OpenClaw, or any other unsupervised autonomous agent operating in an automated loop, are strictly prohibited.

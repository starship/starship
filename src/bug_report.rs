use crate::utils;
use std::fs;
use std::path::PathBuf;

pub fn create() {
    let os_info = os_info::get();

    let environment = Environment {
        os_type: os_info.os_type(),
        os_version: os_info.version().to_owned(),
        shell_info: get_shell_info(),
        starship_config: get_starship_config(),
    };

    let link = make_github_issue_link(crate_version!(), environment);

    if open::that(&link)
        .ok()
        .and_then(|exit_status| {
            if exit_status.success() {
                Some(())
            } else {
                None
            }
        })
        .is_none()
    {
        println!(
            "I was unable to launch your browser. You'll have to copy this link instead:\n\n{}",
            link
        );
    }
}

const UNKNOWN_SHELL: &'static str = "<unknown shell>";
const UNKNOWN_VERSION: &'static str = "<unknown version>";
const UNKNOWN_CONFIG: &'static str = "<unknown config>";

struct Environment {
    os_type: os_info::Type,
    os_version: os_info::Version,
    shell_info: ShellInfo,
    starship_config: String,
}

fn make_github_issue_link(starship_version: &str, environment: Environment) -> String {
    let title = urlencoding::encode("Bug Report:");

    let body = urlencoding::encode(&format!("## Bug Report

#### Current Behavior
<!-- A clear and concise description of the behavior. -->

#### Expected Behavior
<!-- A clear and concise description of what you expected to happen. -->

#### Additional context/Screenshots
<!-- Add any other context about the problem here. If applicable, add screenshots to help explain. -->

#### Environment
- Starship version: {}
- {}: {}
- Operating system: {} {}

#### Relevant Shell Configuration

```bash
{}
```

#### Starship Configuration

```bash
{}
```

#### Possible Solution
<!--- Only if you have suggestions on a fix for the bug -->",
                                            starship_version,
                                            environment.shell_info.name,
                                            environment.shell_info.version,
                                            environment.os_type,
                                            environment.os_version,
                                            environment.shell_info.config,
                                            environment.starship_config,
    ));

    format!(
        "https://github.com/starship/starship/issues/new?title={}&body={}",
        title, body
    )
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

    let version = utils::exec_cmd(&shell, &["--version"])
        .map(|output| output.stdout.trim().to_string())
        .unwrap_or(UNKNOWN_VERSION.to_string());

    let config = get_config_path(&shell)
        .and_then(|config_path| fs::read_to_string(config_path).ok())
        .map(|config| config.trim().to_string())
        .unwrap_or(UNKNOWN_CONFIG.to_string());

    ShellInfo {
        name: shell,
        version,
        config,
    }
}

fn get_config_path(shell: &str) -> Option<PathBuf> {
    dirs::home_dir().and_then(|home_dir| {
        match shell {
            "bash" => Some(".bashrc"),
            "fish" => Some(".config/fish/config.fish"),
            "ion" => None, // ion doesn't provide a config file (yet)
            "powershell" => {
                if cfg!(windows) {
                    Some("Documents/PowerShell/Microsoft.PowerShell_profile.ps1")
                } else {
                    Some(".config/powershell/Microsoft.PowerShell_profile.ps1")
                }
            }
            "zsh" => Some(".zshrc"),
            _ => None,
        }
        .map(|path| home_dir.join(path))
    })
}

fn get_starship_config() -> String {
    dirs::home_dir()
        .and_then(|home_dir| fs::read_to_string(home_dir.join(".config/starship.toml")).ok())
        .unwrap_or(UNKNOWN_CONFIG.to_string())
}

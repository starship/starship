use std::path::{PathBuf, Path};
use std::fs;
use std::process::Command;

pub fn create() {
    let os_info = os_info::get();

    let environment = Environment {
        os_type: os_info.os_type(),
        os_version: os_info.version().to_owned(),
        shell_info: get_shell_info(),
        starship_config: get_starship_config(),
    };

    make_github_issue_link(crate_version!(), environment)
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

fn make_github_issue_link(
    starship_version: &str,
    environment: Environment) {
    let github_issue_url = "https://github.com/starship/starship/issues/new?title=Bug Report:&body=";

    println!("{}## Bug Report

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
             github_issue_url,
             starship_version,
             environment.shell_info.name,
             environment.shell_info.version,
             environment.os_type,
             environment.os_version,
             environment.shell_info.config,
             environment.starship_config,
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

    let version = Command::new(&shell)
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.trim().to_string())
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
    match shell {
        "fish" => Some(expand_tilde(Path::new("~/.config/fish/config.fish").to_path_buf())),
        "bash" => Some(expand_tilde(Path::new("~/.bashrc").to_path_buf())),
        "zsh" => Some(expand_tilde(Path::new("~/.zshrc").to_path_buf())),
        _ => None,
    }
}

fn get_starship_config() -> String {
    fs::read_to_string(expand_tilde(Path::new("~/.config/starship.toml").to_path_buf()))
        .unwrap_or(UNKNOWN_CONFIG.to_string())
}

/// Convert a `~` in a path to the home directory
fn expand_tilde(dir: PathBuf) -> PathBuf {
    if dir.starts_with("~") {
        let without_home = dir.strip_prefix("~").unwrap();
        return dirs::home_dir().unwrap().join(without_home);
    }
    dir
}

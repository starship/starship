use os_info;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub fn bug_report() {
	let os_info = os_info::get();

	let environment = Environment::new(
		os_info.os_type(), 
		os_info.version().to_owned(),
		get_shells_infos(), 
		"starship_config: String".to_string());

	make_github_issue_link(crate_version!(), environment)
}

/// Convert a `~` in a path to the home directory
fn expand_tilde(dir: PathBuf) -> PathBuf {
	if dir.starts_with("~") {
		let without_home = dir.strip_prefix("~").unwrap();
		return dirs::home_dir().unwrap().join(without_home);
	}
	dir
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
- fish: {:#?}
- bash: {:#?}
- ZSH: {:#?}
- Operating system: {} {}

#### Relevant Shell Configuration

**fish**
```bash
{}
```

**bash**
```bash
{}
```

**zsh**
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
	environment.shell_types[0].shell_version, 
	environment.shell_types[1].shell_version,
	environment.shell_types[2].shell_version, 
	environment.os_type, 
	environment.os_version,
	environment.shell_types[0].shell_config, 
	environment.shell_types[1].shell_config,
	environment.shell_types[2].shell_config, 
	"starship config".to_string(),
	)
}

pub struct Environment {
	pub os_type: os_info::Type,
	pub os_version: os_info::Version,

	pub shell_types: Vec<ShellInfo>,

	pub starship_config: String,
}

impl Environment {
	pub fn new(
		os_type: os_info::Type, 
		os_version: os_info::Version,
		shell_types: Vec<ShellInfo>,
		starship_config: String
	) -> Environment {
		Environment {
			os_type: os_type,
			os_version: os_version,
			shell_types: shell_types,
			starship_config: starship_config,
		}
	}
}

#[derive(Debug)]
pub struct ShellInfo {
	shell_type: ShellType,
	shell_config: String,
	shell_version: String,
}

pub fn get_shells_infos() -> Vec<ShellInfo> {
	let mut shell_types: Vec<ShellInfo> = Vec::new();
	let fish_config_path = expand_tilde(Path::new("~/.config/fish/config.fish").to_path_buf());
	let bash_config_path = expand_tilde(Path::new("~/.bashrc").to_path_buf());
	let zsh_config_path = expand_tilde(Path::new("~/.zshrc").to_path_buf());

	if is_shell(&fish_config_path) {
		fs::read_to_string(&fish_config_path).and_then(|content| {
			let shell_version = match Command::new("fish")
			.arg("--version")
			.output() {
				Ok(output) => {
					let stdout_string = String::from_utf8(output.stdout).unwrap();

					stdout_string	
				}
				Err(_) => "Error could not find fish version".to_string(),
			};

			shell_types.push(ShellInfo {
				shell_type: ShellType::Fish, 
				shell_config: content,
				shell_version,
			});

			Ok("Pushed fish config")
		});
	}

	if is_shell(&bash_config_path) {
		let shell_version = match Command::new("bash")
		.arg("--version")
		.output() {
			Ok(output) => {
				let stdout_string = String::from_utf8(output.stdout).unwrap();

				stdout_string	
			}
			Err(_) => "Error could not find bash version".to_string(),
		};

		fs::read_to_string(&bash_config_path).and_then(|content| {
			shell_types.push(ShellInfo {
				shell_type: ShellType::Bash, 
				shell_config: content,
				shell_version,
			});

			Ok("Pushed bash config")
		});
	}

	if is_shell(&zsh_config_path) {
		let shell_version = match Command::new("zsh")
		.arg("--version")
		.output() {
			Ok(output) => {
				let stdout_string = String::from_utf8(output.stdout).unwrap();

				stdout_string	
			}
			Err(_) => "Error could not find fish version".to_string(),
		};

		fs::read_to_string(&zsh_config_path).and_then(|content| {
			shell_types.push(ShellInfo {
				shell_type: ShellType::Zsh, 
				shell_config: content,
				shell_version,
			});

			Ok("Pushed zsh config")
		});
	}

	shell_types
}

pub fn is_shell(path: &PathBuf) -> bool {
	match fs::metadata(&path) {
		Ok(metadata) => metadata.is_file(),
		Err(e) => false
	}
}

#[derive(Debug)]
pub enum ShellType {
	Bash,
	Fish,
	Zsh,
}

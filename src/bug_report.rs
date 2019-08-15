use os_info;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn bug_report() {
	let os_info = os_info::get();

	println!("os_info type: {}", os_info.os_type());
	println!("os_info version: {}", os_info.version());

	// Shell
	// Array of different config files and their locations
	// Each of those we give to a path constructor so we have a path object
	let configArr: [PathBuf; 3] = [
		expand_tilde(Path::new("~/.bashrc").to_path_buf()),
		expand_tilde(Path::new("~/.zshrc").to_path_buf()),
		expand_tilde(Path::new("~/.config/fish/config.fish").to_path_buf()),
	];

	// if let Ok(metadata) = fs::metadata() {
	// 	println!("is file: {}", metadata.is_file());
	// }

	fs::metadata(&configArr[0]).and_then(|metadata| {
		println!("is file: {}", metadata.is_file());

		Ok("Uses bash")
	});

	fs::metadata(&configArr[1]).and_then(|metadata| {
		println!("is file: {}", metadata.is_file());

		Ok("Uses zsh")
	});

	fs::metadata(&configArr[2]).and_then(|metadata| {
		println!("is file: {}", metadata.is_file());

		Ok("Uses fish")
	});

	let metadata = fs::metadata(&configArr[2]);
	println!("{:?}", configArr[2]);

	// Each path object will check that the file exists (fs meta data)
	// Read contents (fs read file) look for existing starship util
	// look for string starship init
}

/// Convert a `~` in a path to the home directory
fn expand_tilde(dir: PathBuf) -> PathBuf {
		if dir.starts_with("~") {
				let without_home = dir.strip_prefix("~").unwrap();
				return dirs::home_dir().unwrap().join(without_home);
		}
		dir
}

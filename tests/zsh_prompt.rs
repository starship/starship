use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Cargo supplies an absolute binary path; the command helper is private"
)]
fn zsh_prompt_preserves_content_and_newline_setting() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let config_path = dir.path().join("starship.toml");
    for (add_newline, prefix) in [(true, "\n"), (false, "")] {
        let mut config = File::create(&config_path)?;
        write!(
            config,
            "add_newline = {add_newline}\nformat = \"quote's; printf unsafe\\n>\"\n"
        )?;
        config.sync_all()?;
        drop(config);

        for zsh in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_starship"));
            command
                .current_dir(dir.path())
                .env("STARSHIP_CONFIG", &config_path)
                .env("STARSHIP_CACHE", dir.path())
                .env("STARSHIP_SHELL", "zsh")
                .env("TERM", "xterm-256color")
                .arg("prompt");
            if zsh {
                command.arg("--zsh");
            }
            let output = command.output()?;
            assert!(output.status.success(), "{:?}", output.stderr);
            let metadata = if zsh {
                u8::from(add_newline).to_string()
            } else {
                String::new()
            };
            assert_eq!(
                output.stdout,
                format!("{metadata}{prefix}quote's; printf unsafe\n>").as_bytes()
            );
        }
    }

    for args in [
        vec!["--right"],
        vec!["--continuation"],
        vec!["--profile", "test"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_starship"))
            .args(["prompt", "--zsh"])
            .args(args)
            .output()?;
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
    }

    dir.close()
}

use super::{Context, Module, ModuleConfig};

use crate::configs::nix_shell::NixShellConfig;
use crate::formatter::StringFormatter;

enum NixShellType {
    Pure,
    Impure,
    /// We're in a Nix shell, but we don't know which type.
    /// This can only happen in a `nix shell` shell (not a `nix-shell` one).
    Unknown,
}

impl NixShellType {
    fn detect_shell_type(use_heuristic: bool, context: &Context) -> Option<Self> {
        use NixShellType::{Impure, Pure, Unknown};

        let shell_type = context.get_env("IN_NIX_SHELL");
        match shell_type.as_deref() {
            Some("pure") => return Some(Pure),
            Some("impure") => return Some(Impure),
            _ => {}
        };

        if use_heuristic {
            Self::in_new_nix_shell(context).map(|()| Unknown)
        } else {
            None
        }
    }

    // Hack to detect if we're in a `nix shell` (in contrast to a `nix-shell`).
    // A better way to do this will be enabled by https://github.com/NixOS/nix/issues/6677.
    fn in_new_nix_shell(context: &Context) -> Option<()> {
        let path = context.get_env("PATH")?;

        std::env::split_paths(&path)
            .any(|path| path.starts_with("/nix/store"))
            .then_some(())
    }
}

/// Creates a module showing if inside a nix-shell
///
/// The module will use the `$IN_NIX_SHELL` and `$name` environment variable to
/// determine if it's inside a nix-shell and the name of it.
///
/// The following options are available:
///     - `impure_msg` (string)  // change the impure msg
///     - `pure_msg` (string)    // change the pure msg
///     - `unknown_msg` (string) // change the unknown message
///
/// Will display the following:
///     - pure (name)    // $name == "name" in a pure nix-shell
///     - impure (name)  // $name == "name" in an impure nix-shell
///     - pure           // $name == "" in a pure nix-shell
///     - impure         // $name == "" in an impure nix-shell
///     - unknown (name) // $name == "name" in an unknown nix-shell
///     - unknown        // $name == "" in an unknown nix-shell
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("nix_shell");
    let config: NixShellConfig = NixShellConfig::try_load(module.config);

    let shell_name = context.get_env("name");
    let shell_type = NixShellType::detect_shell_type(config.heuristic, context)?;
    let shell_type_format = match shell_type {
        NixShellType::Pure => config.pure_msg,
        NixShellType::Impure => config.impure_msg,
        NixShellType::Unknown => config.unknown_msg,
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                "state" => Some(shell_type_format),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "name" => shell_name.as_ref().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `nix_shell`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn no_env_variables() {
        let actual = ModuleRenderer::new("nix_shell").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_env_variables() {
        let actual = ModuleRenderer::new("nix_shell")
            .env("IN_NIX_SHELL", "something_wrong")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn pure_shell() {
        let actual = ModuleRenderer::new("nix_shell")
            .env("IN_NIX_SHELL", "pure")
            .collect();
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("❄️  pure")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn impure_shell() {
        let actual = ModuleRenderer::new("nix_shell")
            .env("IN_NIX_SHELL", "impure")
            .collect();
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("❄️  impure")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn pure_shell_name() {
        let actual = ModuleRenderer::new("nix_shell")
            .env("IN_NIX_SHELL", "pure")
            .env("name", "starship")
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Blue.bold().paint("❄️  pure (starship)")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn impure_shell_name() {
        let actual = ModuleRenderer::new("nix_shell")
            .env("IN_NIX_SHELL", "impure")
            .env("name", "starship")
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Blue.bold().paint("❄️  impure (starship)")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn new_nix_shell() {
        let actual = ModuleRenderer::new("nix_shell")
            .env(
                "PATH",
                "/nix/store/v7qvqv81jp0cajvrxr9x072jgqc01yhi-nix-info/bin:/Users/user/.cargo/bin",
            )
            .config(toml::toml! {
                [nix_shell]
                heuristic = true
            })
            .collect();
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("❄️  ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_new_nix_shell() {
        let actual = ModuleRenderer::new("nix_shell")
            .env("PATH", "/Users/user/.cargo/bin")
            .config(toml::toml! {
                [nix_shell]
                heuristic = true
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_new_nix_shell_with_nix_store_subdirectory() {
        let actual = ModuleRenderer::new("nix_shell")
            .env("PATH", "/Users/user/some/nix/store/subdirectory")
            .config(toml::toml! {
                [nix_shell]
                heuristic = true
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_new_nix_shell_when_heuristic_is_disabled() {
        let actual = ModuleRenderer::new("nix_shell")
            .env(
                "PATH",
                "/nix/store/v7qvqv81jp0cajvrxr9x072jgqc01yhi-nix-info/bin:/Users/user/.cargo/bin",
            )
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }
}

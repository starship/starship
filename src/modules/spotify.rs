use super::{Context, Module, RootModuleConfig};

use crate::configs::spotify::SpotifyConfig;
use crate::formatter::StringFormatter;
use std::env;

pub fn eval_apple_script(script: &str, context: &Context<'_>) -> Option<String> {
    let cmd = context.exec_cmd("osascript", &["-e", &*format!("\"{}\"", script)])?;
    Some(cmd.stdout)
}

pub fn is_running_on_macos(context: &Context<'_>) -> bool {
    eval_apple_script("application \"Spotify\" is running", context).unwrap() == "true"
}

/// Creates a module which will display the song currently playing on Spotify
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("spotify");
    let config = SpotifyConfig::try_load(module.config);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "artist" => {
                    let artist = if env::consts::OS == "macos" {
                        if is_running_on_macos(context) {
                            let artist_pre = eval_apple_script(
                                "tell application \"Spotify\" to artist of current track as string",
                                context,
                            );
                            if let Some(v) = artist_pre {
                                Some(v)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        Some("yes".to_owned())
                    };

                    artist.map(Ok)
                }
                _ => None,
            })
            .map_style(|variable| match variable {
                "song" => Some(Ok(config.style)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `deno`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    #[cfg(windows)]
    fn spotify_windows_running() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mod_ts() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mod.ts"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mod_js() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mod.js"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_deps_ts() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("deps.ts"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_deps_js() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("deps.js"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}

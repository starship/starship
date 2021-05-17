use super::{Context, Module, RootModuleConfig};

use crate::configs::spotify::SpotifyConfig;
use crate::formatter::StringFormatter;
use regex::Regex;
use std::env;
use window_titles::{Connection, ConnectionTrait};

static OS: &str = env::consts::OS;

pub fn eval_apple_script(script: &str, context: &Context<'_>) -> Option<String> {
    let cmd = context.exec_cmd("osascript", &["-e", &*format!("\"{}\"", script)])?;
    Some(cmd.stdout)
}

pub fn is_spotify_running_on_macos(context: &Context<'_>) -> bool {
    eval_apple_script("application \"Spotify\" is running", context).unwrap() == "true"
}

pub fn sanitize_vec(vec: Vec<String>) -> Vec<String> {
    vec.into_iter()
        .filter(|string| !string.trim().is_empty())
        .collect::<Vec<String>>()
}

pub fn is_spotify_running_on_windows() -> bool {
    if sanitize_vec(
        Connection::new()
            .unwrap()
            .windows()
            .into_iter()
            .filter(|window| window.process().name() == "Spotify.exe")
            .map(|window| window.name().unwrap())
            .collect::<Vec<String>>(),
    )
    .len()
        == 0
    {
        false
    } else {
        true
    }
}

pub fn artist(context: &Context<'_>) -> Option<String> {
    if OS == "macos" && is_spotify_running_on_macos(context) {
        eval_apple_script(
            "tell application \"Spotify\" to artist of current track as string",
            context,
        )
    } else {
        // TODO: add windows & linux support
        None
    }
}

pub fn song(context: &Context<'_>) -> Option<String> {
    if OS == "macos" && is_spotify_running_on_macos(context) {
        eval_apple_script(
            "tell application \"Spotify\" to name of current track as string",
            context,
        )
    } else if OS == "windows" && is_spotify_running_on_windows() {
        let re = Regex::new(r"(Spotify.*)|(.*\\s-\\s.*)").unwrap();
        let raw_song = sanitize_vec(
            Connection::new()
                .unwrap()
                .windows()
                .into_iter()
                .filter(|window| window.process().name() == "Spotify.exe")
                .map(|window| window.name().unwrap())
                .collect::<Vec<String>>(),
        )
        .first()
        .unwrap()
        .to_owned();
        let caps = re.captures(&raw_song).unwrap();
        Some("hi".to_string())
    } else {
        // todo: add linux support
        None
    }
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
            .map_meta(|variable, _| match variable {
                "artist" => {
                    let value = artist(context).as_deref().unwrap_or("na").clone();
                    Some(value)
                },
                _ => None,
            })
            .map_meta(|variable, _| match variable {
                "song" => {
                    let value = song(context).as_deref().unwrap_or("na").clone();
                    Some(value)
                },
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `spotify`:\n{}", error);
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

use crate::configs::Palette;
use crate::context::Context;

use crate::utils;
use crate::utils::serde::{ValueDeserializer, ValueRef};
use nu_ansi_term::Color;
use serde::{
    Deserialize, Deserializer, Serialize, de::Error as SerdeError, de::value::Error as ValueError,
};

use std::borrow::Cow;
use std::clone::Clone;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use toml::Value;

/// Root config of a module.
pub trait ModuleConfig<'a, E>
where
    Self: Default,
    E: SerdeError,
{
    /// Construct a `ModuleConfig` from a toml value.
    fn from_config<V: Into<ValueRef<'a>>>(config: V) -> Result<Self, E>;

    /// Loads the TOML value into the config.
    /// Missing values are set to their default values.
    /// On error, logs an error message.
    fn load<V: Into<ValueRef<'a>>>(config: V) -> Self {
        match Self::from_config(config) {
            Ok(config) => config,
            Err(e) => {
                log::warn!("Failed to load config value: {e}");
                Self::default()
            }
        }
    }

    /// Helper function that will call `ModuleConfig::from_config(config)` if config is Some,
    /// or `ModuleConfig::default()` if config is None.
    fn try_load<V: Into<ValueRef<'a>>>(config: Option<V>) -> Self {
        config.map(Into::into).map(Self::load).unwrap_or_default()
    }
}

impl<'a, T: Deserialize<'a> + Default> ModuleConfig<'a, ValueError> for T {
    /// Create `ValueDeserializer` wrapper and use it to call `Deserialize::deserialize` on it.
    fn from_config<V: Into<ValueRef<'a>>>(config: V) -> Result<Self, ValueError> {
        let config = config.into();
        let deserializer = ValueDeserializer::new(config);
        T::deserialize(deserializer).or_else(|err| {
            // If the error is an unrecognized key, print a warning and run
            // deserialize ignoring that error. Otherwise, just return the error
            if err.to_string().contains("Unknown key") {
                log::warn!("{err}");
                let deserializer2 = ValueDeserializer::new(config).with_allow_unknown_keys();
                T::deserialize(deserializer2)
            } else {
                Err(err)
            }
        })
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(untagged)]
pub enum Either<A, B> {
    First(A),
    Second(B),
}

/// A wrapper around `Vec<T>` that implements `ModuleConfig`, and either
/// accepts a value of type `T` or a list of values of type `T`.
#[derive(Clone, Default, Serialize)]
pub struct VecOr<T>(pub Vec<T>);

impl<'de, T> Deserialize<'de> for VecOr<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let either = Either::<Vec<T>, T>::deserialize(deserializer)?;
        match either {
            Either::First(v) => Ok(Self(v)),
            Either::Second(s) => Ok(Self(vec![s])),
        }
    }
}

#[cfg(feature = "config-schema")]
impl<T> schemars::JsonSchema for VecOr<T>
where
    T: schemars::JsonSchema + Sized,
{
    fn schema_name() -> Cow<'static, str> {
        // `Either::<T, Vec<T>>::schema_name()` is not unique per `T`; nested `VecOr`s must not share a `$defs` entry.
        Cow::Owned(format!("VecOr_{}", T::schema_name()))
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Owned(format!("{}::VecOr<{}>", module_path!(), T::schema_id()))
    }

    fn json_schema(generator: &mut schemars::generate::SchemaGenerator) -> schemars::Schema {
        Either::<T, Vec<T>>::json_schema(generator)
    }
}

/// Root config of starship.
#[derive(Default)]
pub struct StarshipConfig {
    pub config: Option<toml::Table>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Resolved locations that may contribute to the effective Starship configuration.
///
/// The values are derived from Starship-specific environment variables and the
/// XDG Base Directory Specification at https://specifications.freedesktop.org/basedir/latest/.
/// For example, `XDG_CONFIG_HOME=/home/alice/.config` resolves the user config
/// file to `/home/alice/.config/starship/starship.toml`.
pub struct ConfigSources {
    /// Explicit config file from `STARSHIP_CONFIG`, for example `/tmp/starship.toml`.
    pub explicit_config: Option<PathBuf>,
    /// User config directory from `STARSHIP_CONFIG_HOME` or the XDG/platform default.
    pub user_config_home: Option<PathBuf>,
    /// Main user config file, for example `$STARSHIP_CONFIG_HOME/starship.toml`.
    pub user_config_file: Option<PathBuf>,
    /// User drop-in config directory, for example `$STARSHIP_CONFIG_HOME/conf.d`.
    pub user_conf_d: Option<PathBuf>,
    /// System drop-in config directories, for example `/etc/xdg/starship/conf.d`.
    pub system_conf_dirs: Vec<PathBuf>,
    /// Legacy config file used only for migration detection, for example `~/.config/starship.toml`.
    pub legacy_config_file: Option<PathBuf>,
}

impl ConfigSources {
    /// Resolve config sources from the provided environment.
    ///
    /// `STARSHIP_CONFIG` points to one explicit config file and disables automatic
    /// `conf.d` loading. Without it, `STARSHIP_CONFIG_HOME` controls the user
    /// config directory, falling back to the XDG/platform config directory.
    pub fn from_env(env: &crate::utils::env::Env<'_>) -> Self {
        let explicit_config = env.get_env_os("STARSHIP_CONFIG").map(PathBuf::from);
        let home_dir = home_dir(env);
        let user_config_home = env
            .get_env_os("STARSHIP_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| default_user_config_home(env));
        let user_config_file = user_config_home
            .as_ref()
            .map(|config_home| config_home.join("starship.toml"));
        let user_conf_d = user_config_home
            .as_ref()
            .map(|config_home| config_home.join("conf.d"));
        let legacy_config_file = home_dir.map(|home| home.join(".config").join("starship.toml"));

        Self {
            explicit_config,
            user_config_home,
            user_config_file,
            user_conf_d,
            system_conf_dirs: system_conf_dirs(env),
            legacy_config_file,
        }
    }

    /// Return the single file edited by `starship config` commands.
    ///
    /// This is `STARSHIP_CONFIG` when set, otherwise the main user config file,
    /// for example `$STARSHIP_CONFIG_HOME/starship.toml`.
    pub fn primary_edit_path(&self) -> Option<PathBuf> {
        self.explicit_config
            .clone()
            .or_else(|| self.user_config_file.clone())
    }

    /// Return concrete TOML files in the order they should be merged.
    ///
    /// With no `STARSHIP_CONFIG`, system `conf.d` files are loaded first, then
    /// `starship.toml`, then user `conf.d` files such as `00-base.toml` before
    /// `90-local.toml`.
    fn ordered_config_files(&self) -> Vec<PathBuf> {
        if let Some(config) = &self.explicit_config {
            return vec![config.clone()];
        }

        handle_legacy_config(self);

        let mut config_files = Vec::new();
        for dir in &self.system_conf_dirs {
            config_files.extend(sorted_toml_files(dir));
        }
        if let Some(config_file) = &self.user_config_file {
            config_files.push(config_file.clone());
        }
        if let Some(conf_d) = &self.user_conf_d {
            config_files.extend(sorted_toml_files(conf_d));
        }
        config_files
    }
}

impl StarshipConfig {
    /// Initialize the Config struct
    pub fn initialize(config_file_path: Option<&OsStr>) -> Self {
        Self::config_from_file(config_file_path)
            .map(|config| Self {
                config: Some(config),
            })
            .unwrap_or_default()
    }

    pub fn initialize_from_sources(config_sources: &ConfigSources) -> Self {
        Self::config_from_sources(config_sources)
            .map(|config| Self {
                config: Some(config),
            })
            .unwrap_or_default()
    }

    /// Build a config table by reading and merging all files from `ConfigSources`.
    ///
    /// Later files override earlier files. For example, `conf.d/10-work.toml`
    /// can override values from `starship.toml` while preserving unrelated keys.
    fn config_from_sources(config_sources: &ConfigSources) -> Option<toml::Table> {
        let mut config = toml::Value::Table(toml::Table::new());
        let mut loaded = false;

        for config_file_path in config_sources.ordered_config_files() {
            let Some(toml_content) =
                Self::read_config_content_as_str(Some(config_file_path.as_os_str()))
            else {
                continue;
            };

            match toml::from_str::<toml::Value>(&toml_content) {
                Ok(parsed) => {
                    merge_toml(&mut config, parsed);
                    loaded = true;
                }
                Err(error) => {
                    log::error!(
                        "Unable to parse config file {}: {error}",
                        config_file_path.display()
                    );
                }
            }
        }

        loaded.then(|| config.as_table().cloned().unwrap_or_default())
    }

    /// Create a config from a starship configuration file
    fn config_from_file(config_file_path: Option<&OsStr>) -> Option<toml::Table> {
        let toml_content = Self::read_config_content_as_str(config_file_path)?;

        match toml::from_str(&toml_content) {
            Ok(parsed) => {
                log::debug!("Config parsed: {parsed:?}");
                Some(parsed)
            }
            Err(error) => {
                log::error!("Unable to parse the config file: {error}");
                None
            }
        }
    }

    pub fn read_config_content_as_str(config_file_path: Option<&OsStr>) -> Option<String> {
        if config_file_path.is_none() {
            log::debug!(
                "Unable to determine `config_file_path`. Perhaps `utils::home_dir` is not defined on your platform?"
            );
            return None;
        }
        let config_file_path = config_file_path.as_ref().unwrap();
        match utils::read_file(config_file_path) {
            Ok(content) => {
                log::trace!("Config file content: \"\n{content}\"");
                Some(content)
            }
            Err(e) => {
                let level = if e.kind() == ErrorKind::NotFound {
                    log::Level::Debug
                } else {
                    log::Level::Error
                };

                log::log!(level, "Unable to read config file content: {e}");
                None
            }
        }
    }

    /// Get the subset of the table for a module by its name
    pub fn get_module_config(&self, module_name: &str) -> Option<&Value> {
        let module_config = self.get_config(&[module_name]);
        if module_config.is_some() {
            log::debug!("Config found for \"{module_name}\": {module_config:?}");
        }
        module_config
    }

    /// Get the value of the config in a specific path
    pub fn get_config(&self, path: &[&str]) -> Option<&Value> {
        let mut prev_table = self.config.as_ref()?;

        assert_ne!(
            path.len(),
            0,
            "Starship::get_config called with an empty path"
        );

        let (table_options, _) = path.split_at(path.len() - 1);

        // Assumes all keys except the last in path has a table
        for option in table_options {
            if let Some(value) = prev_table.get(*option) {
                if let Some(value) = value.as_table() {
                    prev_table = value;
                } else {
                    log::trace!(
                        "No config found for \"{}\": \"{}\" is not a table",
                        path.join("."),
                        option
                    );
                    return None;
                }
            } else if prev_table.contains_key(*option) {
                log::trace!(
                    "No config found for \"{}\": \"{}\" is not a table",
                    path.join("."),
                    option
                );
                return None;
            }
        }

        let last_option = path.last().unwrap();
        let value = prev_table.get(*last_option);
        if value.is_none() {
            log::trace!(
                "No config found for \"{}\": Option \"{}\" not found",
                path.join("."),
                last_option
            );
        }
        value
    }

    /// Get the subset of the table for a custom module by its name
    pub fn get_custom_module_config(&self, module_name: &str) -> Option<&Value> {
        let module_config = self.get_config(&["custom", module_name]);
        if module_config.is_some() {
            log::debug!("Custom config found for \"{module_name}\": {module_config:?}");
        }
        module_config
    }

    /// Get the table of all the registered custom modules, if any
    pub fn get_custom_modules(&self) -> Option<&toml::value::Table> {
        self.get_config(&["custom"])?.as_table()
    }
    /// Get the table of all the registered `env_var` modules, if any
    pub fn get_env_var_modules(&self) -> Option<&toml::value::Table> {
        self.get_config(&["env_var"])?.as_table()
    }
}

/// Return the current user's home directory, using the test environment when available.
///
/// In tests, `HOME=/tmp/home` resolves to `/tmp/home`; in normal builds this
/// delegates to the platform home directory lookup.
fn home_dir(env: &crate::utils::env::Env<'_>) -> Option<PathBuf> {
    if cfg!(test)
        && let Some(home) = env.get_env("HOME")
    {
        return Some(PathBuf::from(home));
    }
    utils::home_dir()
}

/// Resolve the default user config directory.
///
/// This follows the XDG Base Directory Specification at
/// https://specifications.freedesktop.org/basedir/latest/ when `XDG_CONFIG_HOME`
/// is set. For example, `XDG_CONFIG_HOME=/home/alice/.config` resolves to
/// `/home/alice/.config/starship`. Platforms without XDG use their standard
/// config directory with `starship` appended.
fn default_user_config_home(env: &crate::utils::env::Env<'_>) -> Option<PathBuf> {
    env.get_env_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .map(|path| path.join("starship"))
        .or_else(|| dirs::config_dir().map(|path| path.join("starship")))
}

/// Resolve system-level drop-in directories.
///
/// On XDG systems, each `XDG_CONFIG_DIRS` entry contributes
/// `starship/conf.d`; for example `/etc/xdg` becomes
/// `/etc/xdg/starship/conf.d`. If XDG is not configured, Unix defaults to
/// `/etc/xdg/starship/conf.d`, macOS uses `/Library/Application Support/starship/conf.d`,
/// and Windows uses `%PROGRAMDATA%\starship\conf.d` when available.
fn system_conf_dirs(env: &crate::utils::env::Env<'_>) -> Vec<PathBuf> {
    if let Some(config_dirs) = env.get_env("XDG_CONFIG_DIRS") {
        let dirs: Vec<_> = config_dirs
            .split(':')
            .filter(|dir| !dir.is_empty())
            .map(|dir| PathBuf::from(dir).join("starship").join("conf.d"))
            .collect();
        if !dirs.is_empty() {
            return dirs;
        }
    }

    if cfg!(windows) {
        env.get_env_os("PROGRAMDATA")
            .map(PathBuf::from)
            .map(|path| path.join("starship").join("conf.d"))
            .into_iter()
            .collect()
    } else if cfg!(target_os = "macos") {
        vec![PathBuf::from(
            "/Library/Application Support/starship/conf.d",
        )]
    } else {
        vec![PathBuf::from("/etc/xdg/starship/conf.d")]
    }
}

/// Return `.toml` files from a directory in ascending path order.
///
/// Non-TOML files are ignored, so `README.md` is skipped while `00-base.toml`
/// is loaded before `10-local.toml`.
fn sorted_toml_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut files: Vec<_> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "toml")
        })
        .filter(|path| path.is_file())
        .collect();
    files.sort();
    files
}

/// Copy the legacy config to the new location when migration is safe.
///
/// This only applies when `STARSHIP_CONFIG` is not set. The legacy file, for
/// example `~/.config/starship.toml`, is never removed and is not used after a
/// failed migration.
fn handle_legacy_config(config_sources: &ConfigSources) {
    let Some(legacy_config_file) = &config_sources.legacy_config_file else {
        return;
    };
    if !legacy_config_file.exists() {
        return;
    }
    let Some(user_config_file) = &config_sources.user_config_file else {
        log::warn!(
            "Ignoring legacy config at {} because the new config location could not be determined",
            legacy_config_file.display()
        );
        return;
    };
    if user_config_file.exists() {
        log::warn!(
            "Ignoring legacy config at {} because {} is used. Remove the legacy file to silence this warning.",
            legacy_config_file.display(),
            user_config_file.display()
        );
        return;
    }

    if let Err(error) = copy_and_validate_legacy_config(legacy_config_file, user_config_file) {
        log::warn!(
            "Failed to migrate legacy config from {} to {}: {error}. The legacy file is not used.",
            legacy_config_file.display(),
            user_config_file.display()
        );
    } else {
        log::warn!(
            "Copied legacy config from {} to {}. The legacy file is no longer used and can be removed.",
            legacy_config_file.display(),
            user_config_file.display()
        );
    }
}

/// Copy a legacy config file and verify the copied TOML parses to the same value.
///
/// Validation keeps migration non-destructive: if the copied file cannot be read
/// or does not parse to the same TOML value, callers can warn and leave the
/// original file untouched.
fn copy_and_validate_legacy_config(
    legacy_config_file: &Path,
    user_config_file: &Path,
) -> Result<(), String> {
    if let Some(parent) = user_config_file.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::copy(legacy_config_file, user_config_file).map_err(|error| error.to_string())?;

    let legacy_config =
        fs::read_to_string(legacy_config_file).map_err(|error| error.to_string())?;
    let user_config = fs::read_to_string(user_config_file).map_err(|error| error.to_string())?;
    let legacy_config: toml::Value =
        toml::from_str(&legacy_config).map_err(|error| error.to_string())?;
    let user_config: toml::Value =
        toml::from_str(&user_config).map_err(|error| error.to_string())?;

    if legacy_config == user_config {
        Ok(())
    } else {
        Err("copied config does not match legacy config".to_string())
    }
}

/// Recursively merge two TOML values, with `overlay` taking precedence.
///
/// Tables are merged key-by-key. Other values replace the previous value, so a
/// later `disabled = true` overrides an earlier `disabled = false`.
fn merge_toml(base: &mut toml::Value, overlay: toml::Value) {
    match (base, overlay) {
        (toml::Value::Table(base), toml::Value::Table(overlay)) => {
            for (key, value) in overlay {
                if let Some(base_value) = base.get_mut(&key) {
                    merge_toml(base_value, value);
                } else {
                    base.insert(key, value);
                }
            }
        }
        (base, overlay) => *base = overlay,
    }
}

/// Deserialize a style string in the starship format with serde
pub fn deserialize_style<'de, D>(de: D) -> Result<Style, D::Error>
where
    D: Deserializer<'de>,
{
    Cow::<'_, str>::deserialize(de).and_then(|s| {
        parse_style_string(s.as_ref(), None).ok_or_else(|| D::Error::custom("Invalid style string"))
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PrevColor {
    Fg,
    Bg,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// Wrapper for `nu_ansi_term::Style` that supports referencing the previous style's foreground/background color.
pub struct Style {
    style: nu_ansi_term::Style,
    bg: Option<PrevColor>,
    fg: Option<PrevColor>,
}

impl Style {
    pub fn to_ansi_style(&self, prev: Option<&nu_ansi_term::Style>) -> nu_ansi_term::Style {
        let Some(prev_style) = prev else {
            return self.style;
        };

        let mut current = self.style;

        if let Some(prev_color) = self.bg {
            match prev_color {
                PrevColor::Fg => current.background = prev_style.foreground,
                PrevColor::Bg => current.background = prev_style.background,
            }
        }

        if let Some(prev_color) = self.fg {
            match prev_color {
                PrevColor::Fg => current.foreground = prev_style.foreground,
                PrevColor::Bg => current.foreground = prev_style.background,
            }
        }

        current
    }

    fn map_style<F>(&self, f: F) -> Self
    where
        F: FnOnce(&nu_ansi_term::Style) -> nu_ansi_term::Style,
    {
        Self {
            style: f(&self.style),
            ..*self
        }
    }

    fn fg(&self, prev_color: PrevColor) -> Self {
        Self {
            fg: Some(prev_color),
            ..*self
        }
    }

    fn bg(&self, prev_color: PrevColor) -> Self {
        Self {
            bg: Some(prev_color),
            ..*self
        }
    }
}

impl From<nu_ansi_term::Style> for Style {
    fn from(value: nu_ansi_term::Style) -> Self {
        Self {
            style: value,
            ..Default::default()
        }
    }
}

impl From<nu_ansi_term::Color> for Style {
    fn from(value: nu_ansi_term::Color) -> Self {
        Self {
            style: value.into(),
            ..Default::default()
        }
    }
}

/** Parse a style string which represents an ansi style. Valid tokens in the style
 string include the following:
 - 'fg:<color>'    (specifies that the color read should be a foreground color)
 - 'bg:<color>'    (specifies that the color read should be a background color)
 - 'underline'
 - 'bold'
 - 'italic'
 - 'inverted'
 - 'blink'
 - '`prev_fg`'        (specifies the color should be the previous foreground color)
 - '`prev_bg`'        (specifies the color should be the previous background color)
 - '<color>'       (see the `parse_color_string` doc for valid color strings)
*/
pub fn parse_style_string(style_string: &str, context: Option<&Context>) -> Option<Style> {
    style_string
        .split_whitespace()
        .try_fold(Style::default(), |style, token| {
            let token = token.to_lowercase();

            // Check for FG/BG identifiers and strip them off if appropriate
            // If col_fg is true, color the foreground. If it's false, color the background.
            let (token, col_fg) = if token.as_str().starts_with("fg:") {
                (token.trim_start_matches("fg:").to_owned(), true)
            } else if token.as_str().starts_with("bg:") {
                (token.trim_start_matches("bg:").to_owned(), false)
            } else {
                (token, true) // Bare colors are assumed to color the foreground
            };

            match token.as_str() {
                "underline" => Some(style.map_style(nu_ansi_term::Style::underline)),
                "bold" => Some(style.map_style(nu_ansi_term::Style::bold)),
                "italic" => Some(style.map_style(nu_ansi_term::Style::italic)),
                "dimmed" => Some(style.map_style(nu_ansi_term::Style::dimmed)),
                "inverted" => Some(style.map_style(nu_ansi_term::Style::reverse)),
                "blink" => Some(style.map_style(nu_ansi_term::Style::blink)),
                "hidden" => Some(style.map_style(nu_ansi_term::Style::hidden)),
                "strikethrough" => Some(style.map_style(nu_ansi_term::Style::strikethrough)),

                "prev_fg" if col_fg => Some(style.fg(PrevColor::Fg)),
                "prev_fg" => Some(style.bg(PrevColor::Fg)),

                "prev_bg" if col_fg => Some(style.fg(PrevColor::Bg)),
                "prev_bg" => Some(style.bg(PrevColor::Bg)),

                // When the string is supposed to be a color:
                // Decide if we yield none, reset background or set color.
                color_string => {
                    if color_string == "none" && col_fg {
                        None // fg:none yields no style.
                    } else {
                        // Either bg or valid color or both.
                        let parsed = parse_color_string(
                            color_string,
                            context.and_then(|x| {
                                get_palette(
                                    &x.root_config.palettes,
                                    x.root_config.palette.as_deref(),
                                )
                            }),
                        );
                        // bg + invalid color = reset the background to default.
                        if !col_fg && parsed.is_none() {
                            let mut new_style = style;
                            new_style.style.background = Option::None;
                            Some(new_style)
                        } else {
                            // Valid color, apply color to either bg or fg
                            parsed.map(|ansi_color| {
                                if col_fg {
                                    style.map_style(|s| s.fg(ansi_color))
                                } else {
                                    style.map_style(|s| s.on(ansi_color))
                                }
                            })
                        }
                    }
                }
            }
        })
}

/** Parse a string that represents a color setting, returning None if this fails
 There are three valid color formats:
  - #RRGGBB      (a hash followed by an RGB hex)
  - u8           (a number from 0-255, representing an ANSI color)
  - colstring    (one of the 16 predefined color strings or a custom user-defined color)
*/
fn parse_color_string(
    color_string: &str,
    palette: Option<&Palette>,
) -> Option<nu_ansi_term::Color> {
    // Parse RGB hex values
    log::trace!("Parsing color_string: {color_string}");
    if color_string.starts_with('#') {
        log::trace!("Attempting to read hexadecimal color string: {color_string}");
        if color_string.len() != 7 {
            log::debug!("Could not parse hexadecimal string: {color_string}");
            return None;
        }
        let r: u8 = u8::from_str_radix(&color_string[1..3], 16).ok()?;
        let g: u8 = u8::from_str_radix(&color_string[3..5], 16).ok()?;
        let b: u8 = u8::from_str_radix(&color_string[5..7], 16).ok()?;
        log::trace!("Read RGB color string: {r},{g},{b}");
        return Some(Color::Rgb(r, g, b));
    }

    // Parse a u8 (ansi color)
    if let Result::Ok(ansi_color_num) = color_string.parse::<u8>() {
        log::trace!("Read ANSI color string: {ansi_color_num}");
        return Some(Color::Fixed(ansi_color_num));
    }

    // Check palette for a matching user-defined color
    if let Some(palette_color) = palette.as_ref().and_then(|x| x.get(color_string)) {
        log::trace!("Read user-defined color string: {color_string} defined as {palette_color}");
        return parse_color_string(palette_color, None);
    }

    // Check for any predefined color strings
    // There are no predefined enums for bright colors, so we use Color::Fixed
    let predefined_color = match color_string.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "purple" => Some(Color::Purple),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "bright-black" => Some(Color::DarkGray), // "bright-black" is dark grey
        "bright-red" => Some(Color::LightRed),
        "bright-green" => Some(Color::LightGreen),
        "bright-yellow" => Some(Color::LightYellow),
        "bright-blue" => Some(Color::LightBlue),
        "bright-purple" => Some(Color::LightPurple),
        "bright-cyan" => Some(Color::LightCyan),
        "bright-white" => Some(Color::LightGray),
        _ => None,
    };

    if predefined_color.is_some() {
        log::trace!("Read predefined color: {color_string}");
    } else {
        log::debug!("Could not parse color in string: {color_string}");
    }
    predefined_color
}

fn get_palette<'a>(
    palettes: &'a HashMap<String, Palette>,
    palette_name: Option<&str>,
) -> Option<&'a Palette> {
    if let Some(palette_name) = palette_name {
        let palette = palettes.get(palette_name);
        if palette.is_some() {
            log::trace!("Found color palette: {palette_name}");
        } else {
            log::warn!("Could not find color palette: {palette_name}");
        }
        palette
    } else {
        log::trace!("No color palette specified, using defaults");
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Env;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    fn env_with_home(home: &Path) -> Env<'static> {
        let mut env = Env::default();
        env.insert("HOME", home.to_string_lossy().to_string());
        env
    }

    fn write_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn config_sources_explicit_config_wins() {
        let dir = TempDir::new().unwrap();
        let mut env = env_with_home(dir.path());
        env.insert(
            "STARSHIP_CONFIG",
            dir.path().join("custom.toml").to_string_lossy().to_string(),
        );
        env.insert(
            "STARSHIP_CONFIG_HOME",
            dir.path()
                .join("starship-home")
                .to_string_lossy()
                .to_string(),
        );

        let sources = ConfigSources::from_env(&env);

        assert_eq!(
            sources.primary_edit_path(),
            Some(dir.path().join("custom.toml"))
        );
        assert_eq!(
            sources.ordered_config_files(),
            vec![dir.path().join("custom.toml")]
        );
    }

    #[test]
    fn config_sources_use_starship_config_home() {
        let dir = TempDir::new().unwrap();
        let mut env = env_with_home(dir.path());
        env.insert(
            "STARSHIP_CONFIG_HOME",
            dir.path()
                .join("starship-home")
                .to_string_lossy()
                .to_string(),
        );

        let sources = ConfigSources::from_env(&env);

        assert_eq!(
            sources.user_config_home,
            Some(dir.path().join("starship-home"))
        );
        assert_eq!(
            sources.user_config_file,
            Some(dir.path().join("starship-home/starship.toml"))
        );
        assert_eq!(
            sources.user_conf_d,
            Some(dir.path().join("starship-home/conf.d"))
        );
    }

    #[test]
    fn config_sources_use_xdg_config_home() {
        let dir = TempDir::new().unwrap();
        let mut env = env_with_home(dir.path());
        env.insert(
            "XDG_CONFIG_HOME",
            dir.path().join("xdg-config").to_string_lossy().to_string(),
        );

        let sources = ConfigSources::from_env(&env);

        assert_eq!(
            sources.user_config_home,
            Some(dir.path().join("xdg-config/starship"))
        );
    }

    #[test]
    fn config_sources_use_xdg_config_dirs() {
        let dir = TempDir::new().unwrap();
        let mut env = env_with_home(dir.path());
        env.insert(
            "XDG_CONFIG_DIRS",
            format!(
                "{}:{}",
                dir.path().join("xdg-a").display(),
                dir.path().join("xdg-b").display()
            ),
        );

        let sources = ConfigSources::from_env(&env);

        assert_eq!(
            sources.system_conf_dirs,
            vec![
                dir.path().join("xdg-a/starship/conf.d"),
                dir.path().join("xdg-b/starship/conf.d"),
            ]
        );
    }

    #[test]
    fn config_from_sources_merges_in_expected_order() {
        let dir = TempDir::new().unwrap();
        let system_conf_d = dir.path().join("system/starship/conf.d");
        let user_config_home = dir.path().join("user-starship");

        write_file(
            &system_conf_d.join("50-cli.toml"),
            r#"
[custom.tweet]
symbol = "bird"
disabled = false
"#,
        );
        write_file(
            &user_config_home.join("starship.toml"),
            r#"
[custom.tweet]
disabled = true
"#,
        );
        write_file(
            &user_config_home.join("conf.d/90-local.toml"),
            r#"
[custom.tweet]
format = "local"
"#,
        );

        let sources = ConfigSources {
            explicit_config: None,
            user_config_home: Some(user_config_home.clone()),
            user_config_file: Some(user_config_home.join("starship.toml")),
            user_conf_d: Some(user_config_home.join("conf.d")),
            system_conf_dirs: vec![system_conf_d],
            legacy_config_file: None,
        };

        let config = StarshipConfig::initialize_from_sources(&sources);
        let tweet = config
            .config
            .as_ref()
            .unwrap()
            .get("custom")
            .unwrap()
            .get("tweet")
            .unwrap()
            .as_table()
            .unwrap();

        assert_eq!(tweet.get("symbol").unwrap().as_str(), Some("bird"));
        assert_eq!(tweet.get("disabled").unwrap().as_bool(), Some(true));
        assert_eq!(tweet.get("format").unwrap().as_str(), Some("local"));
    }

    #[test]
    fn config_from_sources_sorts_conf_d_files() {
        let dir = TempDir::new().unwrap();
        let user_config_home = dir.path().join("user-starship");

        write_file(
            &user_config_home.join("starship.toml"),
            "[custom.order]\nvalue = 'main'\n",
        );
        write_file(
            &user_config_home.join("conf.d/10-last.toml"),
            "[custom.order]\nvalue = 'last'\n",
        );
        write_file(
            &user_config_home.join("conf.d/00-first.toml"),
            "[custom.order]\nvalue = 'first'\n",
        );

        let sources = ConfigSources {
            explicit_config: None,
            user_config_home: Some(user_config_home.clone()),
            user_config_file: Some(user_config_home.join("starship.toml")),
            user_conf_d: Some(user_config_home.join("conf.d")),
            system_conf_dirs: Vec::new(),
            legacy_config_file: None,
        };

        let config = StarshipConfig::initialize_from_sources(&sources);

        assert_eq!(
            config
                .config
                .as_ref()
                .unwrap()
                .get("custom")
                .unwrap()
                .get("order")
                .unwrap()
                .get("value")
                .unwrap()
                .as_str(),
            Some("last")
        );
    }

    #[test]
    fn legacy_config_is_copied_and_not_removed() {
        let dir = TempDir::new().unwrap();
        let legacy_config = dir.path().join(".config/starship.toml");
        let user_config_home = dir.path().join("xdg/starship");
        let user_config_file = user_config_home.join("starship.toml");
        write_file(&legacy_config, "[custom.legacy]\ncommand = 'true'\n");

        let sources = ConfigSources {
            explicit_config: None,
            user_config_home: Some(user_config_home.clone()),
            user_config_file: Some(user_config_file.clone()),
            user_conf_d: Some(user_config_home.join("conf.d")),
            system_conf_dirs: Vec::new(),
            legacy_config_file: Some(legacy_config.clone()),
        };

        let config = StarshipConfig::initialize_from_sources(&sources);

        assert!(legacy_config.exists());
        assert!(user_config_file.exists());
        assert!(config.config.unwrap().get("custom").is_some());
    }

    #[test]
    fn legacy_config_is_ignored_when_new_config_exists() {
        let dir = TempDir::new().unwrap();
        let legacy_config = dir.path().join(".config/starship.toml");
        let user_config_home = dir.path().join("xdg/starship");
        let user_config_file = user_config_home.join("starship.toml");
        write_file(&legacy_config, "[custom.legacy]\ncommand = 'true'\n");
        write_file(&user_config_file, "[custom.new]\ncommand = 'true'\n");

        let sources = ConfigSources {
            explicit_config: None,
            user_config_home: Some(user_config_home.clone()),
            user_config_file: Some(user_config_file),
            user_conf_d: Some(user_config_home.join("conf.d")),
            system_conf_dirs: Vec::new(),
            legacy_config_file: Some(legacy_config),
        };

        let config = StarshipConfig::initialize_from_sources(&sources);

        let custom = config.config.unwrap().remove("custom").unwrap();
        assert!(custom.get("new").is_some());
        assert!(custom.get("legacy").is_none());
    }
    use nu_ansi_term::Style as AnsiStyle;

    // Small wrapper to allow deserializing Style without a struct with #[serde(deserialize_with=)]
    #[derive(Default, Clone, Debug, PartialEq)]
    struct StyleWrapper(Style);

    impl<'de> Deserialize<'de> for StyleWrapper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_style(deserializer).map(Self)
        }
    }

    #[test]
    fn test_load_config() {
        #[derive(Clone, Default, Deserialize)]
        struct TestConfig<'a> {
            pub symbol: &'a str,
            pub disabled: bool,
            pub some_array: Vec<&'a str>,
        }

        let config = toml::toml! {
            symbol = "T "
            disabled = true
            some_array = ["A"]
        };
        let rust_config = TestConfig::from_config(&config).unwrap();

        assert_eq!(rust_config.symbol, "T ");
        assert!(rust_config.disabled);
        assert_eq!(rust_config.some_array, vec!["A"]);
    }

    #[test]
    fn test_load_nested_config() {
        #[derive(Clone, Default, Deserialize)]
        #[serde(default)]
        struct TestConfig<'a> {
            #[serde(borrow)]
            pub untracked: SegmentDisplayConfig<'a>,
            #[serde(borrow)]
            pub modified: SegmentDisplayConfig<'a>,
        }

        #[derive(PartialEq, Debug, Clone, Default, Deserialize)]
        #[serde(default)]
        struct SegmentDisplayConfig<'a> {
            pub value: &'a str,
            #[serde(deserialize_with = "deserialize_style")]
            pub style: Style,
        }

        let config = toml::toml! {
            untracked.value = "x"
            modified = { value = "∙", style = "red" }
        };

        let git_status_config = TestConfig::from_config(&config).unwrap();

        assert_eq!(
            git_status_config.untracked,
            SegmentDisplayConfig {
                value: "x",
                style: Style::default(),
            }
        );
        assert_eq!(
            git_status_config.modified,
            SegmentDisplayConfig {
                value: "∙",
                style: Color::Red.normal().into(),
            }
        );
    }

    #[test]
    fn test_load_optional_config() {
        #[derive(Clone, Default, Deserialize)]
        #[serde(default)]
        struct TestConfig<'a> {
            pub optional: Option<&'a str>,
            pub hidden: Option<&'a str>,
        }

        let config = toml::toml! {
            optional = "test"
        };
        let rust_config = TestConfig::from_config(&config).unwrap();

        assert_eq!(rust_config.optional, Some("test"));
        assert_eq!(rust_config.hidden, None);
    }

    #[test]
    fn test_load_enum_config() {
        #[derive(Clone, Default, Deserialize)]
        #[serde(default)]
        struct TestConfig {
            pub switch_a: Switch,
            pub switch_b: Switch,
            pub switch_c: Switch,
        }

        #[derive(Debug, PartialEq, Clone, Default)]
        enum Switch {
            On,
            #[default]
            Off,
        }

        impl<'de> Deserialize<'de> for Switch {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                match s.to_ascii_lowercase().as_str() {
                    "on" => Ok(Self::On),
                    _ => Ok(Self::Off),
                }
            }
        }

        let config = toml::toml! {
            switch_a = "on"
            switch_b = "any"
        };
        let rust_config = TestConfig::from_config(&config).unwrap();

        assert_eq!(rust_config.switch_a, Switch::On);
        assert_eq!(rust_config.switch_b, Switch::Off);
        assert_eq!(rust_config.switch_c, Switch::Off);
    }

    #[test]
    fn test_load_unknown_key_config() {
        #[derive(Clone, Default, Deserialize)]
        #[serde(default)]
        struct TestConfig<'a> {
            pub foo: &'a str,
        }

        let config = toml::toml! {
            foo = "test"
            bar = "ignore me"
        };
        let rust_config = TestConfig::from_config(&config);

        assert!(rust_config.is_ok());
        assert_eq!(rust_config.unwrap().foo, "test");
    }

    #[test]
    fn test_from_string() {
        let config = Value::String(String::from("S"));
        assert_eq!(<&str>::from_config(&config).unwrap(), "S");
    }

    #[test]
    fn test_from_bool() {
        let config = Value::Boolean(true);
        assert!(<bool>::from_config(&config).unwrap());
    }

    #[test]
    fn test_from_i64() {
        let config = Value::Integer(42);
        assert_eq!(<i64>::from_config(&config).unwrap(), 42);
    }

    #[test]
    fn test_from_style() {
        let config = Value::from("red bold");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Red.bold().into()
        );
    }

    #[test]
    fn test_from_hex_color_style() {
        let config = Value::from("#00000");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        let config = Value::from("#0000000");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        let config = Value::from("#NOTHEX");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        let config = Value::from("#a12BcD");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Rgb(0xA1, 0x2B, 0xCD).into()
        );
    }

    #[test]
    fn test_from_vec() {
        let config: Value = Value::Array(vec![Value::from("S")]);
        assert_eq!(<Vec<&str>>::from_config(&config).unwrap(), vec!["S"]);
    }

    #[test]
    fn test_from_option() {
        let config: Value = Value::String(String::from("S"));
        assert_eq!(<Option<&str>>::from_config(&config).unwrap(), Some("S"));
    }

    #[test]
    fn table_get_styles_bold_italic_underline_green_dimmed_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD");
        let mystyle = <StyleWrapper>::from_config(&config).unwrap().0;
        assert!(mystyle.to_ansi_style(None).is_bold);
        assert!(mystyle.to_ansi_style(None).is_italic);
        assert!(mystyle.to_ansi_style(None).is_underline);
        assert!(mystyle.to_ansi_style(None).is_dimmed);
        assert_eq!(
            mystyle.to_ansi_style(None),
            AnsiStyle::new()
                .bold()
                .italic()
                .underline()
                .dimmed()
                .fg(Color::Green)
        );
    }

    #[test]
    fn table_get_styles_bold_italic_underline_green_dimmed_inverted_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD InVeRTed");
        let mystyle = <StyleWrapper>::from_config(&config).unwrap().0;
        assert!(mystyle.to_ansi_style(None).is_bold);
        assert!(mystyle.to_ansi_style(None).is_italic);
        assert!(mystyle.to_ansi_style(None).is_underline);
        assert!(mystyle.to_ansi_style(None).is_dimmed);
        assert!(mystyle.to_ansi_style(None).is_reverse);
        assert_eq!(
            mystyle.to_ansi_style(None),
            AnsiStyle::new()
                .bold()
                .italic()
                .underline()
                .dimmed()
                .reverse()
                .fg(Color::Green)
        );
    }

    #[test]
    fn table_get_styles_bold_italic_underline_green_dimmed_blink_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD bLiNk");
        let mystyle = <StyleWrapper>::from_config(&config).unwrap().0;
        assert!(mystyle.to_ansi_style(None).is_bold);
        assert!(mystyle.to_ansi_style(None).is_italic);
        assert!(mystyle.to_ansi_style(None).is_underline);
        assert!(mystyle.to_ansi_style(None).is_dimmed);
        assert!(mystyle.to_ansi_style(None).is_blink);
        assert_eq!(
            mystyle.to_ansi_style(None),
            AnsiStyle::new()
                .bold()
                .italic()
                .underline()
                .dimmed()
                .blink()
                .fg(Color::Green)
        );
    }

    #[test]
    fn table_get_styles_bold_italic_underline_green_dimmed_hidden_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD hIDDen");
        let mystyle = <StyleWrapper>::from_config(&config).unwrap().0;
        assert!(mystyle.to_ansi_style(None).is_bold);
        assert!(mystyle.to_ansi_style(None).is_italic);
        assert!(mystyle.to_ansi_style(None).is_underline);
        assert!(mystyle.to_ansi_style(None).is_dimmed);
        assert!(mystyle.to_ansi_style(None).is_hidden);
        assert_eq!(
            mystyle.to_ansi_style(None),
            AnsiStyle::new()
                .bold()
                .italic()
                .underline()
                .dimmed()
                .hidden()
                .fg(Color::Green)
        );
    }

    #[test]
    fn table_get_styles_bold_italic_underline_green_dimmed_strikethrough_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD StRiKEthROUgh");
        let mystyle = <StyleWrapper>::from_config(&config).unwrap().0;
        assert!(mystyle.to_ansi_style(None).is_bold);
        assert!(mystyle.to_ansi_style(None).is_italic);
        assert!(mystyle.to_ansi_style(None).is_underline);
        assert!(mystyle.to_ansi_style(None).is_dimmed);
        assert!(mystyle.to_ansi_style(None).is_strikethrough);
        assert_eq!(
            mystyle.to_ansi_style(None),
            AnsiStyle::new()
                .bold()
                .italic()
                .underline()
                .dimmed()
                .strikethrough()
                .fg(Color::Green)
        );
    }

    #[test]
    fn table_get_styles_plain_and_broken_styles() {
        // Test a "plain" style with no formatting
        let config = Value::from("");
        let plain_style = <StyleWrapper>::from_config(&config).unwrap().0;
        assert_eq!(plain_style.to_ansi_style(None), AnsiStyle::new());

        // Test a string that's clearly broken
        let config = Value::from("djklgfhjkldhlhk;j");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test a string that's nullified by `none`
        let config = Value::from("fg:red bg:green bold none");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test a string that's nullified by `none` at the start
        let config = Value::from("none fg:red bg:green bold");
        assert!(<StyleWrapper>::from_config(&config).is_err());
    }

    #[test]
    fn table_get_styles_with_none() {
        // Test that none on the end will result in None, overriding bg:none
        let config = Value::from("fg:red bg:none none");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test that none in front will result in None, overriding bg:none
        let config = Value::from("none fg:red bg:none");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test that none in the middle will result in None, overriding bg:none
        let config = Value::from("fg:red none bg:none");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test that fg:none will result in None
        let config = Value::from("fg:none bg:black");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test that bg:none will yield a style
        let config = Value::from("fg:red bg:none");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Red.normal().into()
        );

        // Test that bg:none will yield a style
        let config = Value::from("fg:red bg:none bold");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Red.bold().into()
        );

        // Test that bg:none will overwrite the previous background colour
        let config = Value::from("fg:red bg:green bold bg:none");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Red.bold().into()
        );
    }

    #[test]
    fn table_get_styles_previous() {
        // Test that previous has no effect when there is no previous style
        let both_prevfg = <StyleWrapper>::from_config(&Value::from(
            "bold fg:black fg:prev_bg bg:prev_fg underline",
        ))
        .unwrap()
        .0;

        assert_eq!(
            both_prevfg.to_ansi_style(None),
            AnsiStyle::default().fg(Color::Black).bold().underline()
        );

        // But if there is a style on the previous string, then use that
        let prev_style = AnsiStyle::new()
            .underline()
            .fg(Color::Yellow)
            .on(Color::Red);

        assert_eq!(
            both_prevfg.to_ansi_style(Some(&prev_style)),
            AnsiStyle::new()
                .fg(Color::Red)
                .on(Color::Yellow)
                .bold()
                .underline()
        );

        // Test that all the combinations of previous colors work
        let fg_prev_fg = <StyleWrapper>::from_config(&Value::from("fg:prev_fg"))
            .unwrap()
            .0;
        assert_eq!(
            fg_prev_fg.to_ansi_style(Some(&prev_style)),
            AnsiStyle::new().fg(Color::Yellow)
        );

        let fg_prev_bg = <StyleWrapper>::from_config(&Value::from("fg:prev_bg"))
            .unwrap()
            .0;
        assert_eq!(
            fg_prev_bg.to_ansi_style(Some(&prev_style)),
            AnsiStyle::new().fg(Color::Red)
        );

        let bg_prev_fg = <StyleWrapper>::from_config(&Value::from("bg:prev_fg"))
            .unwrap()
            .0;
        assert_eq!(
            bg_prev_fg.to_ansi_style(Some(&prev_style)),
            AnsiStyle::new().on(Color::Yellow)
        );

        let bg_prev_bg = <StyleWrapper>::from_config(&Value::from("bg:prev_bg"))
            .unwrap()
            .0;
        assert_eq!(
            bg_prev_bg.to_ansi_style(Some(&prev_style)),
            AnsiStyle::new().on(Color::Red)
        );
    }

    #[test]
    fn table_get_styles_ordered() {
        // Test a background style with inverted order (also test hex + ANSI)
        let config = Value::from("bg:#050505 underline fg:120");
        let flipped_style = <StyleWrapper>::from_config(&config).unwrap().0;
        assert_eq!(
            flipped_style.to_ansi_style(None),
            AnsiStyle::new()
                .underline()
                .fg(Color::Fixed(120))
                .on(Color::Rgb(5, 5, 5))
        );

        // Test that the last color style is always the one used
        let config = Value::from("bg:120 bg:125 bg:127 fg:127 122 125");
        let multi_style = <StyleWrapper>::from_config(&config).unwrap().0;
        assert_eq!(
            multi_style.to_ansi_style(None),
            AnsiStyle::new().fg(Color::Fixed(125)).on(Color::Fixed(127))
        );
    }

    #[test]
    fn table_get_colors_palette() {
        // Test using colors defined in palette
        let mut palette = Palette::new();
        palette.insert("mustard".to_string(), "#af8700".to_string());
        palette.insert("sky-blue".to_string(), "51".to_string());
        palette.insert("red".to_string(), "#d70000".to_string());
        palette.insert("blue".to_string(), "17".to_string());
        palette.insert("green".to_string(), "green".to_string());

        assert_eq!(
            parse_color_string("mustard", Some(&palette)),
            Some(Color::Rgb(175, 135, 0))
        );
        assert_eq!(
            parse_color_string("sky-blue", Some(&palette)),
            Some(Color::Fixed(51))
        );

        // Test overriding predefined colors
        assert_eq!(
            parse_color_string("red", Some(&palette)),
            Some(Color::Rgb(215, 0, 0))
        );
        assert_eq!(
            parse_color_string("blue", Some(&palette)),
            Some(Color::Fixed(17))
        );

        // Test overriding a predefined color with itself
        assert_eq!(
            parse_color_string("green", Some(&palette)),
            Some(Color::Green)
        );
    }

    #[test]
    fn table_get_palette() {
        // Test retrieving color palette by name
        let mut palette1 = Palette::new();
        palette1.insert("test-color".to_string(), "123".to_string());

        let mut palette2 = Palette::new();
        palette2.insert("test-color".to_string(), "#ABCDEF".to_string());

        let mut palettes = HashMap::<String, Palette>::new();
        palettes.insert("palette1".to_string(), palette1);
        palettes.insert("palette2".to_string(), palette2);

        assert_eq!(
            get_palette(&palettes, Some("palette1"))
                .unwrap()
                .get("test-color")
                .unwrap(),
            "123"
        );

        assert_eq!(
            get_palette(&palettes, Some("palette2"))
                .unwrap()
                .get("test-color")
                .unwrap(),
            "#ABCDEF"
        );

        // Test retrieving nonexistent color palette
        assert!(get_palette(&palettes, Some("palette3")).is_none());

        // Test default behavior
        assert!(get_palette(&palettes, None).is_none());
    }

    #[test]
    fn read_config_no_config_file_path_provided() {
        assert_eq!(
            None,
            StarshipConfig::read_config_content_as_str(None),
            "if the platform doesn't have utils::home_dir(), it should return None"
        );
    }

    /// Guard against schemars `$defs` collisions by requiring distinct IDs for nested `VecOr` types.
    #[cfg(feature = "config-schema")]
    #[test]
    fn vec_or_schema_ids_distinguish_nesting() {
        use schemars::JsonSchema;

        assert_ne!(
            VecOr::<VecOr<&str>>::schema_id(),
            VecOr::<&str>::schema_id(),
        );
        assert_ne!(
            VecOr::<VecOr<&str>>::schema_name(),
            VecOr::<&str>::schema_name(),
        );
    }
}

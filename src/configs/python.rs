use crate::config::VecOr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct PythonConfig<'a> {
    pub pyenv_version_name: bool,
    pub pyenv_prefix: &'a str,
    #[cfg_attr(
        feature = "config-schema",
        schemars(
            description = "Ordered list of Python executables to probe, tried in order until one responds. Each item is a command name (e.g. \"python3\") or an argv prefix array (e.g. [\"python\", \"-3\"])."
        )
    )]
    pub python_binary: VecOr<VecOr<&'a str>>,
    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub detect_env_vars: Vec<&'a str>,
    pub generic_venv_names: Vec<&'a str>,
}

impl Default for PythonConfig<'_> {
    fn default() -> Self {
        Self {
            pyenv_version_name: false,
            pyenv_prefix: "pyenv ",
            python_binary: VecOr(vec![
                VecOr(vec!["python"]),
                VecOr(vec!["python3"]),
                VecOr(vec!["python2"]),
            ]),
            format: "via [${symbol}${pyenv_prefix}(${version} )(\\($virtualenv\\) )]($style)",
            version_format: "v${raw}",
            style: "yellow bold",
            symbol: "🐍 ",
            disabled: false,
            detect_extensions: vec!["py", "ipynb"],
            detect_files: vec![
                "requirements.txt",
                ".python-version",
                "pyproject.toml",
                "Pipfile",
                "tox.ini",
                "setup.py",
                "__init__.py",
            ],
            detect_folders: vec![],
            detect_env_vars: vec!["VIRTUAL_ENV"],
            generic_venv_names: vec![],
        }
    }
}

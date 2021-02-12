use crate::config::{ModuleConfig, RootModuleConfig, VecOr};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PythonConfig<'a> {
    pub pyenv_version_name: bool,
    pub pyenv_prefix: &'a str,
    pub python_binary: VecOr<&'a str>,
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for PythonConfig<'a> {
    fn new() -> Self {
        PythonConfig {
            pyenv_version_name: false,
            pyenv_prefix: "pyenv ",
            python_binary: VecOr(vec!["python", "python3", "python2"]),
            format: "via [${symbol}${pyenv_prefix}(${version} )(\\($virtualenv\\))]($style)",
            style: "yellow bold",
            symbol: "🐍 ",
            disabled: false,
            detect_extensions: vec!["py"],
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
        }
    }
}

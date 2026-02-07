# Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the
current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version
name. Otherwise, it will display the version number from `python --version`.

By default, the module will be shown if any of the following conditions are met:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- The current directory contains a file with the `.ipynb` extension.
- A virtual environment is currently activated

### Options

| Option               | Default                                                                                                      | Description                                                                           |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                    | The format for the module.                                                            |
| `version_format`     | `'v${raw}'`                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch`             |
| `symbol`             | `'🐍 '`                                                                                                      | A format string representing the symbol of Python                                     |
| `style`              | `'yellow bold'`                                                                                              | The style for the module.                                                             |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                       |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Prefix before pyenv version display, only used if pyenv is used                       |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should execute when getting the version. |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Which extensions should trigger this module                                           |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Which filenames should trigger this module                                            |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                              |
| `generic_venv_names` | `[]`                                                                                                         | Which venv names should be replaced with the parent directory name.                   |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                         |

> [!TIP]
> The `python_binary` variable accepts either a string or a list of strings.
> Starship will try executing each binary until it gets a result. Note you can
> only change the binary that Starship executes to get the version of Python not
> the arguments that are used.
>
> The default values and order for `python_binary` was chosen to first identify
> the Python version in a virtualenv/conda environments (which currently still
> add a `python`, no matter if it points to `python3` or `python2`). This has the
> side effect that if you still have a system Python 2 installed, it may be
> picked up before any Python 3 (at least on Linux Distros that always symlink
> `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but
> cannot remove the system Python 2, changing this to `'python3'` will hide any
> Python version 2, see example below.

### Variables

| Variable     | Example         | Description                                                                 |
| ------------ | --------------- | --------------------------------------------------------------------------- |
| version      | `'v3.8.1'`      | The version of `python`                                                     |
| symbol       | `'🐍 '`         | Mirrors the value of option `symbol`                                        |
| style        | `'yellow bold'` | Mirrors the value of option `style`                                         |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix`                                  |
| virtualenv   | `'venv'`        | The current `virtualenv` name or the parent if matches `generic_venv_names` |

### Example

```toml
# ~/.config/starship.toml

[python]
symbol = '👾 '
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

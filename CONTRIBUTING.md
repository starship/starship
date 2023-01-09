# Contributing

🚀 Thank you for contributing to starship! 🚀

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By contributing to this project you agree to abide by its terms.

If you have any questions that aren't addressed in this document, please don't hesitate to open an issue or drop into our [Discord server](https://discord.gg/8Jzqu3T)! 💬

## Glossary

- **Module**: A component in the prompt giving information based on contextual information from your OS. For example, the `rust` module shows the version of Rust that is currently installed on your computer, if your current directory is a Rust project.

- **Segment**: Smaller sub-components that compose a module. For example, the `symbol` segment in the `rust` module contains the character that is shown before the version number (`🦀` by default).

## Philosophy

We aim to make starship as fast, robust and reliable as possible, while also allowing for extensive customization. We do so by leveraging Rust's inherent safety and with thorough cross-platform testing. We also do our best to eliminate unnecessary work when displaying the prompt by reducing repeated work and by using caching to our favor.

If you spot anywhere that we could trim some time or reduce the prompt's workload, we will gladly accept new issues or PRs! 😄

## Architecture

The project begins in [`main.rs`](src/main.rs), where the appropriate `print::` method is called based on which arguments are given to [clap](https://crates.io/crates/clap). When printing the full prompt, we use [rayon](https://crates.io/crates/rayon) to parallelize the computation of modules.

Any styling that is applied to a module is inherited by its segments. Module prefixes and suffixes by default don't have any styling applied to them.

## Environment Variables and external commands

We have custom functions to be able to test our modules better. Here we show you how.

### Environment Variables

To get an environment variable we have special function to allow for mocking of vars. Here's a quick example:

```rust
use super::{Context, Module, RootModuleConfig};

use crate::configs::php::PhpConfig;
use crate::formatter::StringFormatter;


pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
   // Here `my_env_var` will be either the contents of the var or the function
   // will exit if the variable is not set.
   let my_env_var = context.get_env("MY_VAR")?;

   // Then you can happily use the value
}
```

## External commands

To run a external command (e.g. to get the version of a tool) and to allow for mocking use the `context.exec_cmd` function. Here's a quick example:

```rust
use super::{Context, Module, ModuleConfig};

use crate::configs::php::PhpConfig;
use crate::formatter::StringFormatter;


pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
   // Here `output` will be either the stdout of the called command or the function
   // will exit if the called program was not installed or could not be run.
   let output = context.exec_cmd("my_command", &["first_arg", "second_arg"])?.stdout;

   // Then you can happily use the output
}
```

If using `context.exec_cmd` isn't possible, please use `crate::utils::create_command` instead of `std::process::Command::new`.

## Absolute Filenames

To use absolute filenames in your module, use `crate::utils::context_path()` to create a `PathBuf` from an absolute pathname.
In the test environment the root directory will be replaced with a `Tempdir`, which you can get via `ModuleRenderer::root_path()`.
So, you can populate that mocked root directory with any files you want.

```rust
use crate::utils::context_path;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    if !context_path(context, "/run/test/testfile").exists() {
        return None
    }
    // ..
}
```

```rust
#[test]
fn test_testfile() {
    let renderer = ModuleRenderer::new("mymodule");

    let root_path = renderer.root_path();

    // This creates `$TEMPDIR/run/test/testfile`

    let mut absolute_test_file = PathBuf::from(root_path);

    absolute_test_file.push("run");
    absolute_test_file.push("test");
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&absolute_test_file)?;

    absolute_test_file.push("testfile");
    std::fs::File::create(&absolute_test_file)?;

    // ...
}
```

## Logging

Debug logging in starship is done with our custom logger implementation.
To run starship with debug logs, set the `STARSHIP_LOG` environment variable to the log level needed.
For example, to enable the trace logs, run the following:

```sh
# Run installed starship
STARSHIP_LOG=trace starship

# Run with cargo
STARSHIP_LOG=trace cargo run
```

## Linting

Starship source files are linted with [clippy](https://crates.io/crates/clippy). Clippy will be ran as part of CI. Linting errors will fail a build, so it is suggested that you run Clippy locally:

```sh
rustup component add clippy
cargo clippy --all-targets --all-features
```

## Formatting

Starship source files are formatted with [rustfmt](https://crates.io/crates/rustfmt-nightly). Markdown and TOML files (among others) are formatted with [dprint](https://github.com/dprint/dprint). Unformatted code will fail the CI, so it is suggested that you run these tools locally.

For rustfmt:

```sh
rustup component add rustfmt
cargo fmt
```

For dprint:

```sh
cargo install dprint
dprint fmt
```

Editor plugins/functionality may help you run these automatically so that you don't accidentally create a PR that fails.

If your changes cause changes to the configuration, you will need to update the configuration schema in `.github/config-schema.json` with `cargo run --features config-schema -- config-schema > .github/config-schema.json`.

## Testing

Testing is critical to making sure starship works as intended on systems big and small. Starship interfaces with many applications and system APIs when generating the prompt, so there's a lot of room for bugs to slip in.

Unit tests are written using the built-in Rust testing library in the same file as the implementation, as is traditionally done in Rust codebases. These tests can be run with `cargo test` and are run on GitHub as part of our GitHub Actions continuous integration to ensure consistent behavior.

All tests that test the rendered output of a module should use `ModuleRenderer`. For Example:

```rust
use super::{Context, Module, ModuleConfig};

use crate::configs::php::PhpConfig;
use crate::formatter::StringFormatter;
use crate::utils;


pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
   /* This is where your module code goes */
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::test::ModuleRenderer;
   use nu_ansi_term::Color;
   use std::fs::File;
   use std::io;


   #[test]
   fn should_render() -> io::Result<()> {
      // Here you setup the testing environment
      let tempdir = tempfile::tempdir()?;
      // Create some file needed to render the module
      File::create(tempdir.path().join("YOUR_FILE"))?.sync_all()?;

      // The output of the module
      let actual = ModuleRenderer::new("YOUR_MODULE_NAME")
         // For a custom path
         .path(&tempdir.path())
         // For a custom config
         .config(toml::toml!{
            [YOUR_MODULE_NAME]
            val = 1
         })
         // For env mocking
         .env("KEY","VALUE")
         // Run the module and collect the output
         .collect();

      // The value that should be rendered by the module.
      let expected = Some(format!("{} ",Color::Black.paint("THIS SHOULD BE RENDERED")));

      // Assert that the actual and expected values are the same
      assert_eq!(actual, expected);

      // Close the tempdir
      tempdir.close()
   }
}
```

If a module depends on output of another program, then that output should be added to the match statement in [`utils.rs`](src/utils.rs). The match has to be exactly the same as the call to `utils::exec_cmd()`, including positional arguments and flags. The array of arguments are joined by a `" "`, so `utils::exec_cmd("program", &["arg", "more_args"])` would match with the `program arg more_args` match statement.

If the program cannot be mocked (e.g. It performs some filesystem operations, either writing or reading files) then it has to added to the project's GitHub Actions workflow file([`.github/workflows/workflow.yml`](.github/workflows/workflow.yml)) and the test has to be marked with an `#[ignored]`. This ensures that anyone can run the test suite locally without needing to pre-configure their environment. The `#[ignored]` attribute is bypassed during CI runs in GitHub Actions.

Unit tests should be fully isolated, only testing a given function's expected output given a specific input, and should be reproducible on any machine. Unit tests should not expect the computer running them to be in any particular state. This includes having any applications pre-installed, having any environment variables set, etc.

The previous point should be emphasized: even seemingly innocuous ideas like "if we can see the directory, we can read it" or "nobody will have their home directory be a git repo" have bitten us in the past. Having even a single test fail can completely break installation on some platforms, so be careful with tests!

### Test Programming Guidelines

Any tests that depend on File I/O should use [`sync_all()`](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all) when creating files or after writing to files.

Any tests that use `tempfile::tempdir` should take care to call `dir.close()` after usage to ensure the lifecycle of the directory can be reasoned about. This includes `fixture_repo()` as it returns a TempDir that should be closed.

## Documentation

### Crowdin Translated Pages

Many documentation pages have versions in non-English languages. These
translated pages are managed by
[Crowdin](https://crowdin.com/project/starship-prompt). Please do not edit
these pages directly, even for changes that do not need to be translated (e.g.
whitespace or emoji changes), since this can cause merges to fail.

If you would like to contribute translations or corrections to the Crowdin
generated pages, please visit our Crowdin site.

### Running the Documentation Website Locally

Changes to documentation can be viewed in a rendered state from the GitHub PR page
(go to the CI section at the bottom of the page and look for "deploy preview", then
click on "details"). If you want to view changes locally as well, follow these steps.

After cloning the project, you can do the following to run the VuePress website on your local machine:

1. `cd` into the `/docs` directory.
2. Install the project dependencies:

   ```sh
   npm install
   ```

3. Start the project in development mode:

   ```sh
   npm run dev
   ```

Once setup is complete, you can refer to VuePress documentation on the actual implementation here: <https://vuepress.vuejs.org/guide/>.

## Git/GitHub workflow

This is our preferred process for opening a PR on GitHub:

1. Fork this repository
2. Create a branch off of `master` for your work: `git checkout -b my-feature-branch`
3. Make some changes, committing them along the way
4. When your changes are ready for review, push your branch: `git push origin my-feature-branch`
5. Create a pull request from your branch to `starship/master`
6. No need to assign the pull request to anyone, we'll review it when we can
7. When the changes have been reviewed and approved, someone will squash and merge for you

## New Module Checklist

We love getting new modules for starship! While we try to keep the barrier for
writing new modules low, starship provides a lot of functionality for a module,
which requires quite a few things be done. These are listed here to help
everyone remember what they are. Don't worry: most of them are quite simple!

- [ ] Add a section to `docs/config/README.md` describing the module, and
      its configuration options/variables (more documentation is often
      appropriate--this is a bare minimum).
- [ ] Add the variable to the appropriate location in the "Default Prompt
      Format" section of the documentation
- [ ] Add an appropriate choice of options to each preset in `docs/.vuepress/public/presets/toml`
- [ ] Update the config file schema by running `cargo run --features config-schema -- config-schema > .github/config-schema.json`
- [ ] Create configs structs/traits in `src/configs/<module>.rs` and add the
      following:
  - [ ] An entry in `PROMPT_ORDER` (`src/configs/starship_root.rs`)
  - [ ] An entry in `FullConfig` and the `Default` impl (`src/configs/mod.rs`)
  - [ ] An entry in `ALL_MODULES` (`src/module.rs`)
  - [ ] A `mod` declaration at the top of `src/modules/mod.rs`
  - [ ] An entry in `handle()` (`src/modules/mod.rs`)
  - [ ] A description for the `description()` function (`src/modules/mod.rs`)

Finally, you should make sure to write your module's code in `src/modules`
and add any commands that need to be mocked when testing in `src/utils.rs`.
Command output can also be mocked in test by using `ModuleRenderer::cmd`.

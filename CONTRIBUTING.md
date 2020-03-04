# Contributing

🚀 Thank you for contributing to starship! 🚀

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By contributing to this project you agree to abide by its terms.

If you have any questions that aren't addressed in this document, please don't hesitate to open an issue or drop into our [Discord server](https://discord.gg/8Jzqu3T)! 💬

## Glossary

- **Module**: A component in the prompt giving information based on contextual information from your OS. For example, the `nodejs` module shows the version of NodeJS that is currently installed on your computer, if your current directory is a NodeJS project.

- **Segment**: Smaller sub-components that compose a module. For example, the `symbol` segment in the `nodejs` module contains the character that is shown before the version number (`⬢` by default).

## Philosophy

We aim to make starship as fast, robust and reliable as possible, while also allowing for extensive customization. We do so by leveraging Rust's inherent safety and with thorough cross-platform testing. We also do our best to eliminate unnecessary work when displaying the prompt by reducing repeated work and by using caching to our favor.

If you spot anywhere that we could trim some time or reduce the prompt's workload, we will gladly accept new issues or PRs! 😄

## Architecture

The project begins in [`main.rs`](src/main.rs), where the appropriate `print::` method is called based on which arguments are given to [clap](https://crates.io/crates/clap). When printing the full prompt, we use [rayon](https://crates.io/crates/rayon) to parallelize the computation of modules.

Any styling that is applied to a module is inherited by its segments. Module prefixes and suffixes by default don't have any styling applied to them.

## Logging

Debug logging in starship is done with [pretty_env_logger](https://crates.io/crates/pretty_env_logger).
To run starship with debug logs, set the `RUST_LOG` environment variable to the log level needed.
For example, to enable the trace logs, run the following:

```sh
# Run installed starship
RUST_LOG=starship=trace starship

# Run with cargo
RUST_LOG=starship=trace cargo run
```

## Linting

Starship source files are linted with [clippy](https://crates.io/crates/clippy). Clippy will be ran as part of CI. Linting errors will fail a build, so it is suggested that you run Clippy locally:

```sh
rustup component add clippy
cargo clippy
```

## Formatting

Starship source files are formatted with [rustfmt](https://crates.io/crates/rustfmt-nightly). Rustfmt will be ran as part of CI. Unformatted code will fail a build, so it is suggested that you run rustfmt locally:

```sh
rustup component add rustfmt
cargo fmt
```


## Testing

Testing is critical to making sure starship works as intended on systems big and small. Starship interfaces with many applications and system APIs when generating the prompt, so there's a lot of room for bugs to slip in.

Unit tests and a subset of integration tests can be run with `cargo test`.
The full integration test suite is run on GitHub as part of our GitHub Actions continuous integration.

### Unit Testing

Unit tests are written using the built-in Rust testing library in the same file as the implementation, as is traditionally done in Rust codebases. These tests can be run with `cargo test`.

Unit tests should be fully isolated, only testing a given function's expected output given a specific input, and should be reproducible on any machine. Unit tests should not expect the computer running them to be in any particular state. This includes having any applications pre-installed, having any environment variables set, etc.

The previous point should be emphasized: even seemingly innocuous ideas like "if we can see the directory, we can read it" or "nobody will have their home directory be a git repo" have bitten us in the past. Having even a single test fail can completely break installation on some platforms, so be careful with tests!

### Integration Testing

Integration tests are located in the [`tests/`](tests) directory and are also written using the built-in Rust testing library.

Integration tests should test full modules or the entire prompt. All integration tests that expect the testing environment to have pre-existing state or tests that make permanent changes to the filesystem should have the `#[ignore]` attribute added to them. All tests that don't depend on any preexisting state will be run alongside the unit tests with `cargo test`.

For tests that depend on having preexisting state, whatever needed state will have to be added to the project's GitHub Actions workflow file([`.github/workflows/workflow.yml`](.github/workflows/workflow.yml)).

### Onine one-click Setup

Contribute to Starship using a fully featured online development environment; cloned repo, pre-installed dependencies, running web server for docs and `cargo watch -x run` running.

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/from-referrer/)

## Running the Documentation Website Locally

If you are contributing to the design of Starship's website, the following section will help you get started.

### Setup

After cloning the project, you can do the following to run the VuePress website on your local machine:

1. `cd` into the `/docs` directory.
2. Install the project dependencies:
```
$ npm install
```
3. Start the project in development mode:
```
$ npm run dev
```

Once setup is complete, you can refer to VuePress documentation on the actual implementation here: https://vuepress.vuejs.org/guide/.

### Git/GitHub workflow

This is our preferred process for opening a PR on GitHub:

1. Fork this repository
2. Create a branch off of `master` for your work: `git checkout -b my-feature-branch`
3. Make some changes, committing them along the way
4. When your changes are ready for review, push your branch: `git push origin my-feature-branch`
5. Create a pull request from your branch to `starship/master`
6. No need to assign the pull request to anyone, we'll review it when we can
7. When the changes have been reviewed and approved, someone will squash and merge for you
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

Unit tests and a subset of acceptance tests can be run with `cargo test`.
The full acceptance test suite can be run in a Docker container with the included [`./integration_test`](integration_test) script.

### Unit Testing

Unit tests are written using the built-in Rust testing library in the same file as the implementation, as is traditionally done in Rust codebases. These tests can be run with `cargo test`.

Unit tests should be fully isolated, only testing a given function's expected output given a specific input, and should be reproducible on any machine. Unit tests should not expect the computer running them to be in any particular state. This includes having any applications pre-installed, having any environment variables set, etc.

The previous point should be emphasized: even seemingly innocuous ideas like "if we can see the directory, we can read it" or "nobody will have their home directory be a git repo" have bitten us in the past. Having even a single test fail can completely break installation on some platforms, so be careful with tests!

### Acceptance Testing

Acceptance tests are located in the [`tests/`](tests) directory and are also written using the built-in Rust testing library.

Acceptance tests should test full modules or the entire prompt. All integration tests expecting the testing environment to have preexisting state or making permanent changes to the filesystem should have the `#[ignore]` attribute. These tests will be run in a Docker container, by running the included [`./integration_test`](integration_test) script. All tests that don't depend on any preexisting state will be run alongside the unit tests with `cargo test`.

For tests that depend on having preexisting state, whatever needed state will have to be added to the project's Dockerfile ([`tests/Dockerfile`](tests/Dockerfile)) as well as the project's Azure Pipelines configuration ([`azure-pipelines.yml`](azure-pipelines.yml)).

The reason for having _both_ the Dockerfile as well as the Azure Pipelines configuration is in order to allow acceptance tests to be run on your local development environment via Docker, while also running our test suite on all supported OSes (Windows, Mac, Linux) on Azure Pipelines.

### Benchmarking

Benchmarks are located in the [`benches/`](benches) directory and are written using the [Criterion](https://crates.io/crates/criterion) library.

For the time being, benchmarks aren't actively used, but we plan to integrate benchmark comparison reporting into our CI pipeline in the near future. For the time being, they can be manually run with `cargo bench`.

## Contribution Guidelines

Thank you for considering contributing to this project!
Contributions are always welcome and appreciated.

### How to Contribute

Please check the [issue tracker](https://github.com/habedi/graphina/issues) to see if there is an issue you
would like to work on or if it has already been resolved.

#### Reporting Bugs

1. Open an issue on the [issue tracker](https://github.com/habedi/graphina/issues).
2. Include information such as steps to reproduce, expected/actual behavior, and relevant logs or screenshots.

#### Suggesting Features

1. Open an issue on the [issue tracker](https://github.com/habedi/graphina/issues).
2. Provide details about the feature, its purpose, and potential implementation ideas.

### Submitting Pull Requests

- Make sure all tests pass before submitting a pull request.
- Write a clear description of the changes you made and the reasons behind them.

> [!IMPORTANT]
> Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in the work, as defined
> in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.

### Development Workflow

#### Nix Environments

With Nix and flakes enabled, enter a development shell from the repository root.

| Command | Environment |
| --- | --- |
| `nix develop` | Rust development with the compiler and components from `rust-toolchain.toml`, Python, and project tools. |
| `nix develop .#verification` | Lean and Lake from `verification/lean-toolchain`. |
| `nix develop .#publishing` | uv for uploads and Twine for the isolated package metadata and README check. |
| `nix develop .#extraction` | Pinned Aeneas, Charon, its Rust nightly toolchain, and the matching Lean backend version. |

#### Code Style

- Use the `make format` command to format the code.

#### Running Tests

- Use the `make test` command to run the tests.

#### Running Linters

- Use the `make lint` command to run the linters.

#### See Available Commands

- Run `make help` to see all available commands for managing different tasks.

### Code of Conduct

We adhere to the project's [Code of Conduct](CODE_OF_CONDUCT.md).

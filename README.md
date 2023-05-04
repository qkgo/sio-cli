# Rust Socket.IO Performance Test CLI

This is a command-line interface (CLI) for testing the performance of a Socket.IO server using the [rust-socketio](https://github.com/1c3t3a/rust-socketio) library. This tool measures the time taken for handshake dialing, onConnect connection, and returns the HTTP status code and header lists.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## Building the CLI

To build the CLI, simply run:

```sh
cargo build --release
```
This will create the socketio_perf_test binary in the target/release directory.

Usage
To run the performance test, use the following command, replacing <test_path> with the path to your test server:

```sh
./target/release/socketio_perf_test <test_path>
```
The CLI will output the handshake dialing time, onConnect connection time, HTTP status code, and header lists.

GitHub Actions
This repository includes a GitHub Actions workflow for automatically building the project on push and pull requests to the main or master branch (choose based on your default branch name). The workflow configuration can be found in the .github/workflows/build.yml file.

When the workflow is triggered, you can view the build process under the "Actions" tab of your GitHub repository. After a successful build, the size of the build artifact will be displayed in the workflow logs.

License
This project is licensed under the MIT License.
[![CI](https://github.com/nogibjj/SiMinL_Week7/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/SiMinL_Week7/actions/workflows/CI.yml)

# Rust Project Template with functional CI/CD, devcontainer, dockerfile
This repository contains Rust implementations for approximating definite integrals of functions with the Trapezoidal Rule numerical method. Basic structure including CI/CD, Testing, Makefile and reproducible environments are included.

# Project Structure
- Devcontainer
- Makefile
- github workflows containing the CI/CD pipeline
- SRC: Contains lib.rs which has the core functions to compute the integral, and main.rs which calls functions from lib.rs
- Tests to ensure tool works correctly

# Running the Code
## Option 1
Prerequisites: Rust is installed

1. Build and run
- cargo build
2. Execute the main program
- cargo run
3. Testing
- make test
4. Linting
- Make lint
5. Formatting
- Make format

## Option 2
Download Prebuilt Binary (No Rust Required)
Download the prebuilt binary artifact. The artifact in CI/CD is a file or set of files generated during the build / test/ deployment phases that are saved and made accessible for future use.

1. Go to the Actions tab in the repository.
2. Select the latest workflow run.
3. Scroll down to Artifacts and download the file.
4. Move the binary to a Linux environment where you want to run the tool (e.g., GitHub Codespaces or a Linux machine).

# CI/CD Pipeline
This project uses GitHub Actions for continuous integration. The pipeline automatically:
- Checks formatting using cargo fmt.
- Lints the code with cargo clippy.
- Runs tests using cargo test.
- Builds the project in release mode using cargo build --release.
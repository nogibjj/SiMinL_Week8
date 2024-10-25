[![Rust CI/CD Pipeline](https://github.com/nogibjj/SiMinL_Week8/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/SiMinL_Week8/actions/workflows/CI.yml)

# Individual Project 2: Rust CLI Binary with SQLite
## Requirements
- Rust source code: The code should comprehensively understand Rust's syntax and unique features.
- Use of LLM: In your README, explain how you utilized an LLM in your coding process.
- SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
- Optimized Rust Binary: Include a process that generates an optimized Rust binary as a Gitlab Actions artifact that can be downloaded.
- README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how Gitlab Copilot was used.
- Github/Gitlab Actions: A workflow file that tests, builds, and lints your Rust code.
- Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.

# Video Demo

# Components
- Data Extraction: The extract function downloads data from a specified URL and saves it to a local file.
- Data Transformation and Loading: The transform_load function reads a CSV dataset and inserts its records into a SQLite database after performing necessary table operations. It creates a table named  with specific columns.
- Database Querying: The query function allows users to perform SELECT, INSERT, UPDATE, and DELETE operations on the database. It logs the queries into a Markdown file named query_log.md.
- Logging: The log_query function appends SQL queries to a log file, facilitating tracking and analysis of executed queries.

# Running the Code
1. Build and run
- cargo build for dependencies installation
2. Extract
- cargo run extract
3. Transform and load
cargon run transform_load
4. Query sample
make create, make read, make update or make delete to see sample CRUD operations

# Testing
3. Testing
- make test
4. Linting
- Make lint
5. Formatting
- Make format

# Optimised Rust Binary
Find and download the uploaded artifact by going to actions and clicking on the latest workflow run
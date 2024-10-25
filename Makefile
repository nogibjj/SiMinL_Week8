# Makefile for Rust Project

# Default target
all: format lint test build

# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	@rustc --version              # Rust compiler
	@cargo --version              # Rust package manager
	@rustfmt --version            # Rust code formatter
	@rustup --version             # Rust toolchain manager
	@clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build the project
build:
	cargo build

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

#Debug mode: binary is found at target/debug/
#release mode: binary is found at target/release. Uses full optimisations.
# Extract data
extract:
	cargo run -- extract

# Transform and Load data
transform_load:
	cargo run -- transform_load

# Create a database entry
create_record:
	cargo run -- create_record "Computer Science" "STEM" 1500 1200

# Read data from the database
read_data:
	cargo run -- read_data

# Update a database entry
update_record:
	cargo run -- update_record 1 "Electrical Engineering" "STEM" 2000 1500

# Delete a database entry
delete_record:
	cargo run -- delete_record 1

# Execute a general SQL query
general_query:
	cargo run -- general_query "SELECT * FROM gradstudentsDB WHERE Major='CONSTRUCTION SERVICES';"

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi
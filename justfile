set windows-shell := ["pwsh.exe", "-Command"]
set shell := ["bash", "-uc"]

_help:
	@just -l

# Build for release.
release:
	cargo build --release

# Install required tools
setup:
	cargo install cargo-nextest tomato-toml

# Run tests.
test:
	cargo nextest run

# Clippy has opinions. Find out what they are.
@lint:
	cargo clippy --all-targets --no-deps
	cargo +nightly fmt --fix

# Set the crate version and tag the repo to match. Requires bash.
tag VERSION:
    #!/usr/bin/env bash
    set -e
    tomato set package.version {{VERSION}} Cargo.toml
    cargo check
    git commit Cargo.toml Cargo.lock -m "v{{VERSION}}"
    git tag "v{{VERSION}}"
    echo "Release tagged for version v{{VERSION}}"

# Build the mod containing the folder structure to help people get going.
archive:
	echo "Haven't written this yet."

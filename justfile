set windows-shell := ["pwsh.exe", "-Command"]
set shell := ["bash", "-uc"]

# List available recipes.
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
	cargo clippy --all-targets --no-deps --fix --allow-dirty
	cargo +nightly fmt

# Set the crate version and tag the repo to match. Requires bash.
tag VERSION:
    #!/usr/bin/env bash
    set -e
    tomato set package.version {{VERSION}} Cargo.toml
    cargo check
    git commit Cargo.toml Cargo.lock -m "v{{VERSION}}"
    git tag "v{{VERSION}}"
    echo "Release tagged for version v{{VERSION}}"

# Build a mod archive for the Nexus.
[unix]
archive:
    #!/usr/bin/env bash
    set -e
    version=$(tomato get package.version Cargo.toml)
    release_name=version-swap_v${version}
    mkdir -p "releases/$release_name"
    cp -rp root/* "releases/${release_name}/"
    find "releases/${release_name}/" -name .keep -type f -delete
    cp -p target/release/version-swap.exe "releases/${release_name}/"
    cp -p target/release/version_swap.pdb "releases/${release_name}/"
    cd releases
    rm -f "$release_name".7z
    7z a "$release_name".7z "$release_name"
    rm -rf "$release_name"
    cd ..
    echo "Mod archive for v${version} ready at releases/${release_name}.7z"

# Remind you to run this in WSL.
[windows]
@archive:
	write-host "You need to run this in WSL to get bash."

[windows]
install: release
    cp target/release/version-swap.exe "G:\Vortex Mods\skyrimse\version-swap\root"
    cp target/release/version_swap.pdb "G:\Vortex Mods\skyrimse\version-swap\root"

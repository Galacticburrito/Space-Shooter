#!/bin/sh

# Get the project name from Cargo.toml
project_name=$(grep '^name = ' Cargo.toml | cut -d '"' -f 2)

# Construct the executable name
exe_name="$project_name.exe"

# Build the project for Windows
cargo build --target x86_64-pc-windows-gnu &&

# Copy the executable to the current directory
cp "target/x86_64-pc-windows-gnu/debug/$exe_name" . &&

# Execute the executable
exec "./$exe_name" "$@"

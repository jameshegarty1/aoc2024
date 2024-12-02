#!/bin/bash

day=$(printf "%02d" $1)

for part in a b; do
    dir="day${day}${part}"
    mkdir -p $dir/src
    touch $dir/{input,example,plan,puzzle.md}
    
    # Create minimal Cargo.toml
    echo "[package]
name = \"day${day}${part}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
atoi = { workspace = true }" > $dir/Cargo.toml

    # Create basic main.rs
    echo "fn main() {
    println!(\"Hello, ${dir}!\");
}" > $dir/src/main.rs
done

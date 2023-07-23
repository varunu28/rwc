# rwc
Rust implementation of Linux word count utility

Internally the CLI is built using the [clap crate](https://docs.rs/clap/latest/clap/index.html)

## How to build the project?
```
cargo build
alias rwc='./target/debug/rwc'
```

## Usage for rwc
```
// With no flags set and default output
rwc --name data.txt

// Setting flags
rwc --name data.txt -cwlb

// Setting flags with descriptive names
rwc --name data.txt --line --byte --word --character
```
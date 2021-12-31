# Rust Learning through The Book

I am using Windows so, installed Microsoft Visual Studio Build tool and Rust rustc 1.57.0 (f1edd0429 2021-11-29)

## Basic rule
- all lower case unless CONST
- to separate words, use _
- one space between ) { in function or macro

To read more about .toml, go to https://toml.io/en/
In Rust, packages of code are referred to as crates.

Cargo expects your source files to live inside the src directory. Top-level project directory is just for README files, license, config and others non-code related stuff.

cargo check ensure that it checks if the code will compile but not create executable.
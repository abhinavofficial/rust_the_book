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

##The guessing game

Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers.

How does crate ensure that each time same version of code is generated? This is done using cargo.lock. When you do want to update a crate, you can use `cargo update'. This will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. If that works, Cargo will write those versions to the Cargo.lock file.

**You won’t just know which traits to use and which methods and functions to call from a crate. Instructions for using a crate are in each crate’s documentation. Another neat feature of Cargo is that you can run the cargo doc --open command, which will build documentation provided by all of your dependencies locally and open it in your browser.**

To explicitly handle the possibility of Integer overflow, you can use these families of methods that the standard library provides on primitive numeric types:

Wrap in all modes with the wrapping_* methods, such as wrapping_add
Return the None value if there is overflow with the checked_* methods
Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
Saturate at the value’s minimum or maximum values with saturating_* methods
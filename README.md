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

> You won’t just know which traits to use and which methods and functions to call from a crate. Instructions for using a crate are in each crate’s documentation. Another neat feature of Cargo is that you can run the cargo doc --open command, which will build documentation provided by all of your dependencies locally and open it in your browser.

To explicitly handle the possibility of Integer overflow, you can use these families of methods that the standard library provides on primitive numeric types:

Wrap in all modes with the wrapping_* methods, such as wrapping_add
Return the None value if there is overflow with the checked_* methods
Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
Saturate at the value’s minimum or maximum values with saturating_* methods

## Using Struct

We can use the field init shorthand syntax to rewrite build_user function when the parameter names of the function and the struct field names are exactly the same.

Using struct update syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
The .. user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.
> Note that the struct update syntax is like assignment with = because it moves the data. In this example, we can no longer use user1 after creating user2. If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. The types of active and sign_in_count are types that implement the Copy trait.

You can also define structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit tuple. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

### Ownership of data
In the User struct definition, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
It’s possible for structs to store references to data owned by something else, but to do so, requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

### Using dbg! macro
If we annotate a struct with `#[derive(Debug)]`, we can get a lot of useful debug information. This would help understand what the code is doing at the runtime. See example in rectangle_area_calculator.
Read more about Attributes at ../doc/rust/html/reference/attributes.html and Derivable attribute at ../doc/rust/html/book/appendix-03-derivable-traits.html.

_Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior._

_Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:_
```p1.distance(&p2);
(&p1).distance(&p2);
```
_The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice._

## Using Struct
By default, HashMap uses a hashing function called SipHash (https://en.wikipedia.org/wiki/SipHash) that can provide resistance to Denial of Service (DoS) attacks involving hash tables1. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait.We don’t necessarily have to implement our own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

> To Learn
>- Module std::collections - C:/Users/abhin/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/std/collections/index.html
>- Struct std::vec::Vec - C:/Users/abhin/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/std/vec/struct.Vec.html
>- Enum std::option::Option - C:/Users/abhin/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/std/option/enum.Option.html
>- The Rust Reference - C:/Users/abhin/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/reference/introduction.html

> At the end (may be after 6 months or so) - writing unsafe Rust - C:/Users/abhin/.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/nomicon/intro.html
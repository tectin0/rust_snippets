//! In Rust `lib.rs` and `main.rs` are special files.
//! `lib.rs` is the root of your crate's library.
//! `main.rs` is the root of your crate's binary.
//! The are specifically TWO independent crates and can have two different names -> Cargo.toml.
//! Importing to the binary from the library functions the same as importing from any other crate.

use minimal_example::ANSWER;

fn main() {
    println!("Answer? {}!", ANSWER);
}

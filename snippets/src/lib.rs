//! For more information visit the official rust book:
//! https://doc.rust-lang.org/book
//!
//! You can also try out the Rust Book experiment with interactive quizzes:
//! https://rust-book.cs.brown.edu/

#[macro_use]
extern crate static_assertions;

mod casting;
mod errror_handling;
mod ffi;
mod functions;
mod generics;
mod lifetimes;
mod macros;
mod matching;
#[cfg(feature = "python")]
mod numpy;
mod print;
#[cfg(feature = "python")]
mod pyo3;
mod references;
mod safety;
mod traits;
mod variables;

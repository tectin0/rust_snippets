// Everything in Rust is by default private.
// The `pub` keyword is needed to make the constant visible to the binary.
// `pub(crate)` would make it visible to only the current crate. Which the binary is NOT part of.
pub const ANSWER: usize = 42;

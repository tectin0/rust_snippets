//! Printing in Rust is primarly done with the `println!` macro.
//! Alternatively you can use the `print!` macro to print without a newline.
//! The `dbg!` macro can be used to print the expression inside the macro with its value and location (file:line).

/// The `println!` macro is a variadic macro. It can take any number of arguments.
/// The first argument is a format string. The remaining arguments are the values to be formatted.
/// The standard documentation has very good examples on how to use the `println!` macro.
#[test]
fn println() {
    println!("Hello, world!");

    let value = 0.2f32.sqrt();

    println!("format {:.2} arguments", value); // print with decimal precision

    println!("format {:b} arguments", (value * 100.0) as i32); // print as binary

    // More Formatting Examples: https://doc.rust-lang.org/std/fmt/

    // The `print!` macro works the same way as `println!` but without a newline.
    print!("Hello, ");
    print!("world!\n");
}

/// The standard library `print` macros are useful and sufficient for smaller programs.
/// For larger programs you might want to use a logging framework.
/// The most popular logging "facade" is [log](https://crates.io/crates/log).
/// There are multiple loggers to choose from that implement the `log` facade.
/// A popular logger is [simple_logger](https://crates.io/crates/simple_logger).
#[test]
fn logger() {
    simple_logger::init_with_level(log::Level::Debug).expect("Failed to initialize logger"); // initialize the logger with the debug level

    log::debug!("This is a debug message");
}

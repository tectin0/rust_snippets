// Matching is a way to compare a value against a series of patterns and conditionally execute code based on the pattern that matches.
// Patterns can be made up of literal values, variable names, wildcards, and many other things.

use std::fmt::write;

/// Enums are a way to define Variants of a type
/// They are the typical way type to match against
#[test]
fn match_enum() {
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Red;

    let color_name = match color {
        Color::Red => "red",
        Color::Green => "green",
        Color::Blue => "blue",
    };

    assert_eq!(color_name, "red");
}

/// Option<T> is a type that represents either Some(T) or None
/// It is defined as an Enum in the standard library
/// It is used to express the possible absence of a value
#[test]
fn match_option() {
    let x: Option<i32> = Some(1);

    let y = match x {
        Some(x) => x,
        None => 0,
    };

    assert_eq!(y, 1);
}

/// Result<T, E> is a type that represents either Ok(T) or Err(E)
/// It is defined as an Enum in the standard library
/// It is used to express the possible failure of an operation
/// Functionally Result<T, E> and Option<T> are very similar
/// Result<T, E> is used to express the possible failure of an operation in multiple ways
/// Option<T> is used to express the possible failure of an operation in one specific way (the absence of a value)
#[test]
fn match_result() {
    let x: Result<i32, &str> = Ok(1);

    let y = match x {
        Ok(x) => x,
        Err(_) => 0,
    };

    assert_eq!(y, 1);
}

#[test]
fn match_result_option() {
    use std::io::BufRead;

    // this can fail in multiple ways -> Error
    let file = match std::fs::File::open("does_not_exist.txt") {
        Ok(file) => file,

        Err(_) => {
            // this can fail in multiple ways -> Error
            // `unwrap()` is a shortcut for match
            std::fs::File::create("does_not_exist.txt").unwrap()
        }
    };

    let line = std::io::BufReader::new(file).lines().nth(0); // `Option<Result<String, Error>>`

    // this can fail in multiple ways -> Error
    std::fs::remove_file("does_not_exist.txt").unwrap();

    // the line can either exist or not -> Option
    let line = match line {
        Some(line) => line,
        None => Ok("".to_string()),
    };

    // the reading of the line can fail in multiple ways -> Result
    let line = match line {
        Ok(line) => line,
        Err(_) => "".to_string(),
    };

    assert_eq!(line, "");
}

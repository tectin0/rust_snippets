// Rust functions are denoted by fn with `()` round brackets
// for arguments and curly brackets for the body of the function.
// `->` is used to denote the return type of the function.

// The datatype of the arguments has to be explicitly specified (i32, i64, f32, f64, str, Vec<_> ..)
// The return type has to be explicitly specified except for the unit type `()`.

#[test]
fn function() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    assert_eq!(add(1, 2), 3);
}

/// A closure is a function that can capture the environment in which it is defined. Think `lambda` functions in Python / C++.
/// They are anonymous functions without a name.
#[test]
fn closure() {
    let add = |a: i32, b: i32| -> i32 { a + b };

    assert_eq!(add(1, 2), 3);

    let mut a = 1;

    let mut add_to_a = |b: i32| {
        a += b;
    };

    add_to_a(2);
    add_to_a(10);

    assert_eq!(a, 13);
}

/// A function (pointer) can also be assigned to a variable.
#[test]
fn function_pointer() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let add_ptr = add;

    assert_eq!(add_ptr(1, 2), 3);
}

/// A function can also be passed as an argument to another function.
#[test]
fn function_argument() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn apply<F>(f: F, a: i32, b: i32) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        f(a, b)
    }

    assert_eq!(apply(add, 1, 2), 3);
}



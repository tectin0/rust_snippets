// Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
// A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters
// Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type.
// A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.

// The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions.

// Another important difference between macros and functions is that you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

/// The syntax `macro_rules! name_of_macro` starts the implementation of a macro available by the name `name_of_macro`
/// and `#[macro_export]` makes this macro available to the entire crate
#[macro_export]
macro_rules! point {
    // macros match patterns
    ($x:expr, $y:expr) => {{
        let mut vec: Vec<i32> = Vec::new();
        vec.push($x);
        vec.push($y);
        vec
    }};
    ($x:expr, $y:expr, $z:expr) => {{
        let mut vec: Vec<i32> = Vec::new();
        vec.push($x);
        vec.push($y);
        vec.push($z);
        vec
    }};
    ($x:expr, $y:expr, $z:expr, $t:ty) => {{
        let mut vec: Vec<$t> = Vec::new();
        vec.push($x as $t);
        vec.push($y as $t);
        vec.push($z as $t);
        vec
    }};
    ([$($x:expr),+]: $t:ty) => {{
        let mut vec: Vec<$t> = Vec::new();
        $(
            vec.push($x as $t);
        )*

        vec
    }};
}

#[test]
pub fn test_point_macro() {
    assert_eq!(point!(1, 1), vec![1, 1]);
    assert_eq!(point!(1, 1, 1), vec![1, 1, 1]);
    assert_eq!(point!(1, 1, 1, f32), vec![1.0, 1.0, 1.0]);
    assert_eq!(point!([1, 1, 1, 1, 2]: f32), vec![1.0, 1.0, 1.0, 1.0, 2.0]);
    //println!("{:?}", point!(1, 1, 1, 1));  // 4th argument does not match any rules!
    //println!("{:?}", point!(1.0, 1.0, 1.0));  // wrong input types!
}

#[macro_export]
macro_rules! pyprint {
    ($($arg:tt)*) => {{
        let code = format!($($arg)*);
        pyo3::Python::with_gil(|py| {
            py.eval(&format!("print(\"{code}\")"), None, None).expect("print failed!");
        });
    }};
}

#[test]
pub fn use_pyprint() {
    pyo3::prepare_freethreaded_python();

    pyprint!("hello");
}

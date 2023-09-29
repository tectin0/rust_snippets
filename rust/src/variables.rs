// Variables in Rust are defined with the `let` keyword and require an explicit type annotation if it can't be deterministically inferred.

/// Mutability is a property of variables that determines whether they can be changed after they have been defined.
/// Variables are immutable by default. To make them mutable the `mut` keyword has to be used.
#[test]
fn mutability() {
    let x = 1; // immutable variable
    let mut y = 2; // mutable variable

    y += 1;

    assert_eq!(x, 1);
    assert_eq!(y, 3);
}

/// Type inference is the process of determining the type of an expression from the context.
#[test]
fn type_inference() {
    let vec = vec![1, 2, 3]; // type inference works for vectors if the type can be inferred from the context

    assert_eq!(vec, [1, 2, 3]);

    let vec: Vec<u32> = Vec::new(); // type annotation is required if the type cannot be inferred - `Vec::new()` can be any type.

    assert_eq!(vec, []);

    let vec = Vec::<u32>::new(); // type annotation can also be specified with the turbofish operator

    assert_eq!(vec, []);

    let mut vec = Vec::new(); // the type of the vector can be inferred from the type of the elements that are pushed into it
    vec.push(1.0);

    assert_eq!(vec, [1.0]);

    let a = 1; // the type of a is inferred from later context
    let b = 2u32; // the type of b is explicitly specified

    let c = a + b; // <- a and b have to be of the same type -> a has to be u32

    assert_eq!(c, 3);
}

/// Shadowing is the process of redefining a variable in a nested scope.
/// Rust allows shadowing of variables.
#[test]
fn shadowing() {
    let a = 1;

    {
        let a = 2; // a is shadowed in this scope
        assert_eq!(a, 2);
    }

    assert_eq!(a, 1);
}

/// An expressions is a piece of code that can be evaluated to a value.
#[test]
fn expression() {
    // A block is an expression
    assert_eq!(
        {
            let a = 1;
            let b = 2;
            a + b // the last expression in a block is the return value of the block
        },
        3
    );

    // The value of an expression can be assigned to a variable
    let a = {
        let a = 1;
        let b = 2;
        a + b
    };

    assert_eq!(a, 3);
}

/// Constant expressions are evaluated at compile time.
#[test]
fn constant_expression() {
    // literals are constant expressions (numbers, characters, strings, boolean values)
    const_assert_eq!(2, 2); // const_assert_eq! is a macro that checks the equality of two expressions at compile time -> static_assertions crate

    let a = 1;
    // const_assert_eq!(k, 2); // Error: let bindings are not constant expressions
    assert_eq!(a, 1);

    const A: i32 = 1; // const bindings are constant expressions, always require a type annotation and should be upper case
    const_assert_eq!(A, 1);

    const_assert_eq!('a', 'a');
    const_assert_eq!(true, true);

    const S1: &str = "hello"; // string literals are constant expressions
    const S2: &str = "hello";

    // const B: bool = S1 == S2; // string comparison is not a constant expression ? -> https://internals.rust-lang.org/t/why-i-cannot-compare-two-static-str-s-in-a-const-context/17726/3

    const BS1: &[u8] = b"hello"; // byte string literals are constant expressions
    const BS2: &[u8] = b"hellp";

    const B: bool = {
        let mut i = 0;
        let mut j = 0;

        while i < BS1.len() && j < BS2.len() {
            // looping is a constant expression
            // indexing is a constant expression
            if BS1[i] != BS2[j] {
                false;
                break;
            }

            i += 1;
            j += 1;
        }

        i == BS1.len() && j == BS2.len()
    }; // blocks are constant expressions if the last expression is a constant expression

    const_assert!(!B);

    // const_assert_eq!("hello", "hello"); // Error: string literals can't be used in const_assert_eq! ?

    const V: [u8; 4] = [1, 2, 3, 1]; // array expressions are constant expressions
    const B2: bool = V[0] == V[3]; // indexing is a constant expression
    const_assert!(B2);

    const T: (u8, bool) = (1, true); // tuple expressions are constant expressions
    const_assert_eq!(T.0, 1); // field access is a constant expression

    const_assert_eq!({ 1 }, { 1 }); // blocks are constant expressions

    const_assert_eq!(2 as i32, 2); // type casting is a constant expression

    struct Point {
        x: i32,
    }

    const P: Point = Point { x: 1 }; // struct expressions are constant expressions

    const_assert_eq!(P.x, 1); // field access is a constant expression

    const D: &i32 = &1;

    const_assert_eq!(*D, 1); // dereferencing is a constant expression

    enum Choice {
        A,
        B,
    }

    const C: Choice = Choice::A; // enum expressions are constant expressions

    const_assert!({
        match C {
            Choice::A => true,
            Choice::B => false,
        }
    }); // match expressions are constant expressions

    const CL: &'static dyn Fn(u32) -> u32 = &|x: u32| x + 1; // closures are constant expressions

    // const_assert_eq!(CL(1), 2); // function call expressions are not constant expressions ?
    assert_eq!(CL(1), 2);

    const RG: std::ops::Range<u32> = 1..2; // range expressions are constant expressions

    const_assert_eq!(RG.start, 1); // field access is a constant expression
}

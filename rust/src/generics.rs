//! Generics are a way to loosen the type restrictions of a function or type without losing type safety.
//!
//! In addition to `Traits` sharing functionality between types, they are used to define what functionality a generic type has to implement to be used in a generic function or type.

#[test]
fn generics() {
    use std::ops::Add; // standard library trait for addition `+`

    // This function can be used with any type that implements the `Add` trait
    fn add<T: Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    assert_eq!(add(1, 2), 3); // `i32` implements `Add`
    assert_eq!(add(1.0, 2.0), 3.0); // `f32` implements `Add`
    assert_eq!(add(1usize, 2usize), 3usize); // `usize` implements `Add`

    use std::fmt::Display; // standard library trait for displaying a type

    // The `where` keyword is an alternative syntax for specifying trait bounds
    fn get_display_representation<T>(a: T) -> String
    where
        T: Display, // `T` has to implement `Display`
    {
        format!("{a}") // `format!` requires the type to implement `Display`
    }

    let s = get_display_representation(1);
    assert_eq!(s, "1");

    enum Dead {
        Yes,
        No,
    }

    impl Display for Dead {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Dead::Yes => write!(f, "Yes"),
                Dead::No => write!(f, "No"),
            }
        }
    }

    let t = get_display_representation(Dead::Yes);

    assert_eq!(t, "Yes");
}

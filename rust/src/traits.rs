//! https://doc.rust-lang.org/book/ch10-02-traits.html
//!
//! Traits are a way to share functionality between different types.
//! They are implemented with the `impl` keyword.
//! They can be implemented for any type.
//! There are certain rules that have to be followed when implementing traits for types outside of the crate -> Orphan Rules

#[test]
fn traits() {
    use std::ops::Add; // standard library trait for addition `+`

    use core::cmp::PartialEq; // standard library trait for equality `==`

    use std::fmt::Debug; // standard library trait for debug printing `{:?}`

    struct Point(f32, f32); // define a New Type `Point` with two fields of type `f32`

    // `type Point = (f32, f32);` would be a type alias for a tuple of two `f32` not an actual new type

    // Implementing the `Add` trait for the `Point` type
    // The trait defines what functions have to be implemented for the type and how they have to be implemented
    impl Add for Point {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self(self.0 + other.0, self.1 + other.1)
        }
    }

    let p1 = Point(1.0, 2.0);
    let p2 = Point(3.0, 4.0);

    let p3 = p1 + p2;

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }

    impl Debug for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}", self.0, self.1)
        }
    }

    assert_eq!(p3, Point(4.0, 6.0));
}

/// https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
/// Traits can be implemented through the `derive` attribute if the trait has a derive macro implemented for it.
/// This requires that all fields of the type implement the trait as well.
#[test]
fn derive() {
    use core::cmp::PartialEq;
    use std::fmt::Debug;

    #[derive(Debug, PartialEq)] // `Add` does not have a derive macro implemented for it
    struct Point(f32, f32); // `f32` implements `PartialEq` and `Debug`

    let p1 = Point(1.0, 2.0);

    assert_eq!(p1, Point(1.0, 2.0));
}

/// Custom traits are defined with the `trait` keyword.
/// They can be implemented for any type.
#[test]
fn custom_traits() {
    trait Molecule {
        // the trait defines the function signature
        fn name(&self) -> String;
        fn weight(&self) -> f32;
        fn is_organic(&self) -> bool;
        // the trait can also define default implementations
        fn is_inorganic(&self) -> bool {
            !self.is_organic()
        }
    }

    struct Water;

    // custom traits are implemented the same way as standard library traits
    impl Molecule for Water {
        fn name(&self) -> String {
            "Water".to_string()
        }

        fn weight(&self) -> f32 {
            18.015
        }

        fn is_organic(&self) -> bool {
            false
        }
    }

    struct Methane;

    impl Molecule for Methane {
        fn name(&self) -> String {
            "Methane".to_string()
        }

        fn weight(&self) -> f32 {
            16.043
        }

        fn is_organic(&self) -> bool {
            true
        }
    }

    fn add_atomic_weight<T: Molecule, U: Molecule>(a: T, b: U) -> f32 {
        a.weight() + b.weight()
    }

    let water = Water;
    let methane = Methane;

    let atomic_weight = add_atomic_weight(water, methane);

    assert_eq!(atomic_weight, 34.058);
}

// Traits are often used as marker traits to denote certain properties of a type.
#[test]
fn marker_traits() {
    trait Metallic {}

    struct Iron;

    impl Metallic for Iron {}

    struct Gold;

    impl Metallic for Gold {}

    // `Box<dyn T>` is necessary because the size of `T` is not known at compile time
    // `T` could be any type that implements `Metallic`
    let mut list_of_metals: Vec<Box<dyn Metallic>> = Vec::new();

    list_of_metals.push(Box::new(Iron));
    list_of_metals.push(Box::new(Gold));

    use std::any::Any; // standard library trait for type checking
    use std::ops::Deref; // standard library trait for dereferencing

    assert_eq!(
        list_of_metals[0].deref().type_id(),
        std::any::TypeId::of::<dyn Metallic>()
    );
}

#[test]
fn traits_and_generics() {
    struct Animal {
        name: String,
        age: u32,
    }

    impl Add for Animal {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                name: format!("{}{}", self.name, other.name),
                age: self.age + other.age,
            }
        }
    }

    use std::ops::Add;

    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    let cat = Animal {
        name: "Cat".to_string(),
        age: 1,
    };

    let turtle = Animal {
        name: "Turtle".to_string(),
        age: 120,
    };

    let cat_turtle = add(cat, turtle);

    assert_eq!(cat_turtle.name, "CatTurtle");
    assert_eq!(cat_turtle.age, 121);
}

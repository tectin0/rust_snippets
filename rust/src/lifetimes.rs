// https://stackoverflow.com/questions/31609137/why-are-explicit-lifetimes-needed-in-rust
// If a function receives two references as arguments and returns a reference, then the implementation of the function
// might sometimes return the first reference and sometimes the second one.
// It is impossible to predict which reference will be returned for a given call.
//  Explicit lifetimes help to avoid or clarify such a situation.

/// With explicit lifetimes the compiler knows that the result of greater has the same lifetime as the first argument ('a)
/// and can outlive the second argument. If y had the same lifetime the compiler would complain that y does not live long enough.
#[test]
pub fn epxlicit_lifetimes() {
    fn greater<'a, 'b>(x: &'a u32, y: &'b u32) -> &'a u32 {
        if x > y {
            x
        } else {
            &0
        }
    }

    let x = 1; // x is dropped at the end of the function

    let z: &u32 = {
        let y = 2;
        greater(&x, &y)
    }; // y is dropped here

    assert_eq!(z, &0); // a reference to y would not live long enough
}

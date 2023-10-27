//! Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword `as` or `from`.

#[test]
fn casting() {
    let a: i32 = 0;
    let b: i64 = a as i64 + 1;

    assert_eq!(b, 1i64);

    let c = a + i32::try_from(b).unwrap(); // panics on overflow

    assert_eq!(c, 1i32);

    let c = a.saturating_add(b.try_into().unwrap()); // panics on overflow converting to i32 but not on overflow adding

    assert_eq!(c, 1i32);

    let a = i64::MAX;

    let b = a as i32; // silent overflow

    assert_eq!(b, -1i32);

    let a = -1;

    let b = a as u32; // silent overflow

    assert_eq!(b, u32::MAX);
}

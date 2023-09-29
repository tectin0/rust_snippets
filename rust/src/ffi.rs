// The foreign function interface (FFI) is the boundary between Rust and other languages. This is how you can call C functions from Rust and vice versa.
// The `cc` crate is an easy way to compile C code into a static library and link it to Rust during the rust build process (see build.rs).
// Calling C functions from Rust are inherently unsafe, because Rust can't guarantee that the C code is safe.

#[test]
fn extern_c() {
    extern "C" {
        fn addc(a: i32, b: i32) -> i32;
    }

    unsafe {
        let b = addc(1, 2);
        assert_eq!(b, 3);
    }
}

fn main() {
    #[cfg(feature = "cc")]
    cc::Build::new().file("../c/src/ffi.c").compile("ffi");
}

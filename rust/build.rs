fn main() {
    cc::Build::new().file("../c/src/ffi.c").compile("ffi");
}

/// https://doc.rust-lang.org/nomicon/ownership.html
/// Rusts ownership system allows for memory safety without garbage collection.
/// In C you can write invalid code and have it compile without any warnings or errors.

/// The C compiler does not check for uninitialized variables and lets you use them. This can lead to undefined behavior.
/// Rust will not compile if you try to use an uninitialized variable (and also has useful warnings and potential fixes for the unused variables).
#[test]
fn test_c_uninitialized() {
    /* uninitialized.c

    #include <stdio.h>

    int main() {
        int num; // uninitialized variable

        printf("The value of num is: %d\n", num); // using uninitialized variable

        return 0;
    }

    */

    cc::Build::new()
        .file("../c/src/uninitialized.c")
        .target("x86_64-pc-windows-msvc")
        .host("x86_64-pc-windows-msvc")
        .opt_level(0)
        .compile("unsafe");

    let num: i32; // uninitialized variable

    // println!("The value of num is: {}", num); -> compile error: used binding `num` isn't initialized
}

/// A common mistake in C is to create dangling pointersw hcih point to invalid memory.
/// In this case a function creates an array of integers and returns a pointer of the first element of the array.
/// The array is deallocated when the function returns and the pointer is now dangling.
/// The dangling pointer can them be used by another function which leads to undefined behavior.
/// The Rust compiler will not let you create dangling pointers. The Ownership system ensures that the memory is not deallocated until the last owner goes out of scope.
#[test]
fn test_c_dangling_pointer() {
    /* dangling_pointer.c

     #include <stdio.h>


    int* getArray() {

        int array[3] = {1, 2, 3};

        return array;

    }


    void printArray(int* arr) {

        for (int i = 0; i < 3; i++) {

            printf("%d ", arr[i]);

        }

        printf("\n");

    }


    int main() {

        int* danglingPointer = getArray();

        printArray(danglingPointer); // Using dangling pointer


        return 0;

    }

    */

    cc::Build::new()
        .file("../c/src/dangling_pointer.c")
        .target("x86_64-pc-windows-msvc")
        .host("x86_64-pc-windows-msvc")
        .opt_level(0)
        .compile("unsafe");

    // The following function is in no way possible to compile in Rust.
    // The Ownership system completely prevents this error from happening.

    // Error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // consider using the `'static` lifetime: `'static `

    // fn get_array() -> &i32 {
    //     let array = [1, 2, 3];

    //     return &array[0];
    // }

    // You can however use the const keyword to create a static array.
    // The static array is not deallocated when the function returns and the reference is valid.

    fn get_array() -> &'static i32 {
        const ARRAY: [i32; 3] = [1, 2, 3];

        return &ARRAY[0];
    }

    assert!(get_array() == &1);
}

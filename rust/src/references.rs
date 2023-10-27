//! Rust's entire borrowing model enforces one simple requirement: the contents of a memory location can only be mutated if there is only one pointer through which that location can be accessed.
//! This model enables the compiler to perform aggressive optimisations

#[test]
pub fn reference() {
    let mut x: i32 = 1;

    let y: &i32 = &x;

    println!("{:?}", y);

    let y = *y;

    x += 1;

    println!("{:?}", y);
    println!("{:?}", x);
}

#[test]
#[you_can::turn_off_the_borrow_checker]
pub fn borrow_checker_fine() {
    let mut x = Box::new(1);
    let y: &i32 = &x; // <- borrowed here

    *x += 1; // can't assign to x because it is still borrowed

    println!("{x}");

    assert_eq!(*y, 2); // <- borrow used
}

/// Arc<Mutex<T>> is a thread-safe reference-counted pointer to a mutable value protected by a mutex.
/// This is useful when you want to share a mutable value between multiple threads.
#[test]
fn borrow_checker_arc_mutex() {
    let x = std::sync::Arc::new(std::sync::Mutex::new(1)); // <- x is a shared resource
    let y = &x;
    let z = &x; // You can make any number of immutable references

    *x.lock().unwrap() += 1; // Safely mutate the value inside the mutex because we don't borrow the value directly but the Arc<Mutex<T>>.

    assert_eq!(*y.lock().unwrap(), 2);

    *z.lock().unwrap() += 1;

    assert_eq!(*y.lock().unwrap(), 3);

    let x2 = x.clone();

    // this also works when spawning threads
    std::thread::spawn(move || {
        *x2.lock().unwrap() += 1; // the lock guarantees that only one thread can access the value at a time
    });

    let x3 = x.clone();

    std::thread::spawn(move || {
        *x3.lock().unwrap() += 1;
    });

    std::thread::sleep(core::time::Duration::from_secs(1));

    assert_eq!(*x.lock().unwrap(), 5); // assuming the threads have finished
}

#[test]
#[you_can::turn_off_the_borrow_checker]
fn borrow_checker_undefined() {
    let mut v: Vec<u32> = vec![1, 2, 3];
    let r1 = &v[2];

    dbg!(r1, v.get(2)); // 3 and 3 ✓

    v.pop(); // Error: Cannot borrow `v` as mutable because it is also borrowed as immutable

    let mut v: Vec<i32> = v.into_iter().map(|x| x as i32).collect();

    v.push(-10);

    assert_eq!(v, vec![1, 2, -10]);

    dbg!(r1, v.get(2)); // 4294967286 and -10 X ?

    v.pop();

    assert_eq!(v, vec![1, 2]);

    dbg!(r1, v.get(2)); // 4294967286 and None X ?

    let mut v: Vec<String> = v.into_iter().map(|x| x.to_string()).collect();

    v.push("hello".to_string());

    let r2 = &v[2];

    assert_eq!(v, vec!["1", "2", "hello"]);

    dbg!(r1, v.get(2)); // 4294967286 and Some("hello") X ?

    v.pop();

    let mut v: Vec<i32> = v.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    v.push(3);

    assert_eq!(v, vec![1, 2, 3]);

    dbg!(r1, r2, v.get(2)); // 4294967286 and Some("hello") and Some(3) X ?
}

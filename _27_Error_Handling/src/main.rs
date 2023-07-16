use std::panic;

fn main() {
    // 27.1 Panics
    let v = vec![10, 20, 30];
    // println!("v[100]: {}", v[100]);

    // Panics are for unrecoverable and unexpected errors.
    // Panics are symptoms of bugs in the program.
    // Use non-panicking APIs(such as Vec::get) if crashing is not acceptable


    

    // 27.1.1 Catching and Stack Unwinding
    let result = panic::catch_unwind(|| {
        println!("hello");
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });

    assert!(result.is_err());

    // This can be useful in servers which should keep running even if a single request crashes.
    // This does not work if panic = 'abort' is set in your Cargo.toml.
}

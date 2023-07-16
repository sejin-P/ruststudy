use std::panic;
use std::fs;
use std::io::Read;

fn main() {
    // 27.1 Panics
    let v = vec![10, 20, 30];
    // println!("v[100]: {}", v[100]);

    // Panics are for unrecoverable and unexpected errors.
    // Panics are symptoms of bugs in the program.
    // Use non-panicking APIs(such as Vec::get) if crashing is not acceptable


    

    // 27.1.1 Catching and Stack Unwinding
    // By default, a panic will cause the stack to unwind. The unwinding can be caught:
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










    // 27.2 Structured Error Handling With Result'
    // We have already seen the Result enum. This is used pervasively when errors are expected as part of normal operation:
    let file = fs::File::open("dairy.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            println!("Dear diary: {contents}");
        },
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }


    // As with Option, the successful value sits inside of Result, forcing the developer to explicitly extract it. This encourages error checking. In the case where an error should never happen, unwrap() or expect() can be called, and this is a signal of the developer intent too.
    // Result documentation is a recommended read. Not during the course, but it is worth mentioning. It contains a lot of convenience methods and functions that help functional-style programming.



}

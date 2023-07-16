use std::error::Error;
use std::panic;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};
use thiserror::Error;

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






















    // 27.3 Propagating Errors with `?`

    // The try-operator ? is used to return errors to the caller. It lets you turn the common
    // match some_expression {
    //     Ok(value) => value,
    //     Err(err) =? return Err(err),
    // }

    // into the much simpler
    // some_expression?


    fn read_username(path: &str) -> Result<String, io::Error> {
        let username_file_result = fs::File::open(path);
        let mut uesrname_file = match username_file_result {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        let mut username = String::new();
        match uesrname_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(err) => Err(err),
        }
    }

    let username = read_username("config.dat");
    println!("username or err: {username:?}");














    // 27.3.1.1 Converting Errot Types - Example
    #[derive(Debug)]
    enum ReadUsernameError {
        IoError(io::Error),
        EmptyUsername(String),
    }

    impl Error for ReadUsernameError {}

    impl Display for ReadUsernameError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                Self::IoError(e) => write!(f, "Io error: {e}"),
                Self::EmptyUsername(filename) => write!(f, "Found no username in {filename}")
            }
        }
    }

    impl From<io::Error> for ReadUsernameError {
        fn from(err: io::Error) -> ReadUsernameError {
            ReadUsernameError::IoError(err)
        }
    }

    fn read_username1(path: &str) -> Result<String, ReadUsernameError> {
        let mut username = String::with_capacity(100);
        File::open(path)?.read_to_string(&mut username)?;
        if username.is_empty() {
            return Err(ReadUsernameError::EmptyUsername(String::from(path)));
        }
        Ok(username)
    }
    let username = read_username1("config.dat");
    println!("username or error: {username:?}");

    // Key points:

    // The username variable can be either Ok(string) or Err(error).
    // Use the fs::write call to test out the different scenarios: no file, empty file, file with username.
    // It is good practice for all error types to implement std::error::Error, which requires Debug and Display.
    // It’s generally helpful for them to implement Clone and Eq too where possible, to make life easier for tests and consumers of your library.
    // In this case we can’t easily do so, because io::Error doesn’t implement them.














    // 27.3.2 Deriving Error Enums
    // The `thiserror` crate is a popular way to create an error enum like we did on the previous page
    #[derive(Debug, Error)]
    enum ReadUsernameError1 {
        #[error("Could not read: {0}")]
        IoError(#[from] io::Error),
        #[error("Found no username in {0}")]
        EmptyUsername(String),
    }

    fn read_username2(path: &str) -> Result<String, ReadUsernameError1> {
        let mut username = String::new();
        File::open(path)?.read_to_string(&mut username)?;
        if username.is_empty() {
            return Err(ReadUsernameError1::EmptyUsername(String::from(path)));
        }
        Ok(username)
    }

    match read_username2("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err}"),
    }

    // thiserror’s derive macro automatically implements std::error::Error,
    // and optionally Display (if the #[error(...)] attributes are provided) and From (if the #[from] attribute is added).
    // It also works for structs.
    //
    // It doesn’t affect your public API, which makes it good for libraries.
}

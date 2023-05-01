struct Point(i32, i32);

fn main() {
    // 10.0 Ownership
    // All variable bindings have a scope where they are valid and it is an error to use a variable outside its scope
    // At the end of the scope, the variable is dropped and the data is freed.
    // A destructor can run here to free up resources.
    // We say that the variable owns the value.
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }

    // println!("y: {}", p.1);







    // 10.1 Move Semantics
    // The assignment of s1 to s2 transfers ownership.
    // The data was moved from s1 and s1 is no longer accessible.
    // When s1 goes out of scope, nothing happens: it has no ownership.
    // When s2 goes out of scope, the string data is freed.
    // There is always exactly one variable binding which owns a value.
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");




    // 10.2 Moved Strings in Rust
    // Before move to s2

    // stack           heap
    // s1
    // ptr     -->     "Rust"
    // len 4
    // cap 4

    // After move to s2

    // stack           heap
    // s1(inaccessible)
    // ptr     ---->   "Rust"
    // len 4     |
    // cap 4     |
    //           |
    //           |
    // s2        |
    // ptr     ---
    // len 4
    // cap 4
    let s1: String = String::from("Rust");
    let s2: String = s1;








    // 10.3 Moves in function calls
    // With the first call to say_hello, main gives up ownership of name. Afterwards, name cannot be used anymore within main.
    // The heap memory allocated for name will be freed at the end of the say_hello function.
    // main can retain ownership if it passes name as a reference (&name) and if say_hello accepts a reference as a parameter.
    // Alternatively, main can pass a clone of name in the first call (name.clone()).
    // Rust makes it harder than C++ to inadvertently create copies by making move semantics the default, and by forcing programmers to make clones explicit.
    fn say_hello(name: String) {
        println!("Hello {name}")
    }

    let name = String::from("Alice");
    say_hello(name);
}

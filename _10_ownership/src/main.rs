struct Point(i32, i32);

fn main() {
    // All variable bindings have a scope where they are valid and it is an error to use a variable outside its scope
    // At the end of the scope, the variable is dropped and the data is freed.
    // A destructor can run here to free up resources.
    // We say that the variable owns the value.
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }

    println!("y: {}", p.1);
}

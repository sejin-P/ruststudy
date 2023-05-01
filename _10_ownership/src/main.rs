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

    // println!("y: {}", p.1);







    // The assignment of s1 to s2 transfers ownership.
    // The data was moved from s1 and s1 is no longer accessible.
    // When s1 goes out of scope, nothing happens: it has no ownership.
    // When s2 goes out of scope, the string data is freed.
    // There is always exactly one variable binding which owns a value.
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");


}

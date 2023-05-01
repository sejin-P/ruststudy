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









    // 10.4 Copying and Cloning
    // While move semantics are the default, certain types are copied by default.
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");

    // derive is sufficient to say that this is a way to generate code in Rust at compile time. In this case the default implementations of Copy and Clone traits are generated.
    #[derive(Copy, Clone, Debug)]
    struct Point(i32, i32);

    let p1 = Point(1, 2);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
    // Copying and cloning are not the same thing.
    // Copying refers to bitwise copies of memory regions and does not work on arbitrary objects.
    // Copying does not allow for custom logic (unlike copy constructors in C++).
    // Cloning is a more general operation and also allows for custom behavior by implementing the Clone trait.
    // Copying does not work on types that implement the Drop trait.











    // 10.5 Borrowing
    // Instead of transferring ownership when calling a function, you can let a function borrow the value:
    #[derive(Debug)]
    struct Point2(i32, i32);
    fn add(p1: &Point2, p2: &Point2) -> Point2 {
        let p = Point2(p1.0+p2.0, p1.1+p2.1);
        println!("&p.0: {:p}", &p.0);
        p
    }

    let p1 = Point2(1, 2);
    let p2 = Point2(5, 6);
    let p3 = add(&p1, &p2);
    println!("&p3.0: {:p}", &p3.0);
    println!("{p1:?} + {p2:?} = {p3:?}");
    // The add function borrows two points and returns a new point.
    // The caller retains ownership of the inputs.
    // Demonstrate that the return from add is cheap because the compiler can eliminate the copy operation.
    // The Rust compiler can do return value optimization (RVO).
}

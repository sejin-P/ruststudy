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










    // 10.5.1 Shared and Unique Borrows
    // Constraints on borrowing
    // You can have one or more &T values at any given time, or
    // You can have exactly one &mut T value.

    let mut a: i32 = 10;
    let b: &i32 = &a;
    // println!("b: {b}");

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    // println!("b:  {b}");
    //The above code does not compile because a is borrowed as mutable (through c) and as immutable (through b) at the same time.
    // Move the println! statement for b before the scope that introduces c to make the code compile.
    // After that change, the compiler realizes that b is only ever used before the new mutable borrow of a through c. This is a feature of the borrow checker called “non-lexical lifetimes”.








    // 10.7 Lifetimes in function calls
    #[derive(Debug)]
    struct Point3(i32, i32);

    fn left_most<'a>(p1: &'a Point3, p2: &'a Point3) -> &'a Point3 {
        if p1.0 < p2.0 { p1 } else { p2 }
    }

    let p1: Point3 = Point3(10, 10);
    let p2: Point3 = Point3(20, 20);
    let p3: &Point3 = left_most(&p1, &p2);
    println!("left_most point: {:?}", p3);

    // 'a is a generic parameter, it is inferred by the compiler.
    // Lifetimes start with ' and 'a is a typical default name.
    // Read &'a Point as “a borrowed Point which is valid for at least the lifetime a”.
    // The at least part is important when parameters are in different scopes.

    // chatgpt4 qna
    // Q: Then if I assign p1 and p2 just in function call, I cannot use the return value?
    //
    // ChatGPT
    // Yes, if you pass in references to Point3 objects that are created in the function call itself, then the returned reference from left_most will be dangling as soon as the function call ends because the objects will be dropped when the function call is over. This is because the lifetime of those objects is limited to the function call, and Rust's borrow checker will not allow you to keep references to them outside of that scope.
    //
    //     For example, this would cause a compile-time error:
    //
    //     rust
    // Copy code
    // let point = left_most(&Point3::new(...), &Point3::new(...));
    // This will not compile because the temporary Point3 objects are dropped as soon as the left_most call is over, so point would be a dangling reference.
    //
    //     However, if you keep Point3 objects in a scope that outlives the usage of point, then it is okay. For example:
    //
    //     rust
    // Copy code
    // let p1 = Point3::new(...);
    // let p2 = Point3::new(...);
    // let point = left_most(&p1, &p2);
    // In this case, p1 and p2 live at least as long as point, so there is no problem.


}

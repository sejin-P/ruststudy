fn main() {
    // 23.1 Generic Data Types
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10};
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");















}

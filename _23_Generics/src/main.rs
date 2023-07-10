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














    // 23.2 Generic Methods
    #[derive(Debug)]
    struct Point1<T>(T, T);

    impl<T> Point1<T> {
        fn x(&self) -> &T {
            &self.0
        }

        fn set_x(&mut self, x:T) {
            
        }
    }

    let p = Point1(5, 10);
    println!("p.x = {}", p.x());

    // Q: Why T is specified twice in impl<T> Point<T> {}? Isn’t that redundant?
    // This is because it is a generic implementation section for generic type. They are independently generic.
    // It means these methods are defined for any T.
    // It is possible to write impl Point<u32> { .. }.
    // Point is still generic and you can use Point<f64>, but methods in this block will only be available for Point<u32>.
}

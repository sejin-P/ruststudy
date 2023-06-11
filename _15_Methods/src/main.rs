fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        fn say_hello(&self) {
            println!("hello, my name is {}", self.name);
        }
    }

    let peter = Person {
        name: String::from("peter"),
        age: 27,
    };

    peter.say_hello();
}

fn main() {
    // 15.0 Methods
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












    // 15.1 Method receiver
    // &self: borrows the object from the caller using a shared and immutable reference. The object can be used again afterwards.
    // &mut self: borrows the object from the caller using a unique and mutable reference. The object can be used again afterwards.
    // self: takes ownership of the object and moves it away from the caller. The method becomes the owner of the object. The object will be dropped (deallocated) when the method returns, unless its ownership is explicitly transmitted. Complete ownership does not automatically mean mutability.
    // mut self: same as above, but the method can mutate the object.
    // No receiver: this becomes a static method on the struct. Typically used to create constructors which are called new by convention.

}

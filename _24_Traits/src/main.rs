fn main() {
    // 24.0 Traits
    // Rust lets you abstract over types with traits. They're similar to interfaces.

    trait Pet {
        fn name(&self) -> String;
    }

    struct Dog {
        name: String,
    }

    struct Cat;

    impl Pet for Dog {
        fn name(&self) -> String {
            self.name.clone()
        }
    }

    impl Pet for Cat {
        fn name(&self) -> String {
            String::from("The cat")
        }
    }

    fn greet<P: Pet>(pet: &P) {
        println!("Who's a cutie? {} is!", pet.name());
    }

    let fido = Dog{ name: "Fido".into() };
    greet(&fido);

    let captain_floof = Cat;
    greet(&captain_floof);

















    // 24.1 Trait Objects
    trait Pet1 {
        fn name(&self) -> String;
    }

    struct Dog1 {
        name: String,
    }

    struct Cat1;

    impl Pet1 for Dog1 {
        fn name(&self) -> String {
            self.name.clone()
        }
    }

    impl Pet1 for Cat1 {
        fn name(&self) -> String {
            String::from("The cat")
        }
    }

    let pets: Vec<Box<dyn Pet1>> = vec![
        Box::new(Cat1),
        Box::new(Dog1 { name: String::from("Fido") }),
    ];

    for pet in pets {
        println!("Hello {}!", pet.name());
    }

    // Types that implement a given trait may be of different sizes. This makes it impossible to have things like Vec<Pet> in the example above.
    // dyn Pet is a way to tell the compiler about a dynamically sized type that implements Pet.
    // In the example, pets holds fat pointers to objects that implement Pet. The fat pointer consists of two components, a pointer to the actual object and a pointer to the virtual method table for the Pet implementation of that particular object.
    // Compare these outputs in the above example:

    println!("{} {}", std::mem::size_of::<Dog1>(), std::mem::size_of::<Cat1>());
    println!("{} {}", std::mem::size_of::<&Dog1>(), std::mem::size_of::<&Cat1>());
    println!("{}", std::mem::size_of::<&dyn Pet1>());
    println!("{}", std::mem::size_of::<Box<dyn Pet1>>());
}

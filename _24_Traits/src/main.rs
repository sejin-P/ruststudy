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
}

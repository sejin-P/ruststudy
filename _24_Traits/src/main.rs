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













    // 24.2 Deriving Traits
    // You can let the compiler derive a number of traits:
    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct Player {
        name: String,
        strength: u8,
        hit_points: u8,
    }

    let p1 = Player::default();
    let p2 = p1.clone();
    println!("Is {:?}]]nequal to {:?}?\nThe answer is {}", &p1, &p2, if p1 == p2 {"yes"} else {"no"});













    // 24.3 Default Methods
    // Traits can implement behavior in terms of other trait methods.

    trait Equals {
        fn equal(&self, other: &Self) -> bool;
        fn not_equal(&self, other: &Self) -> bool {
            !self.equal(other)
        }
    }

    #[derive(Debug)]
    struct Centimeter(i16);

    impl Equals for Centimeter {
        // implementation of equal -> automatically implemented not_equal
        fn equal(&self, other: &Centimeter) -> bool {
            self.0 == other.0
        }
    }

    let a = Centimeter(10);
    let b = Centimeter(20);

    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));

    trait Equals1 {
        fn equal(&self, other: &Self) -> bool;
    }

    trait NotEquals {
        fn not_equal(&self, other: &Self) -> bool;
    }

    impl<T> NotEquals for T where T: Equals1 {
        fn not_equal(&self, other: &Self) -> bool {
            !self.equal(other)
        }
    }











    // 24.4 Trait Bounds
    // When working with generics, you often want to require the types to implement some trait, so that you can call this trait’s methods.

    // You can do this with T: Trait or impl Trait:

    fn duplicate<T: Clone>(a: T) -> (T, T) {
        (a.clone(), a.clone())
    }


    fn add_42_millions(x: impl Into<i32>) -> i32 {
        x.into() + 42_000_000
    }

    let foo = String::from("foo");
    let pair = duplicate(foo);

    println!("{pair:?}");

    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
}

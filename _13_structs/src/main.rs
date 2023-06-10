#![recursion_limit = "1000"]

use std::mem::{align_of, size_of};
use std::mem::transmute;

fn main() {
    // 13.0 structs
    struct Person {
        name: String,
        age: u8,
    }

    let mut peter = Person {
        name: String::from("peter"),
        age: 27,
    };

    println!("{} is {} years old", peter.name, peter.age);

    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);

    let jackie = Person {
        name: String::from("jackie"),
        ..peter
    };

    println!("{} is {} years old", jackie.name, jackie.age);


    // Structs work like in C or C++.
    // Like in C++, and unlike in C, no typedef is needed to define a type.
    // Unlike in C++, there is no inheritance between structs.
    // Methods are defined in an impl block, which we will see in following slides.
    // This may be a good time to let people know there are different types of structs.
    // Zero-sized structs e.g., struct Foo; might be used when implementing a trait on some type but don’t have any data that you want to store in the value itself.
    // The next slide will introduce Tuple structs, used when the field names are not important.
    // The syntax ..peter allows us to copy the majority of the fields from the old struct without having to explicitly type it all out. It must always be the last element.













    // 13.1 Tuple Structs
    struct Point(i32, i32);

    let p = Point(1, 2);
    println!("{}, {}", p.0, p.1);

    // let t = p;
    // println!("{}", p.0);












    // 13.2 Filed shorthand syntax
    #[derive(Debug)]
    struct Person1 {
        name: String,
        age: u8,
    }

    impl Person1 {
        fn new(name: String, age: u8) -> Self {
            Person1 {name, age}
        }

        fn default() -> Self {
            Person1 {
                name: "bot".to_string(),
                age: 0,
            }
        }
    }

    let peter = Person1::new(String::from("Peter"), 27);
    println!("{peter:?}");















    // 14.0 Enums
    #[derive(Debug)]
    enum CoinFlip {
        Heads,
        Tails,
    }

    println!("you got: {:?}", CoinFlip::Heads);













    // 14.1 Variany payloads
    enum WebEvent {
        PageLoad,                 // Variant without payload
        KeyPress(char),           // Tuple struct variant
        Click { x: i64, y: i64 }, // Full struct variant
    }

    #[rustfmt::skip]
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad       => println!("page loaded"),
            WebEvent::KeyPress(c)    => println!("pressed '{c}'"),
            WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
        }
    }


    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);

    // The values in the enum variants can only be accessed after being pattern matched. The pattern binds references to the fields in the “match arm” after the =>.
    // The expression is matched against the patterns from top to bottom. There is no fall-through like in C or C++.
    // The match expression has a value. The value is the last expression in the match arm which was executed.
    // Starting from the top we look for what pattern matches the value then run the code following the arrow. Once we find a match, we stop.
    // Demonstrate what happens when the search is inexhaustive. Note the advantage the Rust compiler provides by confirming when all cases are handled.
    // match inspects a hidden discriminant field in the enum.
    // It is possible to retrieve the discriminant by calling std::mem::discriminant()
    // This is useful, for example, if implementing PartialEq for structs where comparing field values doesn’t affect equality.
    // WebEvent::Click { ... } is not exactly the same as WebEvent::Click(Click) with a top level struct Click { ... }. The inlined version cannot implement traits, for example.















    // 14.2 Enum sizes
    macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
                 stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

    enum Foo {
        A,
        B,
    }

    dbg_size!(Foo);

    // Internally Rust is using a field (discriminant) to keep track of the enum variant.
    //
    // You can control the discriminant if needed (e.g., for compatibility with C):
    #[repr(u32)]
    enum Bar {
        A,  // 0
        B = 10000,
        C,  // 10001
    }
    // Without repr, the discriminant type takes 2 bytes, because 10001 fits 2 bytes.
    // Try out other types such as
    //
    // dbg_size!(bool): size 1 bytes, align: 1 bytes,
    // dbg_size!(Option<bool>): size 1 bytes, align: 1 bytes (niche optimization, see below),
    // dbg_size!(&i32): size 8 bytes, align: 8 bytes (on a 64-bit machine),
    // dbg_size!(Option<&i32>): size 8 bytes, align: 8 bytes (null pointer optimization, see below).
    // Niche optimization: Rust will merge unused bit patterns for the enum discriminant.
    //
    // Null pointer optimization: For some types, Rust guarantees that size_of::<T>() equals size_of::<Option<T>>().
    //
    // Example code if you want to show how the bitwise representation may look like in practice. It’s important to note that the compiler provides no guarantees regarding this representation, therefore this is totally unsafe.


    macro_rules! dbg_bits {
        ($e:expr, $bit_type:ty) => {
            println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
        };
    }

    // TOTALLY UNSAFE. Rust provides no guarantees about the bitwise
    // representation of types.
    unsafe {
        println!("Bitwise representation of bool");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Bitwise representation of Option<bool>");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Bitwise representation of Option<Option<bool>>");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);

        println!("Bitwise representation of Option<&i32>");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0), usize);
        dbg_bits!(Some(&0i32), usize);
    }

    // Macro to wrap a value in 2^n Some() where n is the number of "@" signs.
    // Increasing the recursion limit is required to evaluate this macro.
    macro_rules! many_options {
    ($value:expr) => { Some($value) };
    ($value:expr, @) => {
        Some(Some($value))
    };
    ($value:expr, @ $($more:tt)+) => {
        many_options!(many_options!($value, $($more)+), $($more)+)
    };
}

    unsafe {
        assert_eq!(many_options!(false), Some(false));
        assert_eq!(many_options!(false, @), Some(Some(false)));
        assert_eq!(many_options!(false, @@), Some(Some(Some(Some(false)))));

        println!("Bitwise representation of a chain of 128 Option's.");
        dbg_bits!(many_options!(false, @@@@@@@), u8);
        dbg_bits!(many_options!(true, @@@@@@@), u8);

        println!("Bitwise representation of a chain of 256 Option's.");
        dbg_bits!(many_options!(false, @@@@@@@@), u16);
        dbg_bits!(many_options!(true, @@@@@@@@), u16);

        println!("Bitwise representation of a chain of 257 Option's.");
        dbg_bits!(many_options!(Some(false), @@@@@@@@), u16);
        dbg_bits!(many_options!(Some(true), @@@@@@@@), u16);
        dbg_bits!(many_options!(None::<bool>, @@@@@@@@), u16);
    }
}

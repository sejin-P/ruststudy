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
}

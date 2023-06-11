fn main() {
    //16.0 Pattern Matching
    let input = 'x';
    match input {
        'q' => println!("Quiting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }










    // 16.1 Destructing Enums
    enum Result {
        Ok(i32),
        Err(String),
    }

    fn divide_in_two(n: i32) -> Result {
        if n % 2 == 0 {
            Result::Ok(n / 2)
        } else {
            Result::Err(format!("cannot divide {n} into two equal parts"))
        }
    }

    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }











    // 16.2 Destructing structs

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3};

    match foo {
        Foo {x: (1, b), y} => println!("x.0 =1, b = {b}, y = {y}"),
        Foo { y: 2, x: i} => println!("y = 2, x = {i:?}"),
        Foo { y, ..} => println!("y={y}, other fields are ignored")
    }










    // 16.3 Destructing Arrays
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        _         => println!("All elements were ignored"),
    }












    // 16.4 Match Guards
    let pair = (0, 0);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }
}

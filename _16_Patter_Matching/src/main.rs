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
}

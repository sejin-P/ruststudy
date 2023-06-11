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















    //15.2 Examples

    #[derive(Debug)]
    struct Race {
        name: String,
        laps: Vec<i32>,
    }

    impl Race {
        fn new(name: &str) -> Self {
            Race { name: String::from(name), laps: Vec::new() }
        }

        fn add_lap(&mut self, lap: i32) {
            self.laps.push(lap);
        }

        fn print_laps(&self) {
            println!("Recorded {} laps for {}:", self.laps.len(), self.name);
            for (idx, lap) in self.laps.iter().enumerate() {
                println!("Lap {idx}: {lap} sec");
            }
        }

        fn finish(self) {
            let total = self.laps.iter().sum::<i32>();
            println!("Race {} is finished, total lap time: {}", self.name, total);
        }
    }

    let mut race = Race::new("monaco grand pix");

    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();

    race.add_lap(71);
    race.print_laps();

    race.finish();
}

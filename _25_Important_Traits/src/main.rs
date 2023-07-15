use std::io::{BufRead, BufReader, Read, Result, Write};

fn main() {
    // 25.1 Iterators

    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }

    let fib = Fibonacci {curr: 0, next: 1};
    for (i, n) in fib.enumerate().take(5) {
        println!("fib({i}): {n}");
    }













    // 25.2 FromIterator
    let primes = vec![2, 3, 5, 7];
    let prime_squres = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();

    // Iterator implements fn collect<B>(self) -> B where B: FromIterator<Self::Item>, Self: Sized

    // There are also implementations which let you do cool things like convert an Iterator<Item = Result<V, E>> into a Result<Vec<V>, E>.













    
    // 25.3 From and Into

    // Types implement From and Into to facilitate type conversions:
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127 , 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i16::from(123i16);

    println!("{s}, {addr}, {one}, {bigger}");

    // Into is automatically implemented when From is implemented:
    let s: String = "hello".into();
    let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one:i16 = true.into();
    let bigger: i32 = 123i16.into();
    println!("{s}, {addr}, {one}, {bigger}");

    // That’s why it is common to only implement From, as your type will get Into implementation too.
    // When declaring a function argument input type like “anything that can be converted into a String”, the rule is opposite, you should use Into. Your function will accept types that implement From and those that only implement Into.













    // 25.4 Read and Write
    fn count_lines<R: Read>(reader: R) -> usize {
        let buf_reader = BufReader::new(reader);
        buf_reader.lines().count()
    }

    let slice: &[u8] = b"foo]\nbar\nbaz\n";
    println!("lines in slice: {}", count_lines(slice));

    fn exe() -> Result<()> {
        let file = std::fs::File::open(std::env::current_exe()?)?;
        println!("lines in file: {}", count_lines(file));
        Ok(())
    }
    
    exe();

    fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
        writer.write_all(msg.as_bytes());
        writer.write_all("\n".as_bytes())
    }

    fn wexe() -> Result<()> {
        let mut buffer = Vec::new();
        log(&mut buffer, "Hello")?;
        log(&mut buffer, "World")?;
        println!("Logged: {:?}", buffer);
        Ok(())
    }

    wexe();











    // 25.5 DROP
    struct Droppable {
        name: &'static str,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Dropping {}", self.name);
        }
    }

    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "a" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    // a.drop();
    println!("Exiting main");

    // Why doesn’t Drop::drop take self?
    // Short-answer: If it did, std::mem::drop would be called at the end of the block, resulting in another call to Drop::drop, and a stack overflow!










    // 25.6 The Default Trait
    // Default trait produces a default value for a type

    #[derive(Debug)]
    struct Implemented(String);
    
    #[derive(Debug, Default)]
    struct Derived {
        x: u32,
        y: String,
        z: Implemented,
    }

    

    impl Default for Implemented {
        fn default() -> Self {
            Self("John Smith".into())
        }
    }

    let default_struct: Derived = Default::default();
    println!("{default_struct:#?}");

    let almost_default_struct = Derived {
        y: "Y is set!".into(),
        ..Default::default()
    };

    println!("{almost_default_struct:#?}");

    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());

    // It can be implemented directly or it can be derived via #[derive(Default)].
    // A derived implementation will produce a value where all fields are set to their default values.
    // This means all types in the struct must implement Default too.
    // Standard Rust types often implement Default with reasonable values (e.g. 0, "", etc).
    // The partial struct copy works nicely with default.
    // Rust standard library is aware that types can implement Default and provides convenience methods that use it.








    // 25.7 Add, Mul, ...
    // NOTE: if there aren't copy and clone, operations below can't be executed because memory dropped.
    #[derive(Debug, Copy, Clone)]
    struct Point { x: i32, y: i32 }

    impl std::ops::Add for Point {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            Self { x: self.x + other.x, y: self.y + other.y }
        }
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1+p2);
    println!("{:?}", p1);

    // You could implement Add for &Point. In which situations is that useful?
    // Answer: Add:add consumes self. If type T for which you are overloading the operator is not Copy, you should consider overloading the operator for &T as well. This avoids unnecessary cloning on the call site.
    // Why is Output an associated type? Could it be made a type parameter of the method?
    // Short answer: Function type parameters are controlled by the caller, but associated types (like Output) are controlled by the implementor of a trait.
    // You could implement Add for two different types, e.g. impl Add<(i32, i32)> for Point would add a tuple to a Point.










    
}

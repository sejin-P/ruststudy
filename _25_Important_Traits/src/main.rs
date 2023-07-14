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
}

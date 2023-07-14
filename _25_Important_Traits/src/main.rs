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













    

}

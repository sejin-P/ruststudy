fn main() {
    // Option and Result types: used for optional values and error handling.

    // String: the default string type used for owned data.

    // Vec: a standard extensible vector.

    // HashMap: a hash map type with a configurable hashing algorithm.

    // Box: an owned pointer for heap-allocated data.

    // Rc: a shared reference-counted pointer for heap-allocated data.







    // 19.1 Option And Result
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");

    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {idx:?}");

    // Option and Result are widely used not just in the standard library.
    // Option<&T> has zero space overhead compared to &T.
    // Result is the standard type to implement error handling as we will see on Day 3.
    // binary_search returns Result<usize, usize>.
    // If found, Result::Ok holds the index where the element is found.
    // Otherwise, Result::Err contains the index where such an element should be inserted.
    // 약간 kotlin에서의 arrow 같은 느낌? Left, Right 이랬떤 것 같은데.











    // 19.2 String
    // String is the standard heap-allocated growable UTF-8 string buffer.

    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!("s3: len = {}, number of chars = {}", s3.len(), s3.chars().count());
}

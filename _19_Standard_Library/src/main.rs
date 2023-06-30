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

    // String::new returns a new empty string, use String::with_capacity when you know how much data you want to push to the string.
    // String::len returns the size of the String in bytes (which can be different from its length in characters).
    // String::chars returns an iterator over the actual characters. Note that a char can be different from what a human will consider a “character” due to grapheme clusters.
    // When people refer to strings they could either be talking about &str or String.
    // When a type implements Deref<Target = T>, the compiler will let you transparently call methods from T.
    // String implements Deref<Target = str> which transparently gives it access to str’s methods.
    // Write and compare let s3 = s1.deref(); and let s3 = &*s1;.
    // String is implemented as a wrapper around a vector of bytes, many of the operations you see supported on vectors are also supported on String, but with some extra guarantees.
    // Compare the different ways to index a String:
    // To a character by using s3.chars().nth(i).unwrap() where i is in-bound, out-of-bounds.
    // To a substring by using s3[0..4], where that slice is on character boundaries or not.
    let s5 = &*s1;
    println!("{}", s5);
}

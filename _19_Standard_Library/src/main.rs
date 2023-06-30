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
}

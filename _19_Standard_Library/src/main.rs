use std::collections::HashMap;
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

















    // 19.3 Vec
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len()+1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // 이렇게도 assign 가능
    let mut v3 = vec![0, 0, 1, 2, 3, 4];
    // retain only the even elements
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // Remove consecutive duplicates
    v3.dedup();
    println!("{v3:?}");
    //  Vec is a type of collection, along with String and HashMap. The data it contains is stored on the heap. This means the amount of data doesn’t need to be known at compile time. It can grow or shrink at runtime.
    // Notice how Vec<T> is a generic type too, but you don’t have to specify T explicitly. As always with Rust type inference, the T was established during the first push call.
    // vec![...] is a canonical macro to use instead of Vec::new() and it supports adding initial elements to the vector.
    // To index the vector you use [ ], but they will panic if out of bounds. Alternatively, using get will return an Option. The pop function will remove the last element.
    // Show iterating over a vector and mutating the value: for e in &mut v { *e += 50; }























    // 19.4 HashMap
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of H f".to_string(), 207);
    page_counts.insert("Grimm's ".to_string(), 751);
    page_counts.insert("Pride".to_string(), 303);

    if !page_counts.contains_key("Les") {
        println!("We know about {} books, but not Les", page_counts.len());
    }

    for book in ["Adventures of H f", "Pride"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown")
        }
    }

    // Use the .entry() method to insert  a value if nothing is found
    for book in ["Pride", "Alice"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    {
        let a = "ABC";
        page_counts.insert(a.to_string(), 12);
    }

    println!("{page_counts:#?}");

    // HashMap is not defined in the prelude and needs to be brought into scope.

    // Try the following lines of code. The first line will see if a book is in the hashmap and if not return an alternative value. The second line will insert the alternative value in the hashmap if the book is not found.

    // let pc1 = page_counts
    //     .get("Harry Potter and the Sorcerer's Stone ")
    //     .unwrap_or(&336);
    // let pc2 = page_counts
    //     .entry("The Hunger Games".to_string())
    //     .or_insert(374);
    // Unlike vec!, there is unfortunately no standard hashmap! macro.

    // Although, since Rust 1.56, HashMap implements From<[(K, V); N]>, which allows us to easily initialize a hash map from a literal array:

    // let page_counts = HashMap::from([
    //     ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
    //     ("The Hunger Games".to_string(), 374),
    // ]);
    // Alternatively HashMap can be built from any Iterator which yields key-value tuples.

    // We are showing HashMap<String, i32>, and avoid using &str as key to make examples easier. Using references in collections can, of course, be done, but it can lead into complications with the borrow checker.

    // Try removing to_string() from the example above and see if it still compiles. Where do you think we might run into issues?
    // => 
    let mut page_counts_with_String = HashMap::new();
    page_counts_with_String.insert("Adventures of H f", 207);
    page_counts_with_String.insert("Grimm's ", 751);
    page_counts_with_String.insert("Pride", 303);

    if !page_counts_with_String.contains_key("Les") {
        println!("We know about {} books, but not Les", page_counts.len());
    }

    for book in ["Adventures of H f", "Pride"] {
        match page_counts_with_String.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown")
        }
    }

    // Use the .entry() method to insert  a value if nothing is found
    for book in ["Pride", "Alice"] {
        let page_count: &mut i32 = page_counts_with_String.entry(book).or_insert(0);
        *page_count += 1;
    }

    // {
    //     let abc = String::from("ABC");
    //     page_counts_with_String.insert(&abc, 12); => compile err!!!
    // }

    println!("ABC for {:?}", page_counts_with_String.get("ABC"));


    println!("{page_counts:#?}");










    








    // 19.5 Box
    let five = Box::new(5);
    println!("five: {}", *five);

    // Box is like std::unique_ptr in C++, except that it’s guaranteed to be not null.
    // In the above example, you can even leave out the * in the println! statement thanks to Deref.
    // A Box can be useful when you:
    // have a type whose size that can’t be known at compile time, but the Rust compiler wants to know an exact size.
    // want to transfer ownership of a large amount of data. To avoid copying large amounts of data on the stack, instead store the data on the heap in a Box so only the pointer is moved.
















    // 19.5.1 Box with Recursive Data Structures
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");

    //  If the Box was not used here and we attempted to embed a List directly into the List, the compiler would not compute a fixed size of the struct in memory, it would look infinite.

    // Box solves this problem as it has the same size as a regular pointer and just points at the next element of the List in the heap.

    // Remove the Box in the List definition and show the compiler error. “Recursive with indirection” is a hint you might want to use a Box or reference of some kind, instead of storing a value directly.

















    // 19.5.2 Niche Optimization
    // A Box cannot be empty, so the pointer is always valid and non-null. This allows the compiler to optimize the memory layout.

}

fn main() {
    // str = immutable reference to utf-8 encoded string data stored.
    let s1: &str = "World";
    println!("s1: {s1}");

    // String = wrapper around a vector of bytes.
    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3 {s3}");

    // result
    // s1: World
    // s2: Hello
    // s2: Hello World
    // s3 World
}

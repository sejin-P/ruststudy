fn main() {
    // 27.1 Panics
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]);

    // Panics are for unrecoverable and unexpected errors.
    // Panics are symptoms of bugs in the program.
    // Use non-panicking APIs(such as Vec::get) if crashing is not acceptable
}

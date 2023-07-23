fn main() {
    // 29.0 Unsafe Rust
    // Safe Rust: memory safe, no undefined behavior possible.
    // Unsafe Rust: can trigger undefined behavior if preconditions are violated.
    // 1. Dereference raw pointers.
    // 2. Access or modify mutable static variables.
    // 3. Access union fields.
    // 4. Call unsafe functions, including extern functions.
    // 5. Implement unsafe traits.



    // 29.1 Dereferencing Raw Pointers
    // Creating pointers is safe, but dereferencing them requires unsafe:
    let mut num = 5;
    let r1 = &mut num as *mut i32;
    let r2 = r1 as *const i32;


    // Safe because r1 and r2 were obtained from references and so are
    // guaranteed to be non-null and properly aligned, the objects underlying
    // the references from which they were obtained are live throughout the
    // whole unsafe block, and they are not accessed either through the
    // references or concurrently throught and other points.
    unsafe {
        println!("r1 is {}", *r1);
        *r1 = 10;
        println!("r2 is: {}", *r2);
        println!("{num}");
    }

    // In the case of pointer dereferences, this means that the pointers must be valid, i.e.:
    //
    // The pointer must be non-null.
    // The pointer must be dereferenceable (within the bounds of a single allocated object).
    // The object must not have been deallocated.
    // There must not be concurrent accesses to the same location.
    // If the pointer was obtained by casting a reference, the underlying object must be live and no reference may be used to access the memory.
    // In most cases the pointer must also be properly aligned.










    // 29.2 Mutable Static Variables
    // It is safe to read an immutable static variable:
    static HELLO_WORLD: &str = "Hello World";
    println!("HELLO_WORLD: {HELLO_WORLD}");

    // However, since data races can occur, it is unsafe to read and write mutable static variables:
    static mut COUNTER: u32 = 0;
    fn add_to_counter(inc: u32) {
        unsafe { COUNTER += inc;} // Potential Data race!
    }

    add_to_counter(42);
    unsafe { println!("COUNTER: {COUNTER}"); } // Potential Data Race!

    // Using a mutable static is generally a bad idea, but there are some cases where it might make sense
    // in low-level `no_std` code, such as implementing a heap allocator or working with some C APIs.
}

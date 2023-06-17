# confusing things

you should assign struct value as mutable if you want to make that struct's field mutable too.


## Why should I loop with reference of Vec<T>
In Rust, when you iterate over a collection such as a Vec<T> without a reference, you're taking ownership of the entire collection, not just its elements.

Let's take a look at this piece of code:

rust
Copy code
let v = vec![1, 2, 3];
for val in v {
println!("{}", val);
}
In this case, the for loop is equivalent to the following code:

rust
Copy code
let v = vec![1, 2, 3];
let mut iter = v.into_iter();
while let Some(val) = iter.next() {
println!("{}", val);
}
Here's what happens:

The into_iter() method takes ownership of v and returns an iterator that also owns v. This iterator (iter) yields the elements of v one at a time.

Each call to iter.next() takes one element out of v and gives you ownership of that element. The Some(val) pattern matches this owned element, and val is the owned element itself.

At the end of each loop iteration, val goes out of scope and is dropped, because you own it.

When all elements have been yielded and iter.next() returns None, the loop ends. At this point, iter also goes out of scope and is dropped. When iter is dropped, it drops whatever it owns - which in this case, is the (now empty) vector v.

So, you can see that the for loop takes ownership of the entire vector, not just its elements. This is part of Rust's design: a Vec<T> owns its elements, so when you take ownership of a Vec<T>, you also take ownership of its elements.

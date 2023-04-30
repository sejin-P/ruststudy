use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn takes_u32(x: u32) {
    println!("u32: {x}")
}

fn takes_i8(x: i8) {
    println!("i8: {x}")
}

fn main() {
    let x = 20;
    let y = 40;
    takes_u32(x);
    takes_i8(y);

    // The following code tells the compiler to copy into a certain generic container without the code ever explicitly specifying the contained type, using _ as a placeholder:
    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");

    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
    println!("{}", type_of(&vv));

    let vvv = v.iter().collect::<std::collections::HashSet<&(i32, bool)>>();
    println!("vvv: {vvv:?}");
    println!("{}", type_of(&vvv));









    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");

    // The following code demonstrates why the compiler can’t simply reuse memory locations when shadowing an immutable variable in a scope, even if the type does not change.
    let a = 1;
    let b = &a;
    let a = a + 1;
    println!("{a} {b}");
}

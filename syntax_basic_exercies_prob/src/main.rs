// problem from https://google.github.io/comprehensive-rust/exercises/day-1/implicit-conversions.html
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
fn main() {
    let x: i8 = 15;
    let y: i16 = 12;
    println!("{x} * {y} = {}", multiply(x.into(), y))
}

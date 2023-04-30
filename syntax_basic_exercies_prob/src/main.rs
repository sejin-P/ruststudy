// quiz from https://google.github.io/comprehensive-rust/exercises/day-1/implicit-conversions.html
// quiz from https://google.github.io/comprehensive-rust/exercises/day-1/for-loops.html
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[j][i] = matrix[i][j]
        }
    }

    return transposed
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    unimplemented!()
}
fn main() {
    let x: i8 = 15;
    let y: i16 = 12;
    println!("{x} * {y} = {}", multiply(i16::from(x), y));



    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");

    let transposed = transpose(matrix);
    println!("transposed: {transposed:?}");
}

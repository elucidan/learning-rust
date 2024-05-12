use std::io;

/*
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of y is {y}");
}
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3, 5]; // same as [3,3,3,3,3]
}
*/
fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index....");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("the value of element at index {index} is {element}");
    // if a user puts in an index too large rather than letting the user read invalid memory
}

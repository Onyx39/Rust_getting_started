use std::io;

fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");
    //println!("{}", guess)

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // println!("{}", c);
    // println!("{}", z);
    // println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    println!("{:#?}", tup);

    let (_x, y, _z) = tup;
    let firstvalue = tup.0;

    println!("The value of y is: {y}");
    println!("The first value is: {firstvalue}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}


use std::io;

fn main() {
    // scalar type: integers, floating-point numbers, Booleans, and characters

    //* Integer Types
    let _x: i8 = -128;
    let _y: u8 = 255;

    //* Depend on the architecture of the computer
    let _m: isize = -40;
    let _m: usize = 40;

    //* Floating-Point Types (All floating-point types are signed)
    let _f_64 = 2.0;
    let _f_32: f32 = 3.0;

    //* Numeric Operations
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

    // * Boolean Type
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    //* Character Type
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    //* Compound Types
    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // The Array Type
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [3; 5];

    // Accessing Array Elements
    let _first = a[0];
    let _second = a[1];

    // Invalid Array Element Access
    let b = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = b[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    println!("Hello, world!");
}

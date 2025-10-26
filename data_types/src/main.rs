use std::io;

fn main() {
    let number: f64 = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 10 / 2;
    let multiplication = 20 * 20;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    // char type
    let c = 'z';
    let z: char = 'Z';

    // tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructure
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    //accessing the tuple

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tuple.0;
    let six_point_four = tuple.1;
    let one = tuple.2;

    // array

    // use array if you know the number of the elements
    let a = [1, 2, 3, 4, 5];

    let months = [
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

    // with type anotation
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // initialize array with same value
    let a: [i32; 5] = [3; 5];

    // accessing array elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

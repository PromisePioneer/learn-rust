fn main() {
    println!("Hello, world!");

    another_function();
    test_function(5);

    print_labeled_measurement(5, '5');

    // this is a statement
    // let y = 6;

    // this is expression, it doesn't return a value.
    let y = {
        let x = 3;
        x + 1
    };

    println!("There value y is: {y}");

    let x = five();
    println!("the value of x is {x}");
}

fn another_function() {
    println!("Another function")
}

fn test_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//function thats return a value
fn five() -> i32 {
    5
}

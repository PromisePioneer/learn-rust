fn main() {

    // immutable by default
    /*
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {}", x);
     */

    // mutable variable
    /*
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3; // constant variable
     */

    //shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

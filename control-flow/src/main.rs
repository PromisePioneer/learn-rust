fn main() {

    // if expressions
    // let number = 7;
    //
    // if number < 5 {
    //     println!("true");
    // }else{
    //     println!("false");
    // }

    let number = 7;
    // if number { // expected value is bool, but its return integer, error.
    //     println!("number was three");
    // }

    //instead of that, use like this
    // you must explicitly with boolean as its conditions. unlike javascript or php. they can convert value from boolean to non-boolean
    if number != 0 {
        println!("number was something rather than 0");
    }


    // if elseif else


    let number = 6;

    if number % 4 == 0 {
        println!("number  is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }


    // using if on the let statement
    let condition = true; // condition is true
    let number = if condition { 5 } else { 6 }; // this will return 5;

    println!("the value of condition is: {number}"); // 5


    //the type must be the same condition
    // let condition = true;
    // let number = if condition { 5 } else { "six" }; // the type must be the same if not this will got missmatched type.

    // loop
    /*
            execute the block over and over again until you explicitly tell it to stop.
            rust have three kinds of loops:
            loop, while and for
         */

    // loop
    // loop {
    //     println!("again ");
    // }


    let mut counter = 0; // create a mutable variable

    // create a result variable to hold loop result
    let result = loop {
        counter += 1; // we bound +1 to counter value everytime we loop

        if counter == 10 { // tell when it needs to stop
            // break counter * 2; // specify value whe it stop.
            break counter; // like this.
        }
    };

    println!("the result is {result}");


    //loop labels to disambiguate between multiple loops.
    let mut count = 0;
    'counting_up: loop { // first loop we're labeling this loop to 'counting_up
        println!("count {count}"); // print count, this will be 0 at first

        let mut remaining = 10; // init remaining variable, which is 10;
        loop {
            println!("remaining {remaining}"); // this will be 10, at the beginning.
            if remaining == 9 { // if remaining == 9 stop the loop.
                break;
            }

            if count == 2 { // if counting == 2 stop the outer loop which is counting up.
                break 'counting_up;
            }

            remaining -= 1 // -1 to remaining everytime remaining is not stop.
        }

        count += 1; // +1 to count everytime count is not stop.
    };

    println!("End count = {count}");


    // while

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!");
}

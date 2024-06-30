// function as entry point
// fn main() {
//     println!("Hello worlds");
//     another_function(32); // calling another function with args
//
//
//     //statement are instructions that perform some action and do not return value
//     //expressions are evaluate to resultant value
//
//     let y = 6; // these are statement.
//
//
//     // let x =( let y = 6;) cannot assign let statement to another variable.
//
//
//
//     // calling block of scope inside variable is called expression.
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("the value of x is {y}"); //so we can call it.
//
//     let x= seven(); // calling the functions with arg
//
//     println!("the value of x is: {x}"); // check the seven() function is returning value of 7;
// }
//
//
// // another functions
// fn another_function(x: i32) { // parameter
//     println!("the value of x is {x}"); //calling the value of 32 on main functions.
// }
//
//
// //return value
// fn seven() -> i32{ // must returning type i32
//     7
// }


// another example to return value function
fn main() {
    let x = plus_two(2); // calling the function and passing the value of 2
    println!("the value of x is {x}") // print the x variable.
}

fn plus_two(x: i32) -> i32 { // take the value from main type i32 function and return it as i32
    x + 2 // no need semicolon on return functions.
}

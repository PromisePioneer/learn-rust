
/*
fn main() {
    // stack vs heap

    /*
        1. stack
        store values in the order it gets them and remove the value in the opposite order (last in first out).
        all data stored in stack must have known, fixed size. data with unknown size at compile time or a size that might change must be store on the heap instead.


        2. Heap
           when you put data on the heap, you request a certain amount of space. the memory allocator finds an empty spot in the heap that is big enough, mark it as being use, and returns pointer, which is adress of that locations


         1.stack:
            pushing data into stack is faster than allocating on the heap because the allocator never has to search for a place to store new data. the locations are always at the top of stack.
         2.heap:
            accessing data in the heap is slower than accessing data on the stack because you have to follow pointer to get there.

     */


    /*
       Ownership
       - each value in rust has an owner.
       - there can only be one owner at the time.
       - when the owner goes out the scope, the value will be dropped.
    */

    /*
        Variable scope
     */

    { // s no valid here cause of not been declared.
        let s = "hello world"; // s is valid here cause it's been declared

        //do stuff with s.
    } // the scope is over. and s is no longer valid here.


    //String type.

    // the :: operator allow us to namespace this particular from function under the String type rather than using some sort of name like string_from.

    //this kind of string can be mutated.
    let mut s = String::from("Hello");
    s.push_str(", world"); // push_str() appends a literal to a string.
    println!("{s}"); //This will print hello world.

    // cannot be mutated.
    // let mut s = "Hello";
    //
    // s.push_str(", world");
    //
    // println!("{s}");

    // string::from can be mutated and literal cannot because of these kind of two type are different when comes to dealing with memory.

    // rust have some function to free the memory which is drop().


    //Variable and data interacting with Move
    // this one is simple, these values both x and y stored onto stack. because they have a known size.
    let x = 10;
    let y = x;


    //string version, its different
    let s1 = String::from("Hello World");
    let s2 = s1;


    /*
        a String::from made up with three parts: pointer, len, capacity. this group of data stored on the stack.
        and the value stores on heap. because it might grow or shrink later.

        when passing s1 to s2, the string data copied, meaning we copy the pointer, the length and the capacity that are on the stack, we do not copy the data on the heap that the pointer refers to. we copy entire s1 to s2 but not the heap data just stack data into s2. this means not COPY ITS MOVE to s2, to ensure memory safety, when s1 and s2 goes out the scope they try to free the same memory. this is known as double free error.
     */


    // println!("the value of s1 is {s1}"); //if we try to copy value here its


    //Variable and data interacting with clone

    let s1 = String::from("hello");

    // this will copy entire s1 stack and heap to s2, careful with this, will cause a performance issue with your code. this will copy s1 and the s1 still valid.
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");


    // all types that we can implement copy.

    /*
        -all integers types.
        - boolean
        - all floating points.
        - char type
        - tuples if they only contain types that implement copy. string is not
     */


    // ownership and functions

    /*
        the mechanics of passing value to a function are similar to those when assigning a value to a variable.
     */


    let s = String::from("hello"); //s comes into scope

    takes_ownership(s); //s move to scope. its not valid below this code

    let x = 5; // x valid in this code.
    make_copy(x); // x still valid on this, it will copy not move. so we can use it afterward below this code
} // x and s goes out of scope. because the s value is moved, nothing happen


fn takes_ownership(some_string: String) { // some_string comes out to the scope
    println!("{some_string}");
} // some_string out of the scope, memory is freed


fn make_copy(some_integer: i32) { // some_integer comes out to the scoop
    println!("{some_integer}");
} // some integer goes out the scope. nothing special happen.
 */







// returning value and scope.

fn main() {
    let s1 = gives_ownership(); // the return value from gives_ownership func is moved from gives ownership function.

    let s2 = String::from("Hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 moved into takes_and_gives_back function and the return value will move to s3.


}


fn gives_ownership() -> String { // gives_ownership will move its value its return value to a calling function.
    let some_string = String::from("yours"); // some string comes into scope

    some_string //  some string is  return and moves out  to the calling function.
}


fn takes_and_gives_back(a_string: String ) -> String { // a_string comes into scope
    a_string // a string is return and moves out the calling function
}

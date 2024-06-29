fn main() {
    /* there are 2 types of datatypes in rust : scalar and compoud
        1. Scalar
          a scalar type represent a single value. Rust has four primary scalar types.
          1. Integer
          2. floating point
          3. numbers
          4. boolean
          5. characters

        2. Compound
           a compound can group a multiple value into 1 type. rust has 2 primitive compound types.
            1. Tuple
            2. Array


     */


    // integer
    /*
        An integer is a number without fractional component.

        signed integer :the value can be positive only.
        unsigned integer : the value can be positive number and negative number.

        example :
     */

    // let unsigned_integer1: u32 = -23; // can't be negative
    let unsigned_integer2: u32 = 23; // positive only

    let signed_integer: i32 = -32; // can be negative
    let signed_integer2: i32 = 32; // can be positive


    /*
        additionally, the isize and usize types depends on the architecture of the computer your program is running on, 64 bit if your on a 64bit architecture and 32bit if your on 32bit architecture
     */

    /*
        NOTES : integer types default to i32.
     */


    // floating point.
    /*
        floating points is a numbers with decimal point, we got f32 and f64, default is f64.
        example :
     */

    let floating_point = 2.0; //f64, using its default value if we doesnt declare any float type.
    let floating_point: f32 = 3.0; //f32


    //numeric operation

    // sum
    let sum = 5 + 10;
    // subtraction
    let subract = 5 - 10;

    // multiply
    let multiply = 4 * 30;
    // division
    let division = 56.7 / 32.2;
    let truncated = -5 / 3;
    // remainder
    let remainder = 43 % 5;


    // boolean
    let truth = true;
    let falses: bool = false; // with type annotation


    // char type
    let c = 'z'; //with single quoted and only 1 letter.


    // TUPLE

    /*
        tuple is a general way of grouping together a number of values with a variety of types into 1 compound type. tuple has a fixed length. 1 declared, it cannot grow or shrink.

        example :
     */

    let tup: (i32, f64, u8) = (500, 5.4, 1);

    /*
        to destructure tuple
     */

    let (x, y, z) = tup;

    println!("the value of x is {x}", );

    // using period, followed by index we want to access.
    let five_hundred = x.0;


    // array
    /*
        array is useful when you want your data allocated on the stack rather than the heap.
     */
    let a = [1, 2, 3, 4];

    // here, i32 is the type of each element. After the semicolon, the number5 indicates array contain 5 element.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // initial an array to contain the same value for each element by specifying the initial value, followed by semicolon and the length of the array in square brackets, as shown here:
    let a = [3; 5]; // will contain 5 elements that will all be set to the value 3 initially like [3,3,3,3].

    // accessing array
    let first = a[0];
    let second = a[1];


}

fn main() {
    // each value in rus has an owner;
    // there can only be one owner at a time.
    // when the owner goes out of scope, the value will be dropped.
    //
    // { // s is not valid here, since it's not yet declared.
    //     let s = "hello"; //this value stores on stack (fixed size), s is valid from this point.
    //     // do stuff with s.
    // } // this scope is now over, and s is no longer valid.
    //
    //
    // {
    //     let mut s = String::from("Hello"); // this value stores in heap.
    //     s.push_str(", world!");
    //
    //     println!("{s}");
    //
    //
    //     let x = 5;
    //     let y = x;
    //
    //     let mut s = String::from("Hello"); //dropped, karena udah dirubah dibawah
    //     s = String::from("ahoy");
    //
    //
    //     println!("{s}, World");
    //
    //
    //     //clone both heap and stack
    //     let s1 = String::from("Hello");
    //     let s2 = s1.clone(); // clone ini berfungsi untuk mengcopy seluruh stack dan heap s1 ke s2
    //
    //     println!("s1 = {s1}, s2 = {s2}");
    //
    // }


    let s = String::from("Hello");
    let (s2, len) = length_of_string(s);


    println!("The length of {s2} is {len}");
}

fn length_of_string(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

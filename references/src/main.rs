fn main() {
    // let s1 = String::from("Hello");
    // let len = calculate_length(&s1); // take references
    //
    //
    // println!("The length of '{s1}' is {len}");


    let mut s = String::from("Hello");
    change(&mut s);

    println!("{s}");
}




fn change(some_string: &mut String){
    some_string.push_str(", World");
}

// fn calculate_length(s: &String) ->  usize {
//     s.len();
// }

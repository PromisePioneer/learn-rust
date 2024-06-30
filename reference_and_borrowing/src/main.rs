fn main() {

    let s1 =  String::from("Hello");

    // instead moving value from s1 to len variable we use reference, and so we don't need to return it back to s1 value. so we can reuse the value.
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");



    //if we try to modify something we borrowed from the other variable or statement.
    let mut s = String::from("Hello");

    change(&mut s);



    // after we have 1 mutable reference to value, we cant have another reference to that value.
    // example

    let mut s = String::from("Hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // println!("{r1} {r2}");

    // but we can do a multiple mutable reference like this

    {
        let r1 = &mut s;

    }

    println!("{r1}");

    let r2 = &mut s;
}



fn calculate_length(s: &String) -> usize { // we use &String reference type for reference.
    s.len()
} // s goes out the scope but because it has reference from the s1 it is not dropped, so we can still use it



//but we can change its value if variable are mutable and we added &mut table on our args, its called mutable reference
fn change(mut some_string: &mut String) {
    some_string.push_str(", world"); //error cause its reference we cannot change reference value.
}

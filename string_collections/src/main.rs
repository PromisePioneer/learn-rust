use std::fmt::format;

fn main() {
    let mut s = String::new();

    let data = "iniital contents";

    let s = data.to_string();

    //the method also works on a literal directly;

    // ini behaviormya sama dengan String::from()
    let s = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format menghasilkan reference, jadi tidak mengambil ownership.
    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    let s1 = String::from("Hi");
    // let h = s1[0]; // error string di rust tidak mendukung pengindeksan

    // string slice

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    
}

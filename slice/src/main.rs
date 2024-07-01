fn main() {
    //slice let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // slice is kind of reference, so its does not have ownership.
    let mut s = String::from("Hello World");
    let word = first_word(&s); // word will get the value 5

    s.clear(); // this makes the string empty :  "";

    println!("{word}"); // word, It's still work here but no more string.


    //string slice

    //a string slice is a reference to part of a string,
    // and it looks like this
    let s = String::from("Hello world");


    let hello = &s[0..5]; // hello is a reference to a portion of a string.
    let world = &s[6..11];


    let hello = &s[..6]; // same as [0..6].

    let len = s.len();
    let hello = &s[3..len]; //slice string from 3 to last byte.
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to an array.


    for (i, &item) in bytes.iter().enumerate() { // iterate the string.
        if item == b' ' { // search for byte literal syntax, if we find the space we return the current [index].
            return &s[0..i];
        }
    }

    s.len() // otherwise we return the string length if we don't find any space.
}

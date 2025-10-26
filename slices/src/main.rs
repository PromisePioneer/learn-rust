fn main() {
    let mut s = String::from("Hello");
    let word = first_words(&s);
    s.clear(); // this empties the string making it equal to ""

    // string slices

    let s = String::from("Hello world!");
    let hello = &s[0..5]; // [starting_index...ending_index]
    let world = &s[6..11];

    //start from index 0
    let slice = &s[0..2];
    let slice = &s[..2];

    //includes the last bytes of the string
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
}

fn first_words(s: &String) -> &str {
    let bytes = s.as_bytes(); // mengambil bytes: array [72, 101, 108, 108, 111]

    for (i, &item) in bytes
        .iter() // membuat iterator
        .enumerate()
    //mengubah iterator agar dapat indexnya saat iterasi dilakukan
    {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

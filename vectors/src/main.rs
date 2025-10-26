fn main() {
    let vec: Vec<i32> = Vec::new();

    //macro
    let v = vec![1, 2, 3];

    // adding vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let v = vec![1, 2, 3, 4, 5];

    // panic karena merujuk ke element yang tidak ada
    // let does_not_exist = &v[100];

    // akan None
    let does_not_exist = v.get(100);

    println!("{does_not_exist:?}");

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // not working because it has references
    println!("the first element is : {first}");

    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}")
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // using enum to save different types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{row:?}")
}

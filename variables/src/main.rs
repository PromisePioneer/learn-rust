fn main() {
    // const PER_PAGE: u32 = 20;

    let mut name: &str = "Firman";
    println!("The value of name is: {name}");
    name = "Kevin";
    println!("The value of name is: {name}");

    let number1 = 20;

    {
        let number1 = number1 + 2;
        println!("The value of number1 in the inner scope is: {number1}");
    }

    println!("The value of number is: {number1}");


    let spaces = "   ";
    let spaces = spaces.len();


    println!("spaces {spaces}");
}

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    //creating instance
    let mut user1 = User {
        active: true,
        username: String::from("fifirman000"),
        email: String::from("fifirman000@gmail.com"),
        sign_in_count: 1
    };

    // change struct value
    user1.email = String::from("kevariable@gmail.com");


    //create instance from other instance
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("sahabat@gmail.com"),
    //     sign_in_count: user1.sign_in_count
    // };





    // shorthand
    let user2 = User {
        active: false,
        email: String::from("cabat@gmail.com"),
        username : user1.username,
        sign_in_count: 7
    };

    println!("{}", user1.active);

}


fn build_user(email: String, username:String)-> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

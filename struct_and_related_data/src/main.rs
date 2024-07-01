// struct is similar to tuple, can be a different types, in struct you'll name each piece of data. so it's clear what each value mean.

// specify the user struct and its types each key
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit like struct without any fields
struct AlwaysEqual;

fn main() {
    // we can fill user struct but not in order, its flexible

    let mut user1 = User { // mutable instance
        active: false,
        username: String::from("firman"),
        email: String::from("fifirman000@gmail.com"),
        sign_in_count: 0,
    };


    //access the specific struct, and if the instance is mutable we can change the value by using dot notation and assigning particular field

    user1.email = String::from("anotheremail@example.com");

    // creating instance from another instance with struct update syntax
    // let user2 = User{
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count
    // };

    // this is the shorthand struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    let subject = AlwaysEqual;
}


fn build_user(email: String, username: String) -> User {
    User {
        active: false,
        username, //same as username : username
        email, // same as email: email // this is the shorthand syntax
        sign_in_count: 0,
    }
}

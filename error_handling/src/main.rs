use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // let greeting_file_result = File::open("Hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };
    //
    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
    //     ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!("Problem creating the file: {e:?}"),
    //     },
    //     _ => {
    //         panic!("Problem opening the file: {error:?}")
    //     }
    // });

    // with closure.
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file {error:?}");
    //         })
    //     } else {
    //         panic!("Problem creating the file: {error:?}");
    //     }
    // });
    //
    // //expect
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project.");

    read_username_from_file().expect("TODO: panic message");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");
    //
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //
    // let mut username = String::new();
    //
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }


    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

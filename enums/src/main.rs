enum IpAddrKind {
    V4,
    V6
}


enum Message {
    Quit, //tidak ada data yang terkait
    Move {x: i32, y: i32}, // ada field, kaya struct
    Write(String), // string type
    ChangeColor(i32,i32, i32) // ada 3 nilai i32
}


impl Message {
    fn call (&self){

    }
}


enum Option<T> {
    None,
    Some(T),
}

enum IpAddrEnumWithTypes{
    V4(u8,u8,u8,u8),
    V6(String)
}

// struct with enum
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


fn main() {
    let home = IpAddrEnumWithTypes::V4(127,0,0,0);
    let loopback = IpAddrEnumWithTypes::V6(String::from("::1"));

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);


    let m = Message::write(String::from("Hello"));
    m.call();


    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum =  x + y;
}




fn route(ip_kind: IpAddrKind) {}
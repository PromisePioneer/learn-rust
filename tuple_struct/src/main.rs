struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// struct without field
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit like struct
    let subject = AlwaysEqual;
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
    let user2 = User {
        email: String::from("yetanother@example.com"),
        ..user1
    };
    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

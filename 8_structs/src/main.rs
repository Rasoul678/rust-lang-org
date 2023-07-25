fn main() {
    let email = String::from("h.rostami.r@gmail.com");
    let username = String::from("Rasoul678");

    let mut user = build_user(email, username);

    user.sign_in_count = 4;

    println!("{:?}", user);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user // active: user.active,
               // username: user.username,
               // sign_in_count: user.sign_in_count,
    };

    println!("{:?}", user2);

    // Using Tuple Structs Without Named Fields to Create Different Types
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    let _subject = AlwaysEqual;

    // Ownership of Struct Data
    
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

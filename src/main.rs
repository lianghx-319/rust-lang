struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user = build_user(String::from("a@b.com"), String::from("Han"));

    println!("user: {}", user.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: true,
    }
}

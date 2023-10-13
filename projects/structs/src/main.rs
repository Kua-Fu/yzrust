struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("testuser@qq.com"),
//         username: String::from("testuser"),
//         active: true,
//         sign_in_count: 1,
//     };
//     let user2 = User {
//         email: String::from("testuser2@qq.com"),
//         ..user1
//     };
// }

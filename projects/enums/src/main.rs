#![allow(unused)]
fn main() {
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
}

// fn main() {
//     enum Option<T> {
//         None,
//         Some(T),
//     }

//     let some_number = Some(5);
//     let some_char = Some('e');
//     let absent_number: Option<i32> = None;
// }

// fn main() {
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }

//     impl Message {
//         fn call(&self) {}
//     }

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum IpAddrKind {
//     V4,
//     V6,
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// fn main() {
//     println!("Hello, world!");
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }

// fn route(ip_kind: IpAddrKind) {}

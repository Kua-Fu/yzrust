fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("the length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1={}, s2={}", s1, s2);
// }

// fn main() {
//     // let x = 5;
//     // let y = x;

//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s2);
// }

// fn main() {
//     let s = String::from("hello");
//     let mut s1 = String::from("hello");
//     s1.push_str(", world!");
//     println!("{s1}");
//     println!("{s}");
// }

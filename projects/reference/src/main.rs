fn main() {
    println!("hello");
}

// fn main() {
//     let refreence_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     println!("{} and {}", r1, r2);
//     println!("{} and {}", r1, r2);

//     let r3 = &mut s;
//     println!("{}", r3);
// }

// fn main() {
//     let mut s = String::from("hello");
//     {
//         let r1 = &mut s;
//     }
//     let r2 = &mut s;
// }

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
//     println!("{s}");
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of y is: {y}");
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("the values of tup is: {x}, {y}, {z}");
}

// fn main() {
//     let spaces = "   ";
//     println!("spaces string is {spaces}");
//     let spaces = spaces.len();
//     println!("spaces number is {spaces}");
// }

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("the value of x in the inner scope is: {x}");
//     }
//     println!("the value of x is : {x}");
// }

// fn main() {
//     let mut x = 5;
//     println!("the value of x is: {x}");
//     x = 6;
//     println!("the value of x is: {x}");
//     println!("Hello, world!");
// }

// fn main() {
//     let x = 5;
//     println!("the value of x is: {x}");
//     x = 6;
//     println!("the value of x is: {x}");
//     println!("Hello, world!");
// }

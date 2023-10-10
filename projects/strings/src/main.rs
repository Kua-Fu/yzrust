fn main() {
    let s = String::from("hello");
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{s1}");
    println!("{s}");
}

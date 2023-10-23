fn main() {
    use std::collections::HashMap;
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();
//     scores.insert(String::from("blue"), 10);
//     scores.entry(String::from("yellow")).or_insert(50);
//     scores.entry(String::from("blue")).or_insert(20);
//     println!("{:?}", scores);
// }

// fn main() {
//     use std::collections::HashMap;
//     let field_name = String::from("favorite color");
//     let field_value = String::from("blue");

//     let mut map = HashMap::new();

//     map.insert(field_name, field_value);
//     println!("{field_name}");
// }

// fn main() {
//     use std::collections::HashMap;
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("yellow"), 50);

//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
// }

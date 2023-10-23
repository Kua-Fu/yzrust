fn main() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!(
                    "guess value invalid, must be between 1 and 100, got {}.",
                    value
                );
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

// fn main() {
//     use std::net::IpAddr;

//     let home: IpAddr = "127.0.0.1"
//         .parse()
//         .expect("hardcoded IP address should be valid");
// }

// use std::fs::File;

// fn main() {
//     let greeting_file =
//         File::open("hello2.txt").expect("hello2.txt should be included in the project");
// }

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello1.txt").unwrap();
// }

// use std::fs::File;
// use std::io::ErrorKind;
// use std::thread::panicking;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("problem create the file: {:?}", error);
//             })
//         } else {
//             panic!("problem open the file: {:?}", error);
//         }
//     });
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("problem create the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("problem opening the file: {:}", other_error);
//             }
//         },
//     };
// }

// use std::fs::File;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("problem opening the file: {:?}", error),
//     };
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     v[99];
// }

// fn main() {
//     println!("Hello, world!");
//     panic!("crash and burn");
// }

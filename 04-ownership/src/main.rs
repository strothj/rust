// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!");

//     println!("{}", s);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("{}", s2);

//     let mut s3 = s2;
//     s3.push_str(", world!");
// }

// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x: {}, y: {}", x, y);
// }

// fn main() {
//     let s = String::from("hello");

//     takes_ownership(s);

//     // println!("{}", s);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn main() {
//     let s1 = gives_ownership();

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     // let r3 = &mut s;
// }

// fn main() {
//     println!("{}", first_word(&String::from("hello w")));
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn main() {
    let s = "Hello, world!";

    let first = first_word(s);
    println!("{}", first);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{}", slice[0]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

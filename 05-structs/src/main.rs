// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     // let mut user1 = User {
//     //     username: String::from("someusername123"),
//     //     email: String::from("someone@example.com"),
//     //     active: true,
//     //     sign_in_count: 1,
//     // };
//     let mut user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );

//     user1.email = String::from("anotheremail@example.com");

//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         ..user1
//     };

//     println!("{:?}", user2);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}, {:?}", black, origin);
}

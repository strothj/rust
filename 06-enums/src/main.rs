// fn main() {
//     // let four = IpAddrKind::V4;
//     // let six = IpAddrKind::V6;

//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };

//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),
//     // };
//     // let home = IpAddr::V4(String::from("127.0.0.1"));
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));

//     // route(&four);
//     // println!("{:?}, {:?}", four, six);
//     println!("{:?}, {:?}", home, loopback);

//     let message_move = Message::Move { x: 0, y: 0 };
//     // println!("{:?}", message_move);
//     message_move.call();
// }

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", self);
//     }
// }

// #[derive(Debug)]
// // enum IpAddrKind {
// // V4,
// // V6,
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// // #[derive(Debug)]
// // struct IpAddr {
// //     kind: IpAddrKind,
// //     address: String,
// // }

// // fn route(ip_type: &IpAddrKind) {
// //     println!("{:?}", ip_type);
// // }

// fn main() {
//     let coin = Coin::Quarter(UsState::Alabama);

//     println!("{}", value_in_cents(coin));
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("{:?}, {:?}", six, none);

//     let some_u8_value = 0u8;
//     match some_u8_value {
//         1 => println!("one"),
//         3 => println!("three"),
//         _ => (),
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

fn main() {
    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("{}", 3),
    //     _ => (),
    // }
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("something else");
    }
}

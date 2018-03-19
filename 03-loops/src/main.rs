// fn main() {
// loop {
//     println!("again!");
// }

// let mut number = 3;

// while number != 0 {
//     println!("{}!", number);

//     number = number - 1;
// }

// let a = [10, 20, 30, 40, 50];
// let mut index = 0;

// while index < 5 {
//     println!("the value is: {}", a[index]);

//     index = index + 1;
// }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }

// fn main() {
//     for number in (1..4).rev() {
//         println!("{}!", number);
//     }
// }

use std::io;

fn main() {
    let mut choice = String::new();

    println!("1. Fahrenheit to Celsius");
    println!("0. Quit");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: i32 = choice.trim().parse().expect("Expected a number.");

    if choice == 1 {
        fahrenheit_to_celsius();
    }
}

fn fahrenheit_to_celsius() {
    let mut fahrenheit = String::new();

    println!("Enter fahrenheit:");
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Expected a float.");
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Expected float.");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("Celsius: {}", celsius);
}

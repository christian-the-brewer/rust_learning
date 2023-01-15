// hello world
// fn main() {
//     println!("Hello, world!");
// }

// //taking in user data and generating a random number
// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     loop {
//         println!("Enter you're guess: ");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You're guess: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }

// shadowing and tuples and arrays
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is {x}");
    }

    println!("the value of x is {x}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the value of y is {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    println!("five hundred = {five_hundred}");

    let a = [1, 2, 3, 4, 5];

    println!("{} {}", a[0], a[1]);
}

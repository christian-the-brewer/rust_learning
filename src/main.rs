// hello world
// fn main() {
//     println!("Hello, world!");
// }

//taking in user data
use std::io;
fn main() {
    println!("Guess the number!");
    println!("Enter you're guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You're guess: {guess}");
}

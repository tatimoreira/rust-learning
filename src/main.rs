use std::io;

fn main() {
    println!("Guess the number!");

    println!("Input a number!");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");

    print!("You guessed: {}", guess)
}

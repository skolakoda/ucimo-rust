use std::io;

fn main() {
    println!("Pogodi broj!");

    println!("Unesi svoje pogadjanje.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}

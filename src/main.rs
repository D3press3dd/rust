use std::io;
fn main() {
    println!("Guess the number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read the variable");

    println!("you guessed {guess}");
}

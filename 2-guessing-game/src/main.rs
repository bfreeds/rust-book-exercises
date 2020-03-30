use std::io; // bring io from std library to accept user input

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    // let creates a variable, mut makes it mutable
    let mut guess = String::new(); // constructs a new String instance.  ::new() is an associated method of String (like a static method)

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

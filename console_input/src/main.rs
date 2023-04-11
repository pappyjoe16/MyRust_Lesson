use std::io;

fn main() {
    // Print a message to the console
    println!("Please enter your name: ");

    // Create a mutable string variable to store the user's input
    let mut name = String::new();

    // Read input from the user and store it in the name variable
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // Print a message to the console that includes the user's name
    println!("Hello, {}!", name.trim());
}

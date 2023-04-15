use std::io;

fn main() {
    // Note that you can only add, subtract, divide, multiply value of the same data type
    // x: i32 = 9, x = 9i32, x = 9_i32, x = 9 as i32 all this are the same
    // 127_000 is the same as 127000
    // To get the highest value of a data type i32::MAX
    let x: f32 = 99.0 ;
    let y = 8.0 as f32;
    let z = x / y; // casting y with smaller data type to f64, this is type conversion 

    println!("{}", z);

    // converting string to integer.This can only convert digit you entered as string
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Expected to read line");
    let int_input: i64 = input.trim().parse().unwrap(); // the .trim is to remove the invisible special character (new line)
    

    println!("You entered: {}", int_input + 5);

}

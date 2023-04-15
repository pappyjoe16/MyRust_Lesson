use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Enter the First number to add");
    io::stdin()
        .read_line(&mut input1)
        .expect("Expected integer value");
    let num1: i64 = input1.trim().parse().unwrap(); // the .trim is to remove the invisible special character (new line)

    println!("Enter the Second number to add");
    io::stdin()
        .read_line(&mut input2)
        .expect("Expected integer value");
    let num2: i64 = input2.trim().parse().unwrap(); // the .trim is to remove the invisible special character (new line)

    add_numbers(num1, num2);
    subtract_numbers(num1, num2);

}

fn add_numbers(x: i64, y: i64){
    let sum = x + y;
    println!("The sum of the two number is: {}", sum)
}

fn subtract_numbers(x: i64, y: i64){
    let sub = x - y;
    println!("The sum of the two number is: {}", sub)
}
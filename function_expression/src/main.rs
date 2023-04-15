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

    let add_result = add_numbers(num1, num2);  // this is a statment because it did not return any value it only assign value to add_result
    let sub_result = subtract_numbers(num1, num2);
    println!("The sum of the two number is: {}", add_result);
    println!("The sub the two number is: {}", sub_result);

}

fn add_numbers(x: i64, y: i64) -> i64 { // the arrow here tell the data type of the return value 
   // x + y //this is an expression because it return a value and that is why we dont have ';' after because it is an expression
    // To use ';' we can use return keyword 
    return x + y; // with ';' you must use return keyword or you use return keyword without ';'
}

fn subtract_numbers(x: i64, y: i64) -> i64{
    //x - y //without the return keyword do not put ';'
    let result = x - y; //we can also have statement like this then expression must follow if we want to return any value
    return result;
}
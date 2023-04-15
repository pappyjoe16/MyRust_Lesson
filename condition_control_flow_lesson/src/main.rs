use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your favorite food");
    io::stdin()
        .read_line(&mut input)
        .expect("Expected type of food");
    
    if input.trim() == "Pizza"{
        println!("I like Pizza too!");
    }else if input.trim() == "Bread"{
        println!("{}, That sounds boring!", input.trim());
    } else {
        println!("{}, Not my favorite!", input.trim());
    }

}

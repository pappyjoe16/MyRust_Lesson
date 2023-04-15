fn main() {
    // to check for condition
    // the operator we have in rust &&, ||, !.... out of three the order on value is !, && and ||
    let cond1 = "Hello";
    let cond2 = "Hello_Hello";
    let cond3 = cond1 < cond2; 
    
    println!("{}", cond3);
}

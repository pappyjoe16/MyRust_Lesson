fn main() {
    let mut x = 4; //Implicitly define variable type this variable is an immutable variable
    //let y: u32 = 10; // Ecplicitly assign a variable type
    println!("x is : {}", x);

    {
        let x = 17; // This is called name shadowing: defining a variable name in an 
        // inner scope with the same variable name as the one on the outter scope
        println!("x is : {}", x);
    }

    {
        let y = x - 2; //accessing outter variable in the inner scope
        println!("x is : {}", y);
        x = y; // assigning value of inner variable to the outter variable
    }
    x = x + 1;
    println!("x is : {}", x);

    const SECONDS_IN_MINUTES: u32 = 60; //defining a constant in Rust
    println!("Seconds in a minutes is : {}", SECONDS_IN_MINUTES);
}

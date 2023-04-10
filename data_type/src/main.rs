fn main() {
    println!("Hello, world!");
    //scalar primitive data type
    let floating_point: f32 = 10.9;
    println!("floating_point: {}", floating_point);
    let true_or_false: bool = true;
    println!("true_or_false: {}", true_or_false);
    let letter: char = 'a';
    println!("letter: {}", letter);

    // compound primitive data type (Tuple, array). tuple are immutable by default
    // use the keyword "mut" to make it mutable
    //Tuple
    let mut tup1: (i32, bool, char) = (15, true, 'a');
    let tup2: (f32, bool, char) = (15.3, true, 'a');
    println!("{}", tup1.0);
    println!("{}", tup2.0);
    tup1.0 = 30;
    println!("{}", tup1.0);

    //Array
    let mut arr = [1, 2, 3, 4, 5];
    let arr1: [i32; 5] = [1, 2, 4, 8, 16];
    println!("Arry element in index 2 is : {}", arr[2]);
    arr[2] = 17;
    println!("Arry element in index 2 is : {}", arr[2]);
    println!("Arry1 element in index 2 is : {}", arr1[2]);
}

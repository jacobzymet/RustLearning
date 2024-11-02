fn main() {
    println!("Rust is statically typed, so you must declare the type of a variable when you declare it.");
    println!("-------------------------------------------------------");
    println!("let x: i32 = 5; // i32 is a 32-bit integer");
    println!("let y: f64 = 5.0; // f64 is a 64-bit floating point number");
    println!("let is_active: bool = true; // bool is a boolean type");
    println!("let letter: char = 'A'; // char is a character type");
    println!("-------------------------------------------------------");
    println!("This code uses declared variables:");
    continue_execution();
}

fn continue_execution() {
    let x: i32 = 5;
    let y: f64 = 5.0;
    let is_active: bool = true;
    let letter: char = 'A';
    let z: u8 = 255;
    println!("x = {} (i32)", x);
    println!("y = {} (f64)", y);
    println!("is_active = {} (bool)", is_active);
    println!("letter = {} (char)", letter);
}

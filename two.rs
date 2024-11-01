fn main() {
    println!("By default, variables in rust are immutable. This means they are constants and can't be changed later.");
    println!("Variables can be made mutable in Rust.");
    println!("-------------------------------------------------------");
    println!("let x = 5; // immutable");
    println!("let mut y = 5; // mutable");
    println!("-------------------------------------------------------");
    println!("This code uses declared variables:");
    continue_execution();
}

fn continue_execution() {
    let x = 5;
    let mut y = 5;
    println!("x = {} (Immutable/Constant)", x);
    println!("y = {} (Mutable, before change)", y);
    y = 6;
    println!("y = {} (Mutable, after change)", y);
}
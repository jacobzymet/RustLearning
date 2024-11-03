fn main() {
    println!("You can use a variable in a function to return a value from the function.");
    println!("Below are examples of functions that return values:");
    println!("-------------------------------------------------------");
    println!("");
    println!("fn main() {{");
    println!("    greet('Alice');");
    println!("");
    println!(r#"fn greet(name: &str) {{"#);
    println!("    println!(\"Hello, {{}}!\", name);");
    println!("}}");
    println!("");
    println!("-------------------------------------------------------");
    println!("Please also look at this code of this file to see the use of raw strings.");
}
fn main() {
    println!("There are more data types in Rust. All of the primary data types are outlined below:");
    println!("-------------------------------------------------------");
    println!("let x: i32 = 5; // i32 is a 32-bit integer");
    println!("let y: f64 = 5.0; // f64 is a 64-bit floating point number");
    println!("let is_active: bool = true; // bool is a boolean type");
    println!("let letter: char = 'A'; // char is a character type");
    println!("-------------------------------------------------------");
    println!("Another important thing is the different types of strings in Rust:");
    println!("");
    println!("let s1: &str = \"Hello, World!\"; // &str is a string slice");
    println!("let s2: String = String::from(\"Hello, World!\"); // String is a heap-allocated string");
    println!("The difference between &str and String is that &str is a string slice that points to a string in memory, while String is a heap-allocated string that can be modified.");
    println!("Use &str when you don't need to modify the string, and use String when you need to modify the string.");
}
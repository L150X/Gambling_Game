use colored::*; // Import colored crate (from cargo, e.g., cargo add colored)

/// Reads a line of input from the user, returning it as a string
fn read_line() -> String { // Return type is String

    // Create a mutable String to hold the input
    let mut input: String = String::new();

    std::io::stdin() 
        .read_line(&mut input)
        .expect("Failed to read line");
        input // Return input
}

/// Main Function
fn main() {
    let input: String = read_line(); // Call read_line and store the result in input
}


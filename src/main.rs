use colored::*; // Import colored crate (from cargo, e.g., cargo add colored)

// Use crossterm for terminal manipulation and styling, as similar to C# as possible
use crossterm::{
    execute, // Import execute function for terminal commands
    terminal::{Clear, ClearType}, // Import Clear and ClearType for clearing the terminal
    style::{Color, SetForegroundColor}, // Import Color and SetForegroundColor for styling text
};

// Use std::io for input and output operations
// Without this, it would need to be std::io::stdin/stdout() every time
use std::io::stdout;
use std::io::stdin; 

use std::fs; // Import fs for file functions

/// Reads a line of input from the user, returning it as a string
fn read_line() -> String { // Return type is String

    // Create a mutable String to hold the input
    let mut input: String = String::new();

    stdin() // Allows reading of keyboard input
        .read_line(&mut input) // Read a line of input into the mutable string by borrowing
        .expect("Failed to read line"); // Handle potential errors with expect

    // Trim empty space and newlines, convert it to a string, and return the input
    input.trim().to_string() 
}

/// Reads the art.txt to get title ASCII art
fn get_art(path: &str) -> String{

    // Read the contents of art.txt and put it into a string
    let art = fs::read_to_string(path)
        .expect(&format!("Failed to read {path}")); // Handle errors. Format macro builds new string

    // Return art
    art
}

/// Simply clears the terminal
fn clear_terminal(){
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

/// Main Function
fn main() {
    // Setup phase
    // Clear the terminal
    clear_terminal();

    // Get title art for later use
    let title_art = get_art("assets/art.txt");

    // Set terminal text color to dark blue
    execute!(stdout(), SetForegroundColor(Color::DarkBlue)).unwrap();

    // Output title
    println!("{title_art}");

    
    // Output list of what games the user could play
}


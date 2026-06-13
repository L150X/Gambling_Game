use colored::*; // Import colored crate (from cargo, e.g., cargo add colored)=
use std::fs; // Import fs for file functions
use rand::Rng; // RNG needed for gambling (cargo add rand)

// Use crossterm for terminal manipulation and styling, as similar to C# as possible
use crossterm::{
    cursor::MoveTo,                     // Import cursor setting
    execute,                            // Import execute function for terminal commands
    style::{Color, SetForegroundColor}, // Import Color and SetForegroundColor for styling text
    terminal::size,                     // Import code to find terminal size
    terminal::{Clear, ClearType},       // Import Clear and ClearType for clearing the terminal
};

// Use std::io for input and output operations
// Without this, it would need to be std::io::stdin/stdout() every time
use std::io::stdin;
use std::io::stdout;

// Import for thread.sleep equiv
use std::thread;
use std::time::Duration;

/// Reads a line of input from the user, returning it as a string
fn read_line() -> String {
    // Return type is String

    // Create a mutable String to hold the input
    let mut input: String = String::new();

    stdin() // Allows reading of keyboard input
        .read_line(&mut input) // Read a line of input into the mutable string by borrowing
        .expect("Failed to read line"); // Handle potential errors with expect

    // Trim empty space and newlines, convert it to a string, and return the input
    input.trim().to_string()
}

/// Reads a line and returns it as a integer. Loops if incorrect input. Upper and lower bound are inclusive.
fn get_int(lower_bound: i32, upper_bound: i32) -> i32 {
    // Loop around the asking and verification phase
    loop {
        let base_input = read_line(); // Read the user input

        // Convert the input to an integer safely
        let choice: i32 = match base_input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; // Skip rest of loop, go back to asking phase
            }
        };

        // Only continue if the choice is one of the available options
        if !(lower_bound..upper_bound + 1).contains(&choice) {
            println!("Please enter a valid number between {lower_bound} and {upper_bound}");
            continue; // Skip rest of loop, go back to asking phase
        }
        break choice; // Break the loop and return choice
    }
}

/// Reads the art.txt to get title ASCII art
fn get_art(path: &str) -> String {
    // Read the contents of art.txt and put it into a string
    let art = fs::read_to_string(path).expect(&format!("Failed to read {path}")); // Handle errors. Format macro builds new string
    art // Return art
}

/// Simply clears the terminal
fn clear_terminal() {
    // Clear the terminal
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).expect("Issue clearing terminal");
}

/// Outputs a screen with money information
fn output_screen(art_path: &str, extra_info: &str, money: f64) {
    // Set terminal text color to yellow
    execute!(stdout(), SetForegroundColor(Color::Yellow)).unwrap();

    // Clear the terminal
    clear_terminal();

    // Get and output title art
    line();
    let title_art = get_art(art_path);
    println!("{title_art}");
    line();

    // Output extra information
    println!("Cash on Hand: ${money}");
    line();
    println!("{extra_info}");
    line();
}

/// Output a line across the screen
fn line() {
    let (width, _) = size().unwrap(); // Set width equal to the width of the console
    println!("{}", "-".repeat(width as usize)); // Repeat repeats a string n times, so fills the screen
}

/// Main Function
fn main() {
    // Variable to store money, in a f64
    let mut money: f64 = 1000.0;

    // Loop around the asking
    loop {
        // Output title
        output_screen(
            "assets/title_art",
            "Welcome to the gambling games program!",
            money,
        );

        // Output list of what games the user could play
        println!("Which gambling game would you like to play?");
        let choices = ["Slot Machines", "Gambling Dice", "Guess the Dice"]; // Create an array of choices

        // For each choice, output it with formatting (eg., 1. Slot Machines)
        let mut count = 1;
        for choice in choices {
            println!("{count}. {choice}.");
            count += 1;
        }

        // Get user input
        line();
        println!("Enter a number corresponding to the game you want to play:");
        let choice = get_int(1, 3);

        // Call a different method based on the input.
        match choice {
            1 => slot_machines(money),
            2 => gambling_dice(money),
            3 => guess_the_dice(money),
            _ => unreachable!(), // Could not happen
        }
    }
}

/// Slot Machines game
fn slot_machines(money: f64) {
    //create rng variable
    let mut rng = rand::rng();
    let num = rand::random_range(1..=10);
    println!("{num}");
}

// Roulette game
fn gambling_dice(money: f64) {}

/// Guess the Dice game
fn guess_the_dice(money: f64) {}

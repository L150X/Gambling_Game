use colored::*; // Import colored crate (from cargo, e.g., cargo add colored)=
use std::fs; // Import fs for file functions
use rand::Rng; // RNG needed for gambling (cargo add rand)

// Use crossterm for terminal manipulation and styling, as similar to C# as possible
use crossterm::{
    cursor::MoveTo,                     // Import cursor setting
    cursor::MoveUp,                     // Moves cursor up
    execute,                            // Import execute function for terminal commands
    style::{Color, SetForegroundColor}, // Import Color and SetForegroundColor for styling text
    terminal::{Clear, ClearType, size},       // Import Clear and ClearType for clearing the terminal
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
        if !(lower_bound..upper_bound+1).contains(&choice) {
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
fn output_screen(art_path: &str, extra_info: &str, money: i32) {
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
    // Variable to store money, in a i32
    let mut money: i32 = 1000;

    // Loop around the asking
    loop {
        // Output title
        output_screen(
            "assets/title_art.txt",
            "Welcome to the gambling games program!",
            money,
        );

        // Output list of what games the user could play
        println!("Which gambling game would you like to play?");
        let choices = ["Slot Machines", "Gambling Dice", "Guess the Number"]; // Create an array of choices

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
            1 => money = slot_machines(money),
            2 => money = gambling_dice(money),
            3 => money = guess_the_number(money),
            _ => unreachable!(), // Could not happen
        }
    }
}

/// Slot Machines game
fn slot_machines(mut money: i32) -> i32{
    // Output the title for the guess the dice program
    output_screen("assets/guess_the_number_art.txt", "Guess the Number!", money);

    // Get a bet from the user
    println!("Enter a bet. Once you spin, if 2 symbols are the same your bet is doubled. If 3 are the same, you get x7 your bet.");
    let bet = get_int(1,money); // The bet cannot be higher than the current cash, and cannot be 0
    money -= bet; // Subtract the bet from money
    let mut guess_count = 0; // Guess count variable to store how many guesses have already been guessed

    // Set symbols vector
    let symbols = vec!["A", "B", "C", "D", "E"];
    let mut delay = 50; // Delay timing


    // Loop 20 times
    for i in 0..21{

        // Generate random symbols
        let s1 = symbols[rand::random_range(0..symbols.len())]; 
        let s2 = symbols[rand::random_range(0..symbols.len())];
        let s3 = symbols[rand::random_range(0..symbols.len())];

        // Output the machine
        println!("{}","╔══════════════╗".blue());
        println!(
            "{} {} {} {} {} {} {}",
            "║".blue(), 
            s1,
            "|".blue(), 
            s2,
            "|".blue(), 
            s3, 
            "║".blue());
        println!("{}","╚══════════════╝".blue());

        // Delay styling
        thread::sleep(Duration::from_millis(delay));
        
        // Clear the last 3 lines if not the last spin sequence
        if i != 20 {
        execute!(stdout(), MoveUp(3)).unwrap();
        execute!(stdout(), Clear(ClearType::FromCursorDown)).unwrap();
        }

        //Increase the delay
        delay +=20;

        // If it is the last spin sequence
        if i == 20{
            // Calculations and output for winning or not
            if s1 == s2 && s2 == s3 { // If all three are the same
            println!("JACKPOT!");
            money += bet * 7;
            }

            else if s1 == s2 || s2 == s3 || s1 == s3 { // If two are the same
            println!("Two matched!");
            money += bet * 2;
            }

            else { // If none are the same
            println!("No win!");
            }
       }
    }
    
    // Wait a little before going back to main program
    line();
    println!("Thanks for playing!");
    println!("Loading...");
    thread::sleep(Duration::from_secs(2));
    money // Return money
}

// Roulette game
fn gambling_dice(mut money: i32) -> i32{
    money
}

/// Guess the Dice game
fn guess_the_number(mut money: i32) -> i32 {
    // Output the title for the guess the dice program
    output_screen("assets/guess_the_number_art.txt", "Guess the Number!", money);

    // Get a bet from the user
    println!("Enter a bet. If you guess the number correctly on your first try, your money quardruples, if on your second, 
    it triples, and if on your third it multiplies by 2");
    let bet = get_int(1,money); // The bet cannot be higher than the current cash, and cannot be 0
    money -= bet; // Subtract the bet from money
    let mut guess_count = 0; // Guess count variable to store how many guesses have already been guessed

    // Create a random number between 1 and 10
    let num: i32 = rand::random_range(1..=10);

    // Loop around guessing
    loop {
        if guess_count == 3{
            println!("You lost! Sending back to main menu...");
            thread::sleep(Duration::from_millis(1500)); // Wait 1.5 seconds before proceeding
            break money; // Break the loop
        }

        // Increase guess count
        guess_count += 1;

        // Output screen
        output_screen("assets/guess_the_number_art.txt", "Guess the Number! The number is between 1 and 10 (inclusive).", money);
        println!("Guess a number. This is guess: {}", guess_count);


        // Get user's guess
        let guess: i32 = get_int(1, 10);

        // Compare, output, and give respective reward
        if guess == num{
            // Calculate winnings based on bet and guess count
            let mut winnings = 0;
            match guess_count{
                1 => winnings = bet*4,
                2 => winnings = bet*3,
                3 => winnings = bet*2,
                _ => unreachable!(),
            }
            println!("You won! ${winnings} has been added to your account!"); // Output the winnings
            money += winnings; // Add the winnings
            thread::sleep(Duration::from_secs(2)); // Wait two seconds before proceeding
            break money; // Break the loop
        }

        else if guess > num{ // If the guess was greater than the number
            println!("Wrong! Heres a hint: Your guess was greater than the number."); // Output that it was greater than the number
            println!("Refreshing.. Please wait..");
        }

        else{ // Else, guess must be less than the number
            println!("Wrong! Heres a hint: Your guess was less than the number."); // Output that it was less than the number
            println!("Refreshing.. Please wait..");
        }

        thread::sleep(Duration::from_secs(2)); // Wait two seconds before proceeding
    }
}

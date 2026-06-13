use colored::*; // Import colored crate (from cargo, e.g., cargo add colored)=
use std::fs; // Import fs for file functions

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

/// Waits for the user to press enter
fn wait_for_enter(){
    println!("Press enter to continue");
    read_line();
}

/// Main Function
fn main() {
    // Variable to store money, in a i32
    let mut money: i32 = 1000;

    // Loop around the asking
    loop {
            // Set terminal text color to red
            execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();

            // If money is greater than 0, continue as normal
            if money > 0{
            // Output title
            output_screen(
                "assets/title_art.txt",
                "Welcome to the gambling games program!",
                money,
            );
            }

            // Otherwise
            else{

            // Output a different screen
            output_screen(
                "assets/out_of_money_art.txt",
                "You ran out of cash...",
                money,
            );

            // Set terminal text color to white
            execute!(stdout(), SetForegroundColor(Color::White)).unwrap();

            // Output message saying they ran out of money and must withdraw or exit
            println!("Withdraw from the ATM? Enter 1 for yes, 2 for no");
            let choice: i32 = get_int(1, 2); // Get choice
            if choice == 1{ // If choice is to withdraw, output message and add money
                println!("You withdrew 1000 from the ATM");
                money += 1000;

                // Output title
                output_screen(
                "assets/title_art.txt",
                "Welcome to the gambling games program!",
                money,
            );
            }

            // Else, they have to leave the casino
            else { 
                println!("Well, then you have to leave the casino. Thanks for playing!");
                thread::sleep(Duration::from_millis(1500)); // Wait 1.5s before exiting
                break; // Exit
            }
            }

        // Set terminal text color to white
        execute!(stdout(), SetForegroundColor(Color::White)).unwrap();

        // Output list of what games the user could play
        println!("What would you like to do?");
        let choices = ["Slot Machines", "Gambling Dice", "Guess the Number", "Exit"]; // Create an array of choices

        // For each choice, output it with formatting (eg., 1. Slot Machines)
        let mut count = 1;
        for choice in choices {
            println!("{count}. {choice}.");
            count += 1;
        }

        // Get user input
        line();
        println!("Enter a number corresponding to the action you want to:");
        let choice = get_int(1, 4);

       
        match choice { // Call a different method based on the input.
            1 => money = slot_machines(money),
            2 => money = gambling_dice(money),
            3 => money = guess_the_number(money),
            4 => break, // Exit the terminal by breaking the loop
            _ => unreachable!(), // Could not happen
        }
        }
}

/// Slot Machines game
fn slot_machines(mut money: i32) -> i32{
    // Set terminal text color to blue
    execute!(stdout(), SetForegroundColor(Color::Blue)).unwrap();
    // Output the title for the guess the dice program
    output_screen("assets/slot_machines_art.txt", "The Slots", money);

    // Set terminal text color to white
    execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
    // Get a bet from the user
    println!("Enter a bet. Once you spin, if 2 symbols are the same your bet is doubled. If 3 are the same, you get x7 your bet.");
    let bet = get_int(1,money); // The bet cannot be higher than the current cash, and cannot be 0
    money -= bet; // Subtract the bet from money

    // Set terminal text color to blue
    execute!(stdout(), SetForegroundColor(Color::Blue)).unwrap();
    // Reoutput title
    output_screen("assets/slot_machines_art.txt", "The Slots", money);

    // Set terminal text color to white
    execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
    // Set symbols array
    let symbols = ["🍒", "🍋", "⭐", "💎", "7 "];
    let mut delay = 5; // Delay timing

    // Loop 30 times
    for i in 0..31{
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

        // Set terminal text color to white
        execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
        // Delay styling
        thread::sleep(Duration::from_millis(delay));
        
        // Clear the last 3 lines if not the last spin sequence
        if i != 30 {
        execute!(stdout(), MoveUp(3)).unwrap();
        execute!(stdout(), Clear(ClearType::FromCursorDown)).unwrap();
        }

        //Increase the delay
        delay +=20;

        // If it is the last spin sequence
        if i == 30{
            // Calculations and output for winning or not
            if s1 == s2 && s2 == s3 { // If all three are the same
            println!("JACKPOT! You got x7 your bet! ${}", bet*7);
            money += bet * 7;
            }

            else if s1 == s2 || s2 == s3 || s1 == s3 { // If two are the same
            println!("Two matched! You got x2 your bet! ${}", bet*2);
            money += bet * 2;
            }

            else { // If none are the same
            println!("No win! You lost your bet..");
            }
       }
    }
    
    // Wait a little before going back to main program
    line();
    println!("Thanks for playing!");
    line();
    wait_for_enter();
    money // Return money
}

// Gambling dice game
fn gambling_dice(mut money: i32) -> i32{
    // Set terminal text color to green
    execute!(stdout(), SetForegroundColor(Color::Green)).unwrap();

    // Output the title for the gambling dice program
    output_screen("assets/gambling_dice_art.txt", "Entrace to gambling dice.", money);

    // Set terminal text color to white
    execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
    // Get a bet from the user
    println!("Enter a bet. The CPU will bet the same amount. You both will then roll dice at the same time until one of your totals reaches 25 or above. Whoever wins will get the entire pot of money!");
    let bet = get_int(1,money); // The bet cannot be higher than the current cash, and cannot be 0
    money -= bet; // Subtract the bet from money

    // Create variables needed within the loop
    let mut player_total = 0;
    let mut cpu_total = 0;
    let mut round = 1;

    // Set terminal text color to green
    execute!(stdout(), SetForegroundColor(Color::Green)).unwrap();
    // Output the title for refresh
    output_screen("assets/gambling_dice_art.txt", "Entrace to gambling dice.", money);

    // Loop
    loop {
        // Generate random rolls
        let player_roll = rand::random_range(1..7); 
        let cpu_roll = rand::random_range(1..7); 

        // Add the rolls to the totals
        player_total += player_roll;
        cpu_total += cpu_roll;

        // Output board and results
        println!("╔══════════════════════════════╗");
        println!("║ ROUND {:<22}", round);            // Set it to 22 horizontal
        println!("╠══════════════════════════════╣");
        println!("║ Player Roll: {:<15}", player_roll);
        println!("║ CPU Roll:    {:<15}", cpu_roll);
        println!("╠══════════════════════════════╣");
        println!("║ Player Total:{:<15}", player_total);
        println!("║ CPU Total:   {:<15}", cpu_total);
        println!("╚══════════════════════════════╝");
        
        round += 1; // Increase round by 1

        // If any total is over or equal to 25, break the loop
        if player_total >= 25 || cpu_total >= 25{
            break;
        }

        // Output suspenseful rolling message (if the code reaches here, then there is no winner yet)
        line();
        wait_for_enter();
        
        // Clear the last 12 lines
        execute!(stdout(), MoveUp(12)).unwrap();
        execute!(stdout(), Clear(ClearType::FromCursorDown)).unwrap();
    }
    line();

    // If the player had a higher total, the user wins, add 2x the bet to money
    if player_total > cpu_total {
    println!("You win the pot!");
    money += bet * 2;
    }

    // If the cpu had a higher total, the cpu wins
    else if cpu_total > player_total {
    println!("The CPU wins the pot.");
    }

    // Else, it was a tie, and restore the bet to the user
    else {
    println!("Tie game!");
    money += bet;
    }

    // Enter press to continue
    line();
    wait_for_enter();

    // Return money
    money
}

/// Guess the Number game
fn guess_the_number(mut money: i32) -> i32 {
    // Set terminal text color to yellow
    execute!(stdout(), SetForegroundColor(Color::Yellow)).unwrap();
    // Output the title for the guess the number program
    output_screen("assets/guess_the_number_art.txt", "Guess the Number!", money);

    // Set terminal text color to white
    execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
    // Get a bet from the user
    println!("Enter a bet. If you guess the number correctly on your first try, your money quardruples, if on your second, it triples, and if on your third it multiplies by 2");
    let bet = get_int(1,money); // The bet cannot be higher than the current cash, and cannot be 0
    money -= bet; // Subtract the bet from money
    let mut guess_count = 0; // Guess count variable to store how many guesses have already been guessed

    // Create a random number between 1 and 10
    let num: i32 = rand::random_range(1..=10);

    // Loop around guessing
    loop {
        // If its the third guess
        if guess_count == 3{
            println!("You lost! Sending back to main menu...");
            thread::sleep(Duration::from_millis(1500)); // Wait 1.5 seconds before proceeding
            break money; // Break the loop
        }

        // Increase guess count
        guess_count += 1;

        // Set terminal text color to yellow
        execute!(stdout(), SetForegroundColor(Color::Yellow)).unwrap();
        // Output screen
        output_screen("assets/guess_the_number_art.txt", "Guess the Number! The number is between 1 and 10 (inclusive).", money);
        
        // Set terminal text color to white
         execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
        println!("Guess a number. This is guess: {}", guess_count);

        // Get user's guess
        let guess: i32 = get_int(1, 10);
        line();

        // Compare, output, and give respective reward
        if guess == num{
            // Calculate winnings based on bet and guess count
            let winnings;
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
            println!("Wrong! Your guess was greater than the number."); // Output that it was greater than the number
        }

        else{ // Else, guess must be less than the number
            println!("Wrong! Your guess was less than the number."); // Output that it was less than the number
        }

        line();
        wait_for_enter(); // Wait for enter
    }
}
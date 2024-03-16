use std::io;
use std::io::Write;

// Define the maximum number of incorrect guesses allowed
const MAX_WRONG_GUESSES: usize = 6;

fn main() {
    println!("Welcome to Hangman!");
    
    // Define the secret word
    let secret_word = "rust".to_lowercase(); // Can be changed
    
    let mut guessed_letters = vec!['_'; secret_word.len()];
    let mut wrong_guesses = 0;
    let mut guessed_letters_count = 0;

    loop {
        // Display current status of the game
        print_current_status(&guessed_letters, wrong_guesses);
        
        // Get user input
        let guess = get_guess();
        
        // Check if the guessed letter is correct
        let mut found = false;
        for (i, letter) in secret_word.chars().enumerate() {
            if letter == guess {
                guessed_letters[i] = letter;
                guessed_letters_count += 1;
                found = true;
            }
        }
        
        // If the guess is incorrect, increment wrong_guesses
        if !found {
            wrong_guesses += 1;
        }
        
        // Check if the player has won or lost
        if guessed_letters_count == secret_word.len() {
            println!("Congratulations! You guessed the word: {}", secret_word);
            break;
        } else if wrong_guesses == MAX_WRONG_GUESSES {
            println!("Sorry, you've run out of guesses. The word was: {}", secret_word);
            break;
        }
    }
}

fn print_current_status(guessed_letters: &Vec<char>, wrong_guesses: usize) {
    println!("\nWord: {}", guessed_letters.iter().collect::<String>());
    println!("Incorrect Guesses: {}/{}", wrong_guesses, MAX_WRONG_GUESSES);
    println!("Hangman: ");
    print_hangman(wrong_guesses);
}

fn get_guess() -> char {
    loop {
        print!("Enter your guess: ");
        io::stdout().flush().expect("Failed to flush stdout.");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        
        if let Some(c) = guess.trim().chars().next() {
            if c.is_alphabetic() {
                return c.to_lowercase().next().unwrap();
            }
        }
        
        println!("Please enter a single alphabetical character.");
    }
}

fn print_hangman(wrong_guesses: usize) {
    match wrong_guesses {
        0 => println!("  ____\n |    |\n |\n |\n |\n_|___"),
        1 => println!("  ____\n |    |\n |    O\n |\n |\n_|___"),
        2 => println!("  ____\n |    |\n |    O\n |    |\n |\n_|___"),
        3 => println!("  ____\n |    |\n |    O\n |   /|\n |\n_|___"),
        4 => println!("  ____\n |    |\n |    O\n |   /|\\\n |\n_|___"),
        5 => println!("  ____\n |    |\n |    O\n |   /|\\\n |   /\n_|___"),
        _ => println!("  ____\n |    |\n |    O\n |   /|\\\n |   / \\\n_|___"),
    }
}

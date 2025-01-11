use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

struct Difficulty {
    level: String,
    chances: u32,
}

fn wait_enter_difficulty() -> Difficulty {
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut level = String::new();
    io::stdin()
        .read_line(&mut level)
        .expect("Failed to read input");

    let level: u32 = match level.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice. Please try again.");
            return wait_enter_difficulty();
        }
    };

    match level {
        1 => {
            return Difficulty {
                level: String::from("Easy"),
                chances: 10,
            };
        }
        2 => {
            return Difficulty {
                level: String::from("Medium"),
                chances: 5,
            };
        }
        3 => {
            return Difficulty {
                level: String::from("Hard"),
                chances: 3,
            };
        }
        _ => {
            println!("Invalid choice. Please try again.");
            return wait_enter_difficulty();
        }
    }
}

fn play_game(difficulty: Difficulty) {
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 1;

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        // String to store user input
        let mut guess = String::new();

        // Read from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Convert input to an integer (loop back to the start on failure)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if (guess != secret_number) && ((difficulty.chances - attempts) == 0) {
            println!(
                "You have run out of chances. The correct number was {}.",
                secret_number
            );
            break;
        }

        // Compare the input with the correct number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Incorrect! The number is greater than {}.", guess),
            Ordering::Greater => println!("Incorrect! The number is less than {}.", guess),
            Ordering::Equal => {
                println!(
                    "Congratulations! You guessed the correct number in {} attempts.",
                    attempts
                );
                break;
            }
        }
        attempts += 1;
    }
}

fn wait_enter_retry() -> bool {
    print!("Do you want to play again? (y/n): ");
    io::stdout().flush().unwrap();

    let mut retry = String::new();
    io::stdin()
        .read_line(&mut retry)
        .expect("Failed to read input");

    match retry.trim() {
        "y" => true,
        "n" => false,
        _ => {
            println!("Invalid choice. Please try again.");
            return wait_enter_retry();
        }
    }
}

fn start_game() {
    println!(
        r#"
Please select the difficulty level:
1. Easy (10 chances)
2. Medium (5 chances)
3. Hard (3 chances)
"#
    );

    let difficulty = wait_enter_difficulty();
    println!(
        r#"
Great! You have selected the {} difficulty level.
You have {} chances to guess the correct number.
Let's start the game!
"#,
        difficulty.level, difficulty.chances
    );

    play_game(difficulty);
}

fn main() {
    println!(
        r#"
Welcome to the Number Guessing Game!
I'm thinking of a number between 1 and 100.
"#
    );

    loop {
        start_game();
        if wait_enter_retry() {
            continue;
        }
        break;
    }
    println!("Thank you for playing the Number Guessing Game!");
}

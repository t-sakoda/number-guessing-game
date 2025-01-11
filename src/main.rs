use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Write};

struct Difficulty {
    level: String,
    chances: u32,
}

struct HighScore {
    attempts: u32,
    duration: std::time::Duration,
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

fn play_game(difficulty: Difficulty, mut high_scores: HashMap<String, HighScore>) {
    // Start the timer
    let start = std::time::Instant::now();

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 1;

    loop {
        print!("Enter your guess ('h' for a hint): ");
        io::stdout().flush().unwrap();

        // String to store user input
        let mut guess = String::new();

        // Read from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        if guess.trim().eq("h") {
            println!(
                "Hint: The last digit of the secret number is {}.",
                secret_number % 10
            );
            continue;
        }

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
                let mut is_high_score_updated = false;
                if let Some(high_score) = high_scores.get(&difficulty.level) {
                    if attempts < high_score.attempts {
                        is_high_score_updated = true;
                    } else if attempts == high_score.attempts {
                        if start.elapsed() < high_score.duration {
                            is_high_score_updated = true;
                        }
                    }
                } else {
                    is_high_score_updated = true;
                }

                if is_high_score_updated {
                    println!("New high score!");
                    high_scores.insert(
                        difficulty.level.clone(),
                        HighScore {
                            attempts,
                            duration: start.elapsed(),
                        },
                    );
                }

                println!(
                    "Congratulations! You guessed the correct number in {} attempts.",
                    attempts
                );
                break;
            }
        }
        attempts += 1;
    }

    // Stop the timer
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
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

    let high_scores = HashMap::new();

    let difficulty = wait_enter_difficulty();
    println!(
        r#"
Great! You have selected the {} difficulty level.
You have {} chances to guess the correct number.
Let's start the game!
"#,
        difficulty.level, difficulty.chances
    );

    play_game(difficulty, high_scores);
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

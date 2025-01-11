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

fn main() {
    println!(
        r#"
Welcome to the Number Guessing Game!
I'm thinking of a number between 1 and 100.

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
"#,
        difficulty.level, difficulty.chances
    );

    // 1~100のランダムな数値を生成
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("予想する数字を入力してください:");

        // ユーザーの入力を格納するための文字列
        let mut guess = String::new();

        // 標準入力から読み取る
        io::stdin()
            .read_line(&mut guess)
            .expect("入力の読み取りに失敗しました");

        // 入力を整数に変換（失敗時はループの先頭へ）
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("有効な数字を入力してください。");
                continue;
            }
        };

        println!("あなたの予想: {}", guess);

        // 入力と正解を比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("もっと大きい数字です！"),
            Ordering::Greater => println!("もっと小さい数字です！"),
            Ordering::Equal => {
                println!("正解です！おめでとう！");
                break;
            }
        }
    }
}

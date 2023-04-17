use std::io;
mod word_list;

fn main() {
    println!("Welcome to Wordle-rs!");
    println!("You have six guesses, please enter a five letter word:");
    solver();
}

fn solver() {
    let words = word_list::WordList::new().expect("Couldn't get word-list");
    let solution = words.get_random_word().expect("Couldn't get random solution word from word-list");
    assert!(words.is_valid(solution));

    let mut ans_num = 0;
    while ans_num < 6 {
        let guess = get_stdio_line().expect("Couldn't read stdio");
        if !words.is_valid(&guess) {
            println!("invalid guess word: {}", guess);
            continue;
        }
        ans_num += 1;
        output_hint(solution, &guess);
        if guess == solution {
            println!("Congratulations!");
            return;
        }
    }
    println!("game over, the answer was {}", solution);
}

fn get_stdio_line() -> Result<String, Box<dyn std::error::Error>> {
    let mut word = String::new();
    io::stdin().read_line(&mut word)?;
    let word = word.to_lowercase();
    let word = word.trim();
    Ok(word.to_string())
}

fn output_hint(solution: &str, guess: &str) {
    let solution = solution.as_bytes();
    for (i, c) in guess.bytes().enumerate() {
        if c == solution[i] { print!("🟩"); }
        else if solution.contains(&c) { print!("🟨"); }
        else { print!("⬛"); }
    }
    println!();
}
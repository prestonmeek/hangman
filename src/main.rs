use random_word::gen_len;

fn main() {
    const WORD_LENGTH: usize = 5;
    
    let word = match gen_len(WORD_LENGTH) {
        None => panic!("Could not generate word"),
        Some(word) => word
    };

    let mut guess_template: [char; WORD_LENGTH] = ['_'; WORD_LENGTH];

    const MAX_STRIKES: usize = 5;
    let mut strikes: usize = 0;

    let mut previous_guesses: Vec<char> = Vec::new();

    loop {
        println!("{:?}", guess_template);

        println!("Guess a letter!");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<char>() {
            Err(_) => {
                println!("Please enter a single letter");
                continue;
            },
            Ok(guess) => {
                if previous_guesses.contains(&guess) {
                    println!("You already guessed that letter");
                    continue;
                }

                previous_guesses.push(guess);
                
                guess
            }
        };
        
        let mut char_locations = word
            .match_indices(guess)
            .map(|(i, _)| i)
            .peekable();

        if char_locations.peek().is_some() {
            println!("Correct!");

            for idx in char_locations {
                guess_template[idx] = guess;
            }

            if !guess_template.contains(&'_') {
                println!("You win! The word was {}", word);
                break;
            }
        } else {
            strikes += 1;
            println!("Incorrect! Strike {}/{}", strikes, MAX_STRIKES);

            if strikes >= MAX_STRIKES {
                println!("You lose! The word was {}", word);
                break;
            }
        }
    }
}

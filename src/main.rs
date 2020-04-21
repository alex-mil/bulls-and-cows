extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn main() {
    let mut player_guess = vec![];
    let mut input = String::new();
    let mut game = Game::new();

    'start: loop {
        println!("Bulls & Cows Game");
        println!("Enter 4 digits positive number");
        print!("> ");
        stdout().flush().unwrap();
        input.clear();
        player_guess.clear();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read your guess");

        if input.trim() == ".exit" {
            break;
        }

        for ch in input.trim().chars() {
            if ch.is_digit(10) {
                player_guess.push(ch.to_digit(10).unwrap() as u8);
            } else {
                continue 'start;
            }
        }

        game.check(&player_guess);
        game.show_current_state();
    }
}

struct Code;

impl Code {
    fn generate() -> HashMap<u8, usize> {
        let mut rng = rand::thread_rng();
        let mut map = HashMap::<u8, usize>::new();
        while map.len() < 4 {
            let number = rng.gen_range(1, 9);
            if map.contains_key(&number) {
                continue;
            }
            map.insert(number, map.len());
        }
        map
    }
}

struct Game {
    bulls: u8,
    cows: u8,
    tries: u8,
    code: HashMap<u8, usize>,
}

impl Game {
    fn new() -> Self {
        Game {
            bulls: 0,
            cows: 0,
            tries: 0,
            code: Code::generate(),
        }
    }

    fn check(&mut self, player_nums: &[u8]) {
        self.bulls = 0;
        self.cows = 0;
        for (i, num) in player_nums.iter().enumerate() {
            if let Some(position) = self.code.get(num) {
                match position.cmp(&i) {
                    Ordering::Equal => self.bulls += 1,
                    _ => self.cows += 1,
                }
            }
        }
        self.tries += 1;
    }

    fn show_current_state(&self) {
        if self.bulls == 4 {
            println!("We have a winner! You have successfully cracked the code.");
            exit(0);
        }

        println!(
            "{} bulls, {} cows, {} tries",
            self.bulls, self.cows, self.tries
        );
    }

    #[allow(dead_code)]
    fn reset_stats(&mut self) {
        self.bulls = 0;
        self.cows = 0;
        self.tries = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn check_player_guess() {
        let mut game = Game::new();
        let mut guess: Vec<u8> = vec![0; 4];
        for (value, pos) in &game.code {
            guess[*pos] = *value;
        }

        game.check(&guess);
        assert_eq!(game.bulls, 4);

        guess.swap(1, 2);
        game.reset_stats();
        game.check(&guess);
        assert_eq!(game.bulls, 2);
        assert_eq!(game.cows, 2);
    }
}

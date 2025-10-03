use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use rand::{Rng, rngs::ThreadRng};

use crate::collatz::Collatz;

pub struct CollatzGame {
    rng: ThreadRng,
    collatz: Collatz,
}

impl Default for CollatzGame {
    fn default() -> Self {
        Self {
            rng: rand::rng(),
            collatz: Collatz,
        }
    }
}

impl CollatzGame {
    pub fn play(&mut self) -> io::Result<bool> {
        loop {
            println!("Enter a number:");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let mut x = match guess.trim().parse::<u64>() {
                Ok(number) => number,
                Err(_) => {
                    println!("Wrong format of a number!");
                    break Ok(true);
                }
            };

            if x == 0 {
                println!("You entered 0, finishing execution...");
                break Ok(false);
            }

            println!("You entered: {x}");

            let increment: u64 = self.rng.random_range(0..=5);

            x += increment;

            println!("Incremented value: {x}");

            let powers = self.get_powers(x, 10);
            println!("Generated powers: {powers:?}");

            let results = self.collatz.verify_many(&powers, 100);
            println!("Collatz results: {results:?}");

            match self.save_to_file(&results) {
                Ok(()) => {
                    println!("Results saved to xyz.txt");
                }
                Err(e) => {
                    println!("File save error: {e}");
                    break Ok(true);
                }
            }
        }
    }

    fn get_powers(&self, value: u64, size: usize) -> Vec<u64> {
        let mut powers = Vec::with_capacity(size);
        let mut current = value;
        for _ in 0..size {
            powers.push(current);
            current *= value;
        }
        powers
    }

    fn save_to_file(&self, values: &Vec<bool>) -> io::Result<()> {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open("xyz.txt")?;

        writeln!(f, "{values:?}")?;
        Ok(())
    }
}

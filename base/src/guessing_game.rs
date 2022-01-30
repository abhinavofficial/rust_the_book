pub mod guessing_game {
    use core::panicking::panic;
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;
    struct Guess {
        value: i32,
    }

    impl Guess {
        fn new(value: i32) -> Guess {
            if value < 0 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value}
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    pub fn play() {
        println!("Guess a number between 1 to 100. I have selected a secret number. If you guess it right, you win. Else keep trying!!!");
        //The Rng trait defines methods that random
        // number generators implement
        let secret_number = rand::thread_rng().gen_range(1..101);
        //println!("The secret number is: {}", secret_number);
        let mut trial = 0;
        loop {
            println!("Please input your guess");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

             match guess.trim().parse() {
                Ok(num) => Guess::new(num),
                Err(_) => {
                    println!("No hanky panky");
                    continue;
                }
            };
            trial += 1;

            println!(
                "You guessed: {}. You have already tried {} times",
                guess, trial
            );

            match Guess::value(& Guess { 10 }).cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}

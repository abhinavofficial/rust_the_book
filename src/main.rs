mod guessing_game;
mod ownership;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let learning_module = &args[1];
    let learning_module = match learning_module.trim() {
        "0" => "Introduction",
        "1" => "Guessing Game",
        "2" => "Ownership",
        _ => "Invalid module",
    };
    println!("You are now learning module: {}", learning_module);

    match learning_module {
        "Introduction" => println!("Hello, world"),
        "Guessing Game" => guessing_game::guessing_game::play(),
        "Ownership" => {
            let which_word = [0, 1, 2, 3, 4, 5, 6, 7];
            let word_be = "Hello this is a test".to_string();
            for i in which_word.iter() {
                let word_is = ownership::ownership::string_slicing(*i as u8, &word_be).trim();
                println!("The {:?} word is {}", i, word_is);
            }
        }
        _ => (),
    };
}

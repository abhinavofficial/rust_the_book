mod guessing_game;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let learning_module = &args[1];
    let learning_module = match learning_module.trim() {
        "0" => "Introduction",
        "1" => "Guessing Game",
        _ => "Invalid module",
    };
    println!("You are now learning module: {}", learning_module);

    match learning_module {
      "Guessing Game" => guessing_game::guessing_game::play(),
        _ => (),
    };

}


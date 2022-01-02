mod guessing_game;
mod ownership;
mod rectangle_area_calculator;
mod playing_with_enum;
mod utility_functions;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let learning_module = &args[1];
    let learning_module = match learning_module.trim() {
        "0" => "Introduction",
        "1" => "Guessing Game",
        "2" => "Ownership",
        "3" => "Area Calculator",
        _ => "Invalid module",
    };
    println!("You are now learning module: {}", learning_module);

    match learning_module {
        "Introduction" => println!("Hello, world"),
        "Guessing Game" => guessing_game::guessing_game::play(),
        "Ownership" => {
            let which_word = vec![0, 1, 2, 3, 4, 5, 6, 7];
            let word_be = String::from("Hello this is a test");
            for i in &which_word {
                let word_is = ownership::ownership::string_slicing(*i, &word_be).trim();
                println!("The {:?} word is {}", i, word_is);
            }
        },
        "Area Calculator" => rectangle_area_calculator::rectangle_area_calculator::calculate(),
        "Invalid module" => println!("You opted for invalid module"),
        _ => (),
    };

    // Task for 3-Jan-2021
    // Exercise 1: Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the
    // middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    //
    // Exercise 2: Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so
    // “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
    // Keep in mind the details about UTF-8 encoding!
    //
    // Exercise 3: Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in
    // a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
    // department or all people in the company by department, sorted alphabetically.
}

pub mod utility_function {
    use std::io;

    enum InputType {
        Character,
        Numeric,
        Calendar,
    }
    /*pub fn take_input(flash_line: &str, inputType: InputType) -> String {
        println!("{}",flash_line);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let _bool = match inputType {
            InputType::Character => input,
            InputType::Numeric => {
                match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid");
                        String::from("")
                    }
                }
            },
            //InputType::date => String::from(""),
            _ => String::from("Undefined Input Type"),
        };
        String::from("")
    }*/
}
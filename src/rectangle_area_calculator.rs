pub mod rectangle_area_calculator {
    use std::io;

    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        breadth: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.length * self.breadth
        }
    }
    fn take_input() -> Rectangle {
        println!("Rectangle area Calculator");
        let mut rectangle = Rectangle{
            length: 0,
            breadth: 0,
        };
        let mut length = String::new();
        loop {
            if rectangle.length == 0 {
                println!("Please enter length of rectangle:");
                length.clear(); // Ensure that length is cleared
                io::stdin().read_line(&mut length).expect("Failed to read line");

                rectangle.length = match length.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid entry. Try again !!!");
                        continue;
                    }
                };
            }
            println!("Please enter breadth of rectangle:");
            length.clear(); // Ensure that length is cleared
            io::stdin().read_line(&mut length).expect("Failed to read line");

            rectangle.breadth = match length.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid entry. Try again.");
                    continue;
                }
            };
            if rectangle.breadth != 0 {
                break;
            }
        }
        rectangle
    }

    pub fn calculate() {
        let rectangle = take_input();
        println!("Rectangle is: {:#?}", rectangle);
        dbg!(&rectangle);
        println!("The area of rectangle is: {}", dbg!(rectangle.area()));
    }
}
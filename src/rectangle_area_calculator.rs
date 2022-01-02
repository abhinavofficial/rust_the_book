pub mod rectangle_area_calculator {
    use std::io;

    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        breadth: u32,
    }
    // There can be one or more impl block. You can break these impl to each have one function. It does not make sense here but is valid.
    impl Rectangle {
        // All functions defined within an impl block are called associated functions
        // because they’re associated with the type named (Rectangle, in this case) after the impl.
        fn area(&self) -> u32 {
            self.length * self.breadth
        }
        fn is_valid(&self) -> bool {
            self.breadth > 0 && self.length >0
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.breadth > other.breadth && self.length > other.length
        }
        //We can define associated functions that don’t have self as their first parameter (and thus are not methods)
        // because they don’t need an instance of the type to work with. This can be used as constructor and are called with ::
        // Example - Rectangle::square(3)
        fn square(size: u32) -> Rectangle{
            Rectangle{
                breadth: size,
                length: size,
            }
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
            if rectangle.is_valid() {
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
pub mod error_handling {
    use std::error::Error;
    use std::fs::File;
    use std::io::{self, ErrorKind, Read};

    pub fn file_error_handling() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };
    }
    // to revisit post closure understanding
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro for us.

    //Another method, expect, which is similar to unwrap, lets us also choose the
    // panic! error message. Using expect instead of unwrap and providing good error
    // messages can convey your intent and make tracking down the source of a panic
    // easier.
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // expect parameter is the error message used by expect in its call to panic!
    pub fn smarter_file_error_handling() {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    //Sometime it makes to return the error so client can handle more effectively
    pub fn error_propagation() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    //Short cut for propagating errors: the ? operator
    //There is a difference between what the match expression from Listing 9-6
    // does and what the ? operator does: error values that have the ? operator
    // called on them go through the from function, defined in the From trait in
    // the standard library, which is used to convert errors from one type into another.
    // When the ? operator calls the from function, the error type received is
    // converted into the error type defined in the return type of the current
    // function. This is useful when a function returns one error type to represent
    // all the ways a function might fail, even if parts might fail for many
    // different reasons. As long as each error type implements the from function
    // to define how to convert itself to the returned error type, the ? operator
    // takes care of the conversion automatically.
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    //Further code reduction - chaining the ? operator
    fn read_username_from_file_chained() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    // It looks clear that you cannot use ? operation with null return type. For
    // special cases, like main function, it can have Result<(), Box<dyn Error>> as
    // return type. So, the below is a perfectly valid use case.
    // TODO: Will come back to this when we read Chapter 17. Using trait
    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
    //Of course, we can use std::fs::read_to_string function as well.
    // But it does not give us an opportunity to explain all the error handling



}
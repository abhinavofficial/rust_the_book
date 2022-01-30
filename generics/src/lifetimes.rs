pub mod life_times {
    // Main aim to lifetimes is to prevent dangling points, which cause
    // a program to reference data other than the data it's intended to
    // reference. Rust compiler uses burrow checker which tracks the scope of each variable. Rust ensures that variables in outer scope gets larger lifetimes than those in its inner. It rejects the code if it does not happen. Below code will fail (commented to ensure compilation for other sections)
    /*fn arbitrary_function() -> () {
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r: {}", r);
    }*/
    // Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.
    // The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. These relationships are what we want Rust to use when analyzing this code.
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function. Consider this attempted implementation of the longest function that won’t compile: Commenting the code.
    /*pub fn longest<'a>(x: &str, y: &str) -> &'a str {
         let result = String::from("really long string");
         result.as_str()
    }*/

}

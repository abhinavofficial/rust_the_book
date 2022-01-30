pub mod generics {
    use std::fmt::Display;

    // We can use Trait Bounds to Conditionally Implement Methods. See the
// implementation below for Pair
    pub struct Pair<T> {
        x: T,
        y: T,
    }

    // Below - always implements the new function to return a new instance of Pair<T>
    impl<T> Pair<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Pair<T> only implements the cmp_display method if its inner type T
// implements the PartialOrd trait that enables comparison and the
// Display trait that enables printing.
    impl<T: Display + PartialOrd> Pair<T> {
        pub fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // We can have multiple generic types ( T and U can be same when calling)
    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }

    impl<T, U> Point<T, U> {
        pub fn x(&self) -> &T {
            &self.x
        }
        // You can two points with different datatypes and working together
        pub fn mix_up<T1, U1>(self, other: Point<T1, U1>) -> Point<T, U1> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    /// The other option we have is defining methods on the type
    /// with some constraint on the generic type. We could, for
    /// example, implement methods only on Point<f32> instances
    /// rather than on Point<T> instances with any generic type.
    /// In Listing 10-10 we use the concrete type f32, meaning we
    /// don’t declare any types after impl
    impl Point<f64, f64> {
        pub(crate) fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    /// We could pass Vec<i32> as well. But smarter way would be pass the slice
    /// instead.
    pub fn largest<T: PartialOrd + Display>(list: &Vec<T>) -> &T {
        let mut largest = &list[0];
        for number in list {
            if number > largest {
                largest = number;
            }
        }
        //showing error in IntelliJ. It is not really an error. New feature in 1.58
        println!("The largest number is {largest}");
        largest
    }

// We can also conditionally implement a trait for any type that implements
// another trait. Implementations of a trait on any type that satisfies the
// trait bounds are called blanket implementations and are extensively used
// in the Rust standard library. For example, the standard library implements
// the ToString trait on any type that implements the Display trait.
// The impl block in the standard library looks similar to this code:

// impl<T: Display> ToString for T {
// --snip--
// }
// Because the standard library has this blanket implementation, we can call
// the to_string method defined by the ToString trait on any type that
// implements the Display trait. For example, we can turn integers into
// their corresponding String values like this because integers implement
// Display: let s = 3.to_string();
// Blanket implementations appear in the documentation for the trait in
// the “Implementors” section.
}
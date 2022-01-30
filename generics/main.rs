mod generics;
mod lifetimes;

use ::generics::{Summary, Tweet};
use crate::generics::generics::{largest, Point};
use crate::lifetimes::life_times::longest;

fn main() {
    // Generics
    let number_list = vec![34, 50, 25, 100, 655, 12, 56, 9898, 90, 200, 2345, 039, 423, 1232, 23, 2322, 2354, 5635, 54523, 3121, 3454211, 2443, 2342];
    let _result = largest(&number_list);
    let char_list = vec!['a', 'm', 't', 'b', 'k'];
    let _result = largest(&char_list);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 10.52, y: 13.53 };
    let point_distance = float_point.distance_from_origin();
    println!("The distance between points: {point_distance}");
    let mixed_point = Point { x: 5, y: 13.5 };
    let arbitrary_point = Point { x: "Hello", y: 'c' };

    println!("The value of x in different Points are {}, {} and {}",
             integer_point.x(), float_point.x(), mixed_point.x());

    let p3 = mixed_point.mix_up(arbitrary_point);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Traits
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    //Lifetime
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

}

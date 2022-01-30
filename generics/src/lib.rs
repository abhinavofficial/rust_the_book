pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// The impl Trait syntax works for straightforward cases but is actually
/// syntax sugar for a longer form, which is called a trait bound; it looks
/// like this:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// We can return types that implements a particular trait. Example, By using
// impl Summary for the return type, we specify that the returns_summarizable
// function returns some type that implements the Summary trait without naming
// the concrete type. In this case, returns_summarizable returns a Tweet, but
// the code calling this function doesn't know that.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
//The ability to return a type that is only specified by the trait it implements
// is especially useful in the context of closures and iterators, which we
// cover in Chapter 13. Closures and iterators create types that only the
// compiler knows or types that are very long to specify. The impl Trait
// syntax lets you concisely specify that a function returns some type that
// implements the Iterator trait without needing to write out a very long type.
//
// However, you can only use impl Trait if you’re returning a single type. The
// below code therefore does not work. Commenting
/*fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}*/
//Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions
// around how the impl Trait syntax is implemented in the compiler.
// We’ll cover how to write a function with this behavior in the
// “Using Trait Objects That Allow for Values of Different Types” section
// of Chapter 17. TODO
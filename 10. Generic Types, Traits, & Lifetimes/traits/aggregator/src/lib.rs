
pub mod summary;
pub use summary::Summary; // publish this
use std::fmt::Display;
use std::fmt::Debug;

pub struct NewsArticle {
    pub headlines: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headlines, self.author, self.location)
    }
    // notice we don't implement the default summarize function
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// traits as parameters
pub fn notify(item: &impl Summary) {
    println!("wow {}", item.summarize())
}

// that impl trait is actually syntax sugar of this:
// this is called bound trait
// pub fn notify<T: Summary>(item: &T) {
//     println!("wow {}", item.summarize());
// }

// also the bound trait is useful if we want to have different types for parameters
// pub fn notify<T: Summary>(item1: &T, item2: &T) {


// multiple trait bounds
// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {

// using where clause for clearer code
// from this:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// into this:
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{return 0}

// returning type with trait
// but this required to return only single type
pub fn return_summarizeable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// this means that implement display if T has PartialOrd trait
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementation
// implement trait only if T has trait Display
impl<T: Display> Summary for T {
    fn summarize(&self) -> String {
        String::from("boom")
    }
}


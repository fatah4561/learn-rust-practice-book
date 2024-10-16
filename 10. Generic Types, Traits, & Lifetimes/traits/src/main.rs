use aggregator::{notify, return_summarizeable, some_function, Summary, Tweet};

fn main() {
    let tweet = Tweet{
        username: "Fatah".to_string(),
        content: String::from("lorem ipsum ....... ... ... .. . .. .."),
        reply: false,
        retweet: false,
    };

    println!("u got 1 new tweet '{}'", tweet.summarize());
    println!("default summary {}", tweet.summarize_with_default());
    println!("default that call other {}", tweet.summarize_with_other());
    notify(&tweet);
    some_function(&3, &3);
    let _summarizeable = return_summarizeable(); // the type is impl Summary
}

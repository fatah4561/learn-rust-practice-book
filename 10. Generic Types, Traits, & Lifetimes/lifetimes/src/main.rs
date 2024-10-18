use std::fmt::Display;

// struct with references field need to have lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);

    // lifetime annotation
    // in summary lifetime annotation is to specify scope
    // &i32
    // &'a i32
    // &'a mut i32

    // this works
    let string = String::from("longest string ever");
    {
        let string2 = String::from("xyz");
        let result = longest(string.as_str(), string2.as_str());
        println!("the longest string is {result}");
    }

    // but not this
    // we will get string2 does not live long enough
    // because result is borrowing string2 but string2 goes out of scope
    // in the print statement
    // let string = String::from("longest string ever");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string.as_str(), string2.as_str());
    // }
    // println!("the longest string is {result}");

    let novel = String::from("lord of the rings. paw paw paw");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime will live till the program end
    // (all string literals have static lifetime)
    let s: &'static str = "I have a static lifetime.";

}

// need to specify that x and y is both has the same lifetime
// also the returned lifetime
// fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// this won't need to specify y lifetime
// since the returned value is always x with lifetime 'a
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// this is invalid since the returned value must have
// the same lifetime as one of the parameters
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// function with reference but without lifetime annotations
// it is called lifetime ellision
// well they said it is historical so the lifetime is inferred
// while the code is supposed to be like this
// fn first_word<'a>(s: &'a str) -> &'a str {
// why that ? it is because the func only has one parameter 
// and returning the same reference
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// functions that implement type parameters, trait bounds, and lifetime:
// also notes since lifetimes are a type of generic it also goes
// to the angle bracket <>
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
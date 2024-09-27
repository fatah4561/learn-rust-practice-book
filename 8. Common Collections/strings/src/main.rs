use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut s = String::new();
    s = "some strings".to_string(); 

    let mut s2 = "initial strings"; // string slice &str
    let s3 = s2.to_string();

    println!("{s} ,{s2}, {s3}");

    // String::form(); and to_string() are the same thing
    let mut s = String::from("hello");
    println!("{s}");

    s = s + " world";
    println!("{s}");

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = ss1 + &ss2;
    // println!("{ss1}, {ss2}, {ss3}"); // this will error if ss1 is moved
    println!("{ss2}, {ss3}");

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");

    // let ttt = ss1 + "-" + &ss2 + "-" + &ss3; // disable this because below we will reused ss1
    // println!("{ttt}");

    // better string concat
    let ttt = format!("{}-{}-{}", ss1, ss2, ss3); // oh and also format! doesn't take ownership of any string
    println!("{ttt}");

    // iterating strings
    for c in "Зд".chars() { // by char
        println!("char is {c}");
    }

    for c in "Зд".bytes() { // by byte
        println!("byte is {c}");
    }

    for c in "Зд".graphemes(true) { // by graphemes (require dependency)
        println!("graph is {c}");
    }

}


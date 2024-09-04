// some summary 
// expressions return a value ex 1 + 2 = 3, function calls, macros, code block like {...}
// statements don't return a value, example variable binding let y = 6; 1 + 2; (has semicolons)

fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(4450, 'g');

    let y = 6; // statement

    // statement doesn't returned a value so we can't use statement to bind another statement
    // let x = (let y = 6);
    // let x = let y = 6;
    // ^^ Syntax Error: `let` expressions are not supported here

    let y = { // i32 
        let x = 3;
        x + 1 // expression doesn't include ;
    };
    println!("The value of y is: {y}");

    let _y = { // the type become unit ()
        let x = 3;
        x + 1; // become statement (also give warning)
    };
    // println!("The value of y is: {y}");
    // ^ unit doesn't implement the `fmt::Display` trait

    println!("The value of five() is: {}", plus_one(five()));
}

fn another_function(x: i32) { // func definitions are also statement
    println!("Another function. parameter: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}

// return value fn
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // becomes unit
}
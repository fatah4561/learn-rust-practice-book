fn main() {
    let mut x = 5;
    println!("The value x is {x}");
    x = 6;
    // ^compile time error if x is not mutable
    println!("The value x is {x}");

    const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60; // global scope

    // shadowing
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}"); // be 14
    }
    println!("The value of x is {x}"); // still 7

    // unlike mut shadowing can change the variable types
    let spaces = "    ";
    let spaces = spaces.len();
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // ^compile time error
    println!("The value of spaces is {spaces}");
}

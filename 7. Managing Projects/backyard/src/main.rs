use crate::garden::vegetables::Asparagus;
use restaurant; // this is from lib not create root so no crate
// use restaurant::eat_at_restaurant; // this is unidiomatic since eat_at_restaurant will seems like it's local function

// while struct, enums and other items is idiomatic using full path
use std::collections::HashMap;

// also notes
// Bringing two types with the same name into the same scope requires using their parent modules.
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// there is also a solution to use `as` keyword for the above problem

use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;


// use std::io; // this first statement can written as self when merged 
// use std::io::Write;

use std::io::{self, Write};

use rand::Rng;
pub mod garden;

// this file is also known as crate root along the src/lib.rs
fn main() {
    let plant = Asparagus {};
    println!("this is plant, {plant:?}!");

    restaurant::eat_at_restaurant();

    println!("random number: {}", rand::thread_rng().gen_range(1..=100));
}

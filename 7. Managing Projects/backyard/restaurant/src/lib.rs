// this is library crate also known as root crate
// examples of module with sub modules in brackets

// The mod keyword declares modules, and Rust looks in a file with the same name as 
// the module for the code that goes into that module.
mod front_of_house;

// this stay private because while the add_to_waitlist is public and call this method
// the main.rs doesn't directly call to this method
fn deliver_order() {
    println!("deliver order");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // using super to call parent module
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // since the breakfast struct has private fields we need have constructor
        // function
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // while enum will have all of it's variants public if set public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub mod point {
        #[derive(Debug)]
        pub struct Point(i32, i32); // unnamed struct field also need to be set public for outside changes
        impl Point {
            pub fn origin() -> Self {
                Point(0, 0)
            }
        }
    }
}

// when using use, it's not public in that scope,
// also called as re-export
pub use crate::front_of_house::hosting; 

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // add to waitlist must be put as public function
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // this give out error
    // let mut meal2 = back_of_house::Breakfast{toast: String::from("Rye"), seasonal_fruit: String::from("blueberries")};

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // -- use keyword
    // note use only create shortcut for the scope it occurs in
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
    
}

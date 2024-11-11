use core::panic;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width < other.width && self.height > other.height
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2 // + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
    // String::from("Hello")
}

pub struct Guess {
    number: i32
}

impl Guess {
    pub fn new(number: i32) -> Guess {
        // if number < 0 || number > 100 {
        //     panic!("paw paw not between 0-100");
        // }
        if number <0 {
            panic!("Guess value must be greater than or equal 0, current value is {number}")
        } else if number > 100 {
            panic!("Guess value must be less than or equal to 100, current value is {number}")
        }
        Guess { number }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[should_panic] // can't use should panic on manual Result<>
    fn it_works_2() -> Result<(), String> {
        let result = add(2,2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("paw paw not correct"))
        }
    }

    #[test]
    #[should_panic] // must panic from the test result
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected="less than or equal to 100")] // with expectation of panic mesage
    fn guess_with_expected() {
        Guess::new(200);
        // Guess::new(-1);
    }

    #[test]
    fn greeting_contains_name() {
        let name = greeting("fatah");
        assert!(
            name.contains("fatah"),
            "greeting didn't contain name, value was {name}" // with formatting
        )
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
        assert_eq!(add_two(4), 6);
    }

    #[test]
    fn explore() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("paw paw")
        assert!(true)
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 3,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(!smaller.can_hold(&larger)); // negate it with !
    }
}

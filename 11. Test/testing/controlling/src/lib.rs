fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)] // for unit test
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        // this won't print
        // try to use `cargo test -- --show-output`
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(value, 5);
    // }

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    #[ignore] // can only be run when specified ex `cargo test -- --ignored`, ex `cargo test -- --ignored expensive`
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn test_the_logger() { /* ... */
    }
    #[test]
    fn test_the_database() { /* ... */
    }
    #[test]
    fn test_logger_and_database() { /* ... */
    }
}

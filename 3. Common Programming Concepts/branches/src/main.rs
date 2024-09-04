fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;
    // if number {
    //     println!("number was three");
    // }
    // ^ expected bool, found integer

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // some note even though 6 is divisible by 2 rust doesn't get there
    // because the 2nd arm is checked first
    // tips too many else if will clutter code and make it hard to read
    // refactor it (i prefer match / switch case)

    // if is expression so:
    let condition = true;
    let number = if condition { 5 } else { 6 }; // must same type
                                                // let number = if condition {5} else {"six"};
    println!("The value of number is: {}", number);

    // loop {
    //     println!("again!");
    // }

    // returning values from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            //  After the loop, we use a semicolon to end the statement that assigns the value to result
        }
        /* Note: the semicolon after break counter * 2 is technically optional.
        break is very similar to return, in that both can optionally take an expression as an argument,
        both cause a change in control flow. Code after a break or return is never executed,
        so the Rust compiler treats a break expression and a return expression as having the value unit, or (). */
    };
    println!("result is {}", result);

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // inner break only
            }
            if count == 2 {
                break 'counting_up; // label break
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // using while to loop array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // notes use for instead of while (because if we changed the array and forgot to update the index
    // we could get error)

    println!("for loop start here");
    for element in a {
        println!("the value is: {}", element);
    }

    // countdown for loop
    for number in (1..4).rev() {
        // .rev is to reverse the numbers (also use .=4 if 4 want to be included)
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // TODO: if i am free
    /*
    Convert temperatures between Fahrenheit and Celsius.
    Generate the nth Fibonacci number.
    Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
     */
}

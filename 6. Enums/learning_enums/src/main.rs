use rand::Rng;

#[derive(Debug)]
enum Dice { // enum is a set of variants
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}


impl Dice {
    fn roll(&self) -> i32 {
        match self {
            Dice::One => 1,
            Dice::Two => 2,
            Dice::Three => 3,
            Dice::Four => 4,
            Dice::Five => 5,
            Dice::Six => 6,
        }
    }
    fn random_roll() -> Dice {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=6) {
            1 => Dice::One,
            2 => Dice::Two,
            3 => Dice::Three,
            4 => Dice::Four,
            5 => Dice::Five,
            _ => Dice::Six,
        }
    }
}


// this got error chatGPT give me solution to use trait instead -.-
// impl Option<Dice> {
//     fn print_value(&self) {
//         match self {
//             None => println!("None"),
//             Some(Dice::One) => println!("One"),
//             Some(Dice::Two) => println!("Two"),
//             Some(Dice::Three) => println!("Three"),
//             Some(Dice::Four) => println!("Four"),
//             Some(Dice::Five) => println!("Five"),
//             Some(Dice::Six) => println!("Six"),
//             _ => println!("Unknown"),
//         }
//     }
// }


trait PrintDiceValue {
    fn print_value(&self) {
        println!("PrintValue");
    }
}

impl PrintDiceValue for Option<Dice> {
    fn print_value(&self) {
        match self {
            None => println!("None"),
            Some(Dice::One) => println!("One"),
            Some(Dice::Two) => println!("Two"),
            Some(Dice::Three) => println!("Three"),
            Some(Dice::Four) => println!("Four"),
            Some(Dice::Five) => println!("Five"),
            Some(Dice::Six) => println!("Six"),
        }
    }
}

fn main() {
    let d1 = Dice::One;

    println!("d1: {:?}", d1);

    // match with enum
    match d1 {
        Dice::One => println!("One"),
        Dice::Two => println!("Two"),
        Dice::Three => println!("Three"),
        Dice::Four => println!("Four"),
        _ => println!("Other than 1-4"),
    }

    // option is num with null and Some(), Some mean it contains value
    let d2: Option<Dice> = None;
    let d3: Option<Dice> = Some(Dice::One);

    println!("d2: {:?}", d2);
    println!("d3: {:?}", d3);
    println!("d4: {}", d1.roll());

    let random_d: Dice = Dice::random_roll();
    println!("random_d: {:?}", random_d);

    match d2 {
        None => println!("None"),
        Some(Dice::One) => println!("One"),
        Some(Dice::Two) => println!("Two"),
        Some(Dice::Three) => println!("Three"),
        Some(Dice::Four) => println!("Four"),
        Some(Dice::Five) => println!("Five"),
        Some(Dice::Six) => println!("Six"),
    }

    d3.print_value();
    d2.print_value();
}

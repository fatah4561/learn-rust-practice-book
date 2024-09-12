struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
    sign_in_count: u32,
}

struct Point2 {
    x: i32,
    y: i32,
}

fn main() {
    // instance
    let mut user = User {
        // mut here (not per field)
        name: String::from("John"), // heap
        age: 30,
        email: String::from("pBh9M@example.com"), // heap
        active: true,
        sign_in_count: 0,
    };

    user.email = "John123@example.com".to_string();

    println!("{} is {}", user.name, user.age);

    let another_user = build_user(user.email, user.name, user.age);
    println!("{}", another_user.email);

    let user1 = User {
        email: String::from("someone@example.com"),
        name: String::from("someusername123"),
        age: 0,
        active: true,
        sign_in_count: 1,
    };

    println!("user 1 name{}", user1.name);

    // normal way to duplicate a struct
    let mut user2 = User {
        active: user1.active,
        name: user1.name, // but this will refer to the same heap
        age: user1.age,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    user2.name = String::from("user2");

    println!("user 2 name{}", user2.name);
    // error can't do this (because user 2 take ownership of user 1 field name (heap))
    // println!("user 1 name{}", user1.name);
    // also borrow checker will check per field not per struct

    // update struct syntax
    // take all fields from user2 except name
    let user3 = User {
        name: String::from("user3"),
        ..user2
    };

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0); // can be accessed like tuple normally using .index

    // unit like struct
    // behave like unit ()
    // useful for traits but we don't need data
    //
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // borrow checker will check per field

    let mut p = Point2 { x: 0, y: 0 };
    // ex this will not remove the p.y ownership
    let x = &mut p.x;
    // print_point(&p);
    // above will get error because even if we only use x, the struct is not usable
    println!("py {}", p.y); // but by field still usable
    *x += 1;
    println!("p {}, {}", p.x, p.y);

    let mut p = Point2 { x: 1, y: 2 };
    let x = &mut p.x; // this
    let y = &mut p.y; // and this is valid
    // bcs rust take each field individually
    *x += 1;
    *y += 1;
    println!("{} {}", p.x, p.y);
}

fn print_point(p: &Point2) {
    println!("{}, {}", p.x, p.y);
}

fn build_user(email: String, name: String, age: u32) -> User {
    User {
        active: true,
        name: name,
        age, // shorthand age: age
        email: email,
        sign_in_count: 1,
    }
}

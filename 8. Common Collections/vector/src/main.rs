fn main() {
    let v: Vec<i32> = Vec::new(); // vector can only hold same T type
    println!("v is {v:?}");

    let v = vec![1, 2, 3];
    println!("v is now {v:?}");

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // accessing vector element by index
    let third = &v[2];
    println!("The third element is {third}");

    // accessing vector by get() method (this give Option type)
    let third = &v.get(2);
    match third {
        Some(value) => println!("value is {value}"),
        None => println!("not found"),
    }

    // this will give error out of bonds
    // let out_of_index = &v[9];

    let out_of_index = &v.get(9);
    match out_of_index {
        Some(value) => println!("value is {value}"),
        None => println!("not found"),
    }

    for n in &v {
        let n_plus_one = *n + 1;
        println!("n is now {n_plus_one}");
    }
    println!("{v:?}");

    // iterate mutable reference
    let mut v = vec![1, 2];
    println!("mut v before is {v:?}");
    for n in &mut v {
        *n += 1;
    }
    println!("mut v is now {v:?}");

    // another method to iterate vector
    let mut iter = v.iter();
    let n1 = iter.next().unwrap();
    let n2 = iter.next().unwrap();
    // let none = iter.next().unwrap(); // this is none (no more element to iterate)
    let none = iter.next(); // this is none (no more element to iterate), can't be unwrap
                            // because unwrap take the some value of option
    println!("{n1}, {n2}, {none:?}");

    // since vector can only stored the same type
    // we can use enum to store different types
    enum Spreadsheet {
        Int(i64),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        Spreadsheet::Int(30),
        Spreadsheet::Float(3.33),
        Spreadsheet::Text(String::from("blood")),
    ];

    for item in &row {
        // println!("{item:?}");
        match item {
            Spreadsheet::Int(i) => println!("{i}"),
            Spreadsheet::Float(f) => println!("{f}"),
            Spreadsheet::Text(t) => println!("{t}"),
        }
    }

    let last = row.pop(); // removing and returning last element
    match last {
        Some(s) => match s {
            Spreadsheet::Int(i) => println!("s is {i}"),
            Spreadsheet::Float(f) => println!("s is {f}"),
            Spreadsheet::Text(t) => println!("s is {t}"),
        },
        None => println!("bum empty"),
    }

    {
        // when vector goes out of scope, all of its content also dropped
        let v_scope = vec![1, 2, 3];
        println!("{v_scope:?}");
    } // v_scope goes out of scope below here

    {
        /*
        i has type &mut i32, meaning it is a pointer to a number within v. 
        So if we push i into v2, then v2 contains pointers to v. 
        Therefore mutating v2[0] actually mutates v[0].
         */
        let mut v: Vec<i32> = vec![1, 2, 3];
        let mut v2: Vec<&mut i32> = Vec::new();
        for i in &mut v {
            v2.push(i);
        }
        *v2[0] = 5;
        let a = *v2[0];
        let b = v[0];
        println!("{a} {b}");
    }

    {
        // this is solution that doesn't change the original v value
        let mut v: Vec<i32> = vec![1, 2, 3];
        let mut v2: Vec<i32> = Vec::new();
        for i in &v {
            v2.push(*i);
        }
        v2[0] = 5;
        let a = v2[0];
        let b = v[0];
        println!("{a} {b}");
    }
}

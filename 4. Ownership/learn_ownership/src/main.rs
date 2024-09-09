fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut n = &v[0];

    println!("Hello, world! {}", *n);

    println!("v is now {:?}", v);
    println!("v is now {:?}", &v);
    // umm just my testing
    n = &10;
    // n = &10;
    // binding = &(*n +  1);
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // mutable borrow occurs here
    println!("n is now {} and v is now {:?}", n, &v);
    v.push(4); // n no longer used after here so v can be pushed back
    println!("v is now {:?}", v);

    {
        let mut v: Vec<i32> = vec![1, 2, 3];
        // see this is &mut v[2] not &v[] like above, this is called mutable reference
        // unique reference
        let num: &mut i32 = &mut v[2];
        *num += 1;
        println!("Third element is {}", *num);
        println!("Vector is now {:?}", v);
    }

    let mut n = 10;

    let mut a = &n; // this is immutable reference means i can change teh pointer it references to but not the value it points to
    println!("a is now {}", *a);
    println!("n is now {}", n);
    let mut n2 = 11;

    a = &n2;
    println!("a is now {}", *a);
    println!("n is now {} and reference to n2", n);

    let b = &mut n; // this is mutable reference it means i can change the value of n through b
    println!("b is now {}", *b);
    *b += 1;
    println!("b is now {}", *b);
    println!("n is now {}", n);

    println!("====vector=====");
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("s is {s}, v is {}", v[0]);

    {
        let mut name = (String::from("Ferris"), String::from("Rustacean"));
        let first = &name.0;
        name.1.push_str(", Esq.");
        println!("{first} {}", name.1);
    }

    println!("====slices====");

    let mut s = String::from("paw paw");
    println!("f word {}", first_word(&s));

    // this can be used to prevent empty string,
    // ex when s is changed to empty string
    // then 
    println!("f word sliced {}", first_word_slice(&s));
    let word = first_word_slice(&s);
    s.clear();
    println!("f word sliced s empty {}", first_word_slice(&s));
    // println!("word is {}", word); 
    // ^ error compile time

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// fn first_word(s: &String) -> &str {
// fn first_word(s: &str) -> &str { // so we can use both &String and &str
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


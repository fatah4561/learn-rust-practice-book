use std::{collections::HashMap, fmt::format};

fn main() {
    /*
       1.
       Given a list of integers, use a vector and return the median
       (when sorted, the value in the middle position)
       and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    */

    let mut integers = vec![3, 3, 2, 14, 5, 6, 1, 7, 7, 8, 9, 23, 20];
    integers.sort();

    println!("integers len {}", integers.len());

    let median = integers[integers.len() / 2];
    println!("the median is {median}");

    // -- mode
    let mut integer_counts: HashMap<i32, i32> = HashMap::new();
    for value in &integers {
        let count = integer_counts.entry(*value).or_insert(1);
        *count += 1
    }

    let mut mode = 0;
    for (_, value) in &integer_counts {
        if *value > mode {
            mode = *value
        }
    }
    println!("median is {median}, mode is {mode}");
    println!("------------- end no 1 -------------");

    /*
       2.
       Convert strings to pig latin.
       The first consonant of each word is moved to the end of the word and ay is added,
       so first becomes irst-fay.
       Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
       Keep in mind the details about UTF-8 encoding!
    */

    let sample_text = String::from("Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!");

    let sample_words = vec![
        String::from("test"),
        String::from("first"),
        String::from("second"),
        String::from("banana"),
        String::from("apple"),
        String::from("oreo"),
    ];

    let mut vowels = HashMap::new();
    vowels.insert('a', true);
    vowels.insert('i', true);
    vowels.insert('u', true);
    vowels.insert('e', true);
    vowels.insert('o', true);

    let mut result_words: Vec<String> = vec![];
    for word in &sample_words {
        let mut add_to_end = "".to_string();
        let mut res_word = "".to_string();
        let mut found_first_vowel = false;
        let mut ay_or_hay = "ay".to_string();

        for c in word.chars() {
            if found_first_vowel {
                res_word = format!("{}{}", res_word, c);
            } else if vowels.get(&c).is_some() {
                res_word = format!("{}{}", res_word, c);
                found_first_vowel = !found_first_vowel
            } else {
                add_to_end = format!("{}{}", add_to_end, c);
            }
        }
        res_word = format!("{}-{}", res_word, add_to_end);

        if add_to_end.len() == 0 {
            ay_or_hay = "hay".to_string();
        }

        res_word = format!("{}{}", res_word, ay_or_hay);
        result_words.push(res_word);
    }
    println!(
        "sample words is {:?} \nresult words is {:?}",
        sample_words, result_words
    ); // nice

    let mut result_text = "".to_string();
    {
        let mut res_word = "".to_string();
        for word in sample_text.split_whitespace() {
            let mut add_to_end = "".to_string();
            let mut found_first_vowel = false;
            let mut ay_or_hay = "ay".to_string();

            for c in word.chars() {
                if found_first_vowel {
                    res_word = format!("{}{}", res_word, c);
                } else if vowels.get(&c).is_some() {
                    res_word = format!("{}{}", res_word, c);
                    found_first_vowel = !found_first_vowel
                } else {
                    add_to_end = format!("{}{}", add_to_end, c);
                }
            }
            res_word = format!("{}-{}", res_word, add_to_end);

            if add_to_end.len() == 0 {
                ay_or_hay = "hay".to_string();
            }

            res_word = format!("{}{} ", res_word, ay_or_hay);
        }
        result_text = res_word;
    }
    println!("-------------");
    println!("sample text is: \n{sample_text}\n-------------\nresult text is: \n{result_text}");
    println!("------------- end no 2 -------------");

    /*
       3.
       Using a hash map and vectors,
       create a text interface to allow a user to add employee names to a department in a company;
       for example, “Add Sally to Engineering” or “Add Amir to Sales.”
       Then let the user retrieve a list of all people in a department or all people in the company by department,
       sorted alphabetically.
    */
    let text_interface = vec![
        String::from("Add Sally to Engineering"),
        String::from("Add Amir to Sales"),
        String::from("Add Charles to Finance"),
        String::from("Add Job to Finance"),
        String::from("Add Bob to Engineering"),
        String::from("Add Steve to Engineering"),
        String::from("Add Mark to Engineering"),
        String::from("Add Peter to Sales"),
        String::from("Add Parker to Finance"),
        String::from("Add Fatah to Engineering"),
    ];

    // well i usually list things in map to check
    let not_name_words = HashMap::from([
        ("Add".to_string(), true),
        ("to".to_string(), true),
        ("Engineering".to_string(), true),
        ("Sales".to_string(), true),
        ("Finance".to_string(), true),
    ]);

    let department_list = HashMap::from([
        ("Engineering".to_string(), true),
        ("Sales".to_string(), true),
        ("Finance".to_string(), true),
    ]);

    let mut department_members: HashMap<String, Vec<String>> = HashMap::new();

    for prompt in &text_interface {
        let mut name = "".to_string();
        let mut department = "".to_string();

        for word in prompt.split_whitespace() {
            if department_list.get(word).is_some() {
                // then department
                department = word.to_string();
                // println!("got dept");
            } else if !not_name_words.get(word).is_some() {
                // then name
                name = word.to_string();
                // println!("got name");
            }
        }

        let vec_name = vec![name.clone()];
        department_members
            .entry(department)
            .and_modify(|f| {
                f.push(name.to_string());
            })
            .or_insert(vec_name);
    }

    println!(
        "get Engineering people: {:?}",
        department_members.get("Engineering").unwrap_or(&vec![])
    );
    println!(
        "get Finance people: {:?}",
        department_members.get("Finance").unwrap_or(&vec![])
    );
    println!(
        "get Sales people: {:?}",
        department_members.get("Sales").unwrap_or(&vec![])
    );

    // -- sorting
    println!("=== Sorting no 3 ===");
    let mut sorted_department_members: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in &department_members {
        // clone so we don't modify v, dunno clone is easier than playing with pointers -.-
        let mut vec = v.clone(); 
        vec.sort(); // sorting here
        sorted_department_members
            .entry(k.to_string())
            .or_insert(vec);
    }
    // println!("{:?}", sorted_department_members);
    println!(
        "get sorted Engineering people: {:?}",
        sorted_department_members
            .get("Engineering")
            .unwrap_or(&vec![])
    );
    println!(
        "get sorted Finance people: {:?}",
        sorted_department_members.get("Finance").unwrap_or(&vec![])
    );
    println!(
        "get sorted Sales people: {:?}",
        sorted_department_members.get("Sales").unwrap_or(&vec![])
    );

    println!("------------- end no 3 -------------");
}

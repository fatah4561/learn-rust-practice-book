fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn main() {
    let numbers = vec![90, 230, 100, 400, 10, 300];
    let result = largest(&numbers);

    println!("largest number is {}", result);

    let numbers_2 = vec![1,3,4,5,6,2,9];
    let result = largest(&numbers_2);

    println!("largest number 2nd list is {}", result);

    let result = largest(&[1,3,4,5,6]);
    println!("inline list largest {}", result);
}

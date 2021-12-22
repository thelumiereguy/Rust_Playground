use std::mem;

pub fn run() {
    let mut numbers: Vec<&str> = vec!["1", "2", "3", "4", "5"];

    //Add to vector
    numbers.push("6");
    numbers.push("7");

    println!("{:?}", numbers);

    numbers.pop();

    println!("{:?}", numbers);

    // get single value
    println!("0 index-{} ", numbers[0]);

    numbers[0] = "abc";
    println!("New 0 index-{} ", numbers[0]);

    // Length
    println!("Vector Length {} ", numbers.len());

    // Vectors stack allocated - memory
    println!("Memory allocated {} ", mem::size_of_val(&numbers));

    let slice: &[&str] = &numbers[1..3];

    println!("Slice - {:?}", slice);

    // Loop through vector values

    for value in numbers.iter() {
        println!("Number {}", value);
    }

    //Loop and mutate
    for value in numbers.iter_mut() {
        *value = "a";
    }

    println!("New values {:?}", numbers);
}


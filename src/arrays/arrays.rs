use std::mem;

pub fn run() {

    let mut numbers: [&str; 5] = ["1", "2", "3", "4", "5"];

    println!("{:?}", numbers);

    // get single value

    println!("0 index-{} ", numbers[0]);

    numbers[0] = "abc";

    println!("New 0 index-{} ", numbers[0]);

    // Length
    println!("Length {} ", numbers.len());

    // Arrays stack allocated - memory
    println!("Memory allocated {} ", mem::size_of_val(&numbers));


    let slice: &[&str] = &numbers[1..3];

    println!("Slice - {:?}", slice);
}


pub fn run() {
    let mut count = 0;

    // Infinite loop

    loop {
        count += 1;
        println!("New Value {}", count);

        if count > 50 {
            break;
        }
    }


    //Fizzbuzz problem
    count = 0;
    while count < 100 {
        let mut word = String::from("");

        if count % 3 == 0 {
            word.push_str("fizz");
        }
        if count % 5 == 0 {
            word.push_str("buzz");
        }

        if word.is_empty() {
            word.push_str(&*count.to_string());
        }

        println!("{}", word);

        count += 1;
    }

    //ranged loop

    for x in 0..10 {}
}


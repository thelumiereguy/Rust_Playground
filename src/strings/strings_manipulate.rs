pub fn run() {
    let hello = "Hello";

    let mut helloGrowable = String::from("Hello World Growable ");

    println!("{}, {}, Length -  {} ", hello, helloGrowable, helloGrowable.len());

    //append char
    helloGrowable.push('o');

    println!("{} New string", helloGrowable);

    //append string
    helloGrowable.push_str(" This is new string appended");

    println!("{} Newest string", helloGrowable);

    //Check capacity and is empty
    println!("Capacity {}, isEmpty {}", helloGrowable.capacity(), helloGrowable.is_empty());
    println!("Contains word {}", helloGrowable.contains("World"));
    println!("Replace {}", helloGrowable.replace("Hello", "Goodbye "));

    for word in helloGrowable.split_whitespace() {
        println!("Word {}", word);
    }


    // Create string with capacity
    let mut mutableString = String::with_capacity(3);

    mutableString.push('a');
    mutableString.push('b');
    mutableString.push('c');

    println!("mutableString {}", mutableString);


    // assertion
    assert_eq!(3, mutableString.capacity());
}

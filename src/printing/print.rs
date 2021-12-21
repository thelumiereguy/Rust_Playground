pub fn run() {

    //template
    println!("Hello from the {} file", "Print.rs");

    //named template
    println!("This is a {name} template", name = "Named");

    //Placeholder traits
    println!("This is Binary {:b} Hex {:x}", 10, 11);

    //debug trait - value as it is
    println!("This is {:?}", ("debug", " ", "trait"));
}

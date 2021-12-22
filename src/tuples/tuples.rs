pub fn run() {
    let person: (&str, &str, i8) = ("Piyush", "Pratheep", 26);

    println!("{} {} is {} years old", person.0, person.1, person.2);
}

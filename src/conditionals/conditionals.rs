pub fn run() {
    let age = 24;
    let check_id = false;

    if age >= 21 || check_id {
        println!("Ready to marry");
    } else {
        println!("You're too young");
    }

    // Shorthand of if

    let is_of_age = if age >= 21 {
        true
    } else {
        false
    };

    println!("Is of age {}", is_of_age);
}

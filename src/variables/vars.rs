pub fn run() {

    //immutable
    let name = "Piyush";

    //mutable data type
    let mut age = 25;

    println!("My name is {} and I am {}", name, age);

    age = 26;

    println!("My name is {} and I am {} now", name, age);


    //constants
    const ID: i32 = 12345;
    println!("Id is {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Piyush", 26);
    println!("My new name is {} and I am {}", my_name, my_age);
}

pub fn run() {
    greet("Hello", "Piyush");

    let sum = add_numbers(1, 2);
    assert_eq!(3, sum);

    // Closure
    let add_numbers = |n1: i32, n2: i32| n1 + n2;

    assert_eq!(4, add_numbers(2, 2));
}

fn greet(greeting: &str, name: &str) {
    println!("{} {}, Nice to meet you", greeting, name);
}

fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

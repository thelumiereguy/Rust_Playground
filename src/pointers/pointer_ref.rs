pub fn run() {
    let array = [1, 2, 3, 4, 5];


    //Will work because array is primitive
    let array2 = array;

    println!("{:?}", (array2, array));

    //When you copy reference of non primitive data to another variable, the first variable will lose that reference -vector

    let vector = vec![1, 2, 3, 4, 5];

    // wont directly work, so we have to set &
    let vector2 = &vector;

    println!("{:?}", (&vector, vector2));
}

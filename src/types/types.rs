// Integer types - unsigned int - u8,u16,etc (No negative values)
//normal int - i8,i16,etc (Normal integer values)
//floats f32, f64,
//boolean (bool)
//characters (char)
//tuples
//arrays (fixed length)

pub fn run() {
    let int32 = 1;
    let float64 = 2.5;

    let int64: i64 = 12312312;

    // find max size
    println!("{} max i32", std::i32::MAX);
    println!("{} max i64", std::i64::MAX);


    let is_active = true;

    let is_greater_than_10 = int32 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';


    println!("vars {:?}", (is_active, int32, float64, int64, is_greater_than_10, a1, face));
}

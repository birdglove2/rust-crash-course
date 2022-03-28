pub fn run() {
    println!("======VARIABLES======");

    // NUMBER
    // unsigned integer == cannot be negative
    // u8, u16, u32, u64, u128
    let unsigned: u8 = 10;

    // signed integer
    // i8, i16, i32, i64, i128
    let signed: i8 = -10;

    // float is used for decimals
    let float: f32 = 1.2;

    println!(
        "unsigned: {}, signed: {}, float: {}",
        unsigned, signed, float
    );

    // STRING
    let char = 'c'; // single quote
    let emoji = "\u{1F600}"; // :D
    let str = "This is str!"; // double quote
    println!("char: {}, emoji: {}, str: {}", char, emoji, str);

    // bool
    let is_true = true;
    println!("is_true: {}", is_true);

    // ARRAY
    let arr = [1, 2, 3];
    let other_arr = [100; 5];
    println!("index: {}, length: {}", arr[0], other_arr.len());
    println!("Structure of arr: {:?}, {:?}", arr, other_arr);

    // TUPLE
    // can hold different type of elements
    let tuple = (5, true, 2.1);

    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple);

    // destructuring
    let (a, b, c) = tuple;
    println!("first {}, second {}, third {}", a, b, c);
}

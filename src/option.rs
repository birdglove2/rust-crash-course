pub fn run() {
    println!("======OPTION======");

    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    // Unwrapping a 'Some' variant will extract the value wrapped.
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // Unwrapping a 'None' variant will 'panic!'
    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());
}

// Option is an enum with type { Some and None }
// - Some(value), is a tuple struct that wraps a value with type T
// - None, to indicate failure or lack of value
fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

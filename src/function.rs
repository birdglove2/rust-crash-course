pub fn run() {
    println!("======FUNCTION======");
    println!("{}", is_even(8));
}

fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // no semicolon == return function
}

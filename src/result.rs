#[derive(Debug)]
enum MyError {
    Error1,
}

impl Clone for MyError {
    fn clone(&self) -> MyError {
        match self {
            _Error1 => MyError::Error1,
        }
    }
}

pub fn run() {
    println!("======RESULT======");

    let ref divide1: Result<i32, MyError> = divide(4, 2);
    let ref divide2: Result<i32, MyError> = divide(2, 3);

    match divide1 {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{:?}", v),
    }

    if divide1.is_ok() {
        println!("{}", divide1.as_ref().unwrap());
    }

    // if err return 100 instead
    println!("{}", divide2.as_ref().unwrap_or(&100));

    // if err panic with expected message
    // let res = divide2.as_ref().expect("We crashed!");
    // println!("{}", res);
}

// Result is an enum with type { Ok and Err }
// - Ok(value), is a tuple struct that wraps a value with type T
// - Err, An enum that contains an error code
fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

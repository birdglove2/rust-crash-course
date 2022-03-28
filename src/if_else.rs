pub fn run() {
    println!("======IF ELSE======");

    let n = 3;
    if n > 0 {
        println!("{} is greater than 0", n)
    } else if n < 0 {
        println!("{} is less than 0", n)
    } else {
        println!("{} is 0", n)
    }
}

pub fn run() {
    println!("======MATCH======");

    let i = 5;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=5 => println!("3,4,5"), // 0..3 == [0,1,2] BUT 0..=3 == [0,1,2,3]
        _ => println!("default"),
    }
}

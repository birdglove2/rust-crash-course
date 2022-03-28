use std::collections::HashMap;

pub fn run() {
    println!("======MAP======");

    let mut map: HashMap<i32, &str> = HashMap::new();

    map.insert(0, "Hi0");
    map.insert(1, "Hi1");
    println!("map: {:?}", map);

    match map.get(&0) {
        Some(str) => println!("{}", str),         // resolve
        None => println!("Doesn't exist in map"), // reject
    }

    match map.get(&2) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map"),
    }

    map.remove(&0);
    println!("{:?}", map)
}

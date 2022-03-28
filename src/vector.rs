// vector is a dynamic array
pub fn run() {
    println!("======VECTOR======");

    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];

    vec.push(6);
    println!("length: {}, vec[0]: {},  {:?}", vec.len(), vec[0], vec)
}

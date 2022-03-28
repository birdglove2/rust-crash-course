pub fn run() {
    println!("======SLICE======");

    // fix length
    let arr = [0, 1, 2, 3];

    // does not fix length
    let slice = &arr[1..3]; // [inclusive .. exclusive] == [1,2]

    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}

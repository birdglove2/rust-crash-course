pub fn run() {
    println!("======FOR LOOP======");
    forloop();

    println!("======WHILE LOOP======");
    whileloop();
}

fn forloop() {
    for i in 0..6 {
        println!("{}", i);
    }
}

fn whileloop() {
    let mut i = 0;
    while i < 6 {
        println!("{}", i);
        i += 1;
        if i == 2 {
            i += 1;
            continue;
        }
        if i == 5 {
            println!("exit!");
            break;
        }
    }
}

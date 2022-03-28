pub fn run() {
    println!("======ENUM======");

    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C { x: 10, y: 20 };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    // extract value from enum
    if let MyEnum::B(val) = b {
        println!("B={}", val)
    }

    if let MyEnum::C { x, y } = c {
        println!("c.x={} c.y={}", x, y)
    }
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C { x: i32, y: i32 },
}

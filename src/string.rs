//NOTE: See: https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str

pub fn run() {
    println!("======STRING======");

    let _str = "hello world!";

    // String is the dynamic heap string type,
    // like Vec: use it when you need to own or modify your string data.
    let mut string = String::from("Morning");

    string.push('1');
    string.push_str(" 2 3 4");
    string = string.replace("Morning", "Good night");

    println!("{}", string);

    println!("{}", &string[6..]);
}

// use String if you need owned string data
// (like passing strings to other threads,
// or building them at runtime),
// and use &str if you only need a view of a string.

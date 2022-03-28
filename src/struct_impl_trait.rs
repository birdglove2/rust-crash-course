pub fn run() {
    println!("======STRUCT && IMPL && TRAIT======");

    let bird = Bird {
        name: String::from("Chicken"),
        attack: 5,
    };

    bird.print_name();
    bird.attack();
    println!("{} {}", bird.can_fly(), bird.is_animal())
}

struct Bird {
    name: String,
    attack: u64,
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }

    fn attack(&self) {
        println!("{} is attacking you with power {}", self.name, self.attack);
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
    fn is_animal(&self) -> bool {
        false // can override
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

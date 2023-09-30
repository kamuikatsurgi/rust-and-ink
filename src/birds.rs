fn main() {
    let i: i32 = 5;
    match i {
        0 => println!("It is zer0"),
        1 | 2 => println!("It is either 1 or 2"),
        3..=10 => println!("Between 3 and 10"),
        _ => println!("Don't know, maybe a default?")
    }

    let bird: Bird = Bird{ name: String::from("Eagle"), attackpower: 5 };
    bird.print_bird_name();
    bird.can_fly();
    bird.is_animal();
}

struct Bird {
    name: String,
    attackpower: u64
}

impl Bird {
    fn print_bird_name(&self) {
        println!("{}", self.name);
    }
}
// trait in rust is like interface in solidity
trait Animal {
    fn can_fly(&self)-> bool;
    fn is_animal(&self) -> bool;
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        println!("It can fly!");
        return true;
    }
    fn is_animal(&self) -> bool {
        println!("It is an animal too!");
        return true;
    }
}
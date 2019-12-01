#![allow(dead_coad)]

#[derive(Debug)]
enum Person {
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect(p: Person) {
    match p {
        Person::Skinny => println!("Is skinny"),
        Person::Fat => println!("Is fat!"),
        Person::Height(i) => println!("Has a height of {}", i),
        Person::Weight(i) => println!("Has a weight of {}", i),
        Person::Info { name, height } => {
            println!("{} is {} tall", name, height);
        },
    }
}

fn main() {
    let danny  = Person::Weight(10);
    let dave   = Person::Info { name: "Dave".to_owned(), height: 72 };
    let john   = Person::Fat;
    let larry  = Person::Skinny;

    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);

    use Status::{Rich, Poor};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich"),
        Poor => println!("The poor"),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}


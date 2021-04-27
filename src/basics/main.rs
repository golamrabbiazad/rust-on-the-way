#![allow(dead_code)]
// :: -> static methods
// . -> instance mathods

/*
Rust program have 3 memory regions where data is stored.

- data memory
    --> this data is fixed in size and static. it's read from one place.

- stack momory
    --> variables declared in the function that stored in this region. it didn't change the memory location during the funciton call.

- heap memory
    --> this heap memory create when application is runnning. it's nature is dynamic. it's is slower to use. and uses much creative useage memory.
    when data is added to this is called "allocation" and remove is "deallocation"

*/

// Object-like Struct
struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

// Tuple-like Struct
struct Locaiton(i32, i32);

// Unit-like Struct
// struct Marker;

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}

enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}

enum Size {
    Big,
    Small,
}

enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

fn object_type() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    // let sarah = SeaCreature {
    //     animal_type: Species::Fish,
    //     name: String::from("Sarah"),
    //     arms: 8,
    //     legs: 0,
    //     weapon: String::from("brain"),
    // };

    match ferris.species {
        Species::Crab => match ferris.weapon {
            Weapon::Claw(num_claws, size) => {
                let size_description = match size {
                    Size::Big => "big",
                    Size::Small => "Small",
                };
                println!(
                    "ferris is a crab with {} {} claws",
                    num_claws, size_description
                )
            }
            _ => println!("ferris is a crab with some other weapons"),
        },
        _ => println!("ferris is some otehr animal"),
    }
}

fn tupple_like() {
    let loc = Locaiton(324, 235);
    println!("first loc: {}, second loc: {}", loc.0, loc.1)
}

// fn unit_like() {
//     let m = Marker;
// }

fn main() {
    object_type();
    tupple_like();
}

// OOP
// -> OOP is a popular paradigm widely used.

// Encapsulation -> data and fn into the coceptual unit of a single type called an object.

// Abstruction -> Hiding data and fn members and impl details of an object.

// Polymorphism -> The ability to intract with an object from different functional perspectives.

// Inheritance -> The ability to inherit data and behavior from other objects.

// * Struct can't inherit fields from a parent struct.
// * Struct can't inherit function form a parent struct.

struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound())
    }
}

struct Ocean {
    animals: Vec<Box<dyn NoiseMaker>>,
}

// fn static_make_noise(creature: &SeaCreature) {
//     creature.make_noise()
// }

// fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
//     noise_maker.make_noise()
// }

// Generics in function
// fn generic_make_noise<T>(creature: &T)
// where
//     T: NoiseMaker,
// {
//     creature.make_noise()
// }

// shorthand expressing generics...
fn generic_make_noise(creature: &impl NoiseMaker) {
    creature.make_noise()
}

fn main() {
    let ferris = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("Octopus"),
    };
    let sarah = SeaCreature {
        name: String::from("Sarah"),
        noise: String::from("jellyfish"),
    };

    // static_make_noise(&creature);
    // dynamic_make_noise(&creature);

    generic_make_noise(&ferris);

    let ocean = Ocean {
        animals: vec![Box::new(ferris), Box::new(sarah)],
    };

    for a in ocean.animals.iter() {
        a.make_noise();
    }
}

// Box -> move data from stack to heap.
// Box -> a smart pointer that holds the pointer to our data on the heap.

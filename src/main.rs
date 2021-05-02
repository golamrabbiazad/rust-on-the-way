mod basics;
mod foo;
use crate::basics::basics::{object_type, tupple_like};
use crate::foo::foo::eat;

fn main() {
    // [3, 6, 9, 12, 15, 18, 21, 24]
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    println!("{}", x);

    eat("Modon");

    object_type();
    tupple_like();
}

// - crate => the root module
// - super => parent module from the cur module
// self => the current module.

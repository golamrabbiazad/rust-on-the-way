struct Foo {
    x: i32,
}

// the parameter foo and return value share the same lifetime
fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    return &foo_b.x;
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };

    let x = do_something(&foo_a, &foo_b);

    println!("{}", x);
}

// reference prevents data race.
// what is data race?
// -> A data race when reading from data has the possiblity of being out of sync due to the existence of a writter to the data at the same time. This happens often in multi-threaded programming.

// reference prevents misuse of reference that non-existant data. (called daggling pointer in C.)

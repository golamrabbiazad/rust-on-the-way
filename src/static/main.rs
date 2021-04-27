// A static variable is a memory resource created at compile-time that exits through a program start to finish. They must have their explicit specified.

static PI: f64 = 3.1415;

struct Foo<'a> {
    i: &'a i32,
}

fn main() {
    static mut SECRET: &'static str = "swordfish";

    // string literals have a 'static lifetime
    let msg: &'static str = "Hello world";
    let p: &'static f64 = &PI;

    println!("{} {}", msg, p);

    // break some rules, but this is explicit.
    unsafe {
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }

    let x = 32;
    let foo = Foo { i: &x };

    println!("{}", foo.i)
}

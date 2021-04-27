// & -> is referring to the place in memory. it lack of &mut which is not permitted to modification.
// 'static -> is string data that available till end the program. (it never drops)
// str -> its a sequence of bytes that are always valid utf-8

fn main() {
    let a: &'static str = "hi 🐙";

    println!("{}, len: {}", a, a.len());

    let b: &'static str = "Ferris says: 
    \t\"hello octopus\"";

    println!("{}", b);

    println!(
        "hello \
    world"
    );

    let raw_string: &'static str = r#"
        <div class="advice">
            Raw strings are amazing in rust.
        </div>
    "#;

    println!("{}", raw_string);

    let hello_html = include_str!("./text/utf-8.txt");

    println!("{}", hello_html);

    let a = "hi 🐙";
    println!("{}", a.len());
    let first_word = &a[0..2];
    let second_word = &a[3..7];

    let result = a.find("🐙").unwrap();

    println!("{} {}", first_word, second_word);

    println!("{}", result);

    let chars = "rust 🚀".chars().collect::<Vec<char>>();

    println!("{}", chars.len());
    println!("{}", chars[3] as u32);

    let mut hello_world = String::from("Hello");
    hello_world.push_str(" world!");

    let rs = &mut hello_world.replace("!", "");

    println!("{}", rs.to_uppercase());

    say_it_loud(&hello_world);

    let arr_of_hello = ["hello", " ", "octopus", "!"].join("!!!!!!!!!!");

    println!("{}", arr_of_hello);

    let m = 23;
    let f = format!("secret to life {}", m);

    println!("{}", f);

    // convert_str();
}

// char => always 4 bytes long.

fn say_it_loud(msg: &str) {
    println!("{}!!!", msg.to_string().to_uppercase())
}

// fn convert_str() -> Result<(), std::num::ParseIntError> {
//     let a = 35;
//     let a_string = a.to_string();
//     let b = a_string.parse::<i32>()?;
//     println!("{} {}", a, b);
//     Ok(());
// }

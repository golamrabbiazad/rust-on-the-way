fn main() {
    let mut i32_vec = Vec::<i32>::new();

    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    for teger in i32_vec.iter() {
        println!("{}", teger);
    }

    let float_vec = vec![1.2, 1.5, 1.8];

    for fnum in float_vec.iter() {
        println!("{}", fnum);
    }

    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}

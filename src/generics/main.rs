struct BagOfHolding<T> {
    item: Option<T>,
}

fn main() -> Result<(), String> {
    let i32_bag = BagOfHolding::<i32> { item: None };
    if i32_bag.item.is_none() {
        println!("there is nothing in the bag.")
    } else {
        println!("there is something in the bag.")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(32) };

    if i32_bag.item.is_some() {
        println!("there is something in the bag.")
    } else {
        println!("there is nothing in the bag.")
    }

    match i32_bag.item {
        Some(v) => println!("Found {} in the bag", v),
        None => println!("found nothing..!"),
    }

    let v = do_something_that_might_fail(42).unwrap();
    println!("found {}", v);

    let v = do_something_that_might_fail(1).unwrap();
    println!("found {}", v);

    Ok(())
}

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

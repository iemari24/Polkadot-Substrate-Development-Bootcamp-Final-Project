fn main()
{
    let number = -4.0;
    let sqrt = find_sqr(number);

    match sqrt {
        Some(_value) => println!("a"),
        None => println!("b"),
    }
}


fn find_sqr(number: f64) -> Option<f64> {

    if number >= 0.0 {
        Some(number.sqrt())
    }
    else {
        None
    }
}
fn main()
{
    let string1 = String::from("mauro");
    let string2 = String::from("icardi");
    
    let concatenated_string: String = concatenate_strings(string1, string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(a: String, b: String) -> String
{
    let mut result = String::new();
    result.push_str(&a);
    result.push_str(&b);

    return result;
}
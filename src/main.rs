fn main()
{
    let string1 = String::from("emre");
    let string2 = String::from("yilmazcan");
    
    let _concatenated_string: String = concatenate_strings(string1, string2);
    print!("{}", _concatenated_string);
}

fn concatenate_strings(a: String, b: String) -> String
{
    let mut result = String::from("");
    result.push_str(&a);
    result.push_str(&b);

    return result;
}
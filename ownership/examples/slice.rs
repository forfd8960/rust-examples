fn main() {
    let my_str = String::from("hello");
    either_string_or_literal(&my_str);

    let str_literal = "hello";
    either_string_or_literal(&str_literal);

    // create a str slice
    let str_slice = &my_str[..];
    println!("{}", str_slice);

    let mut integers = vec![10, 20, 30];
    let i_slice = &integers[1..];
    println!("{:?}", i_slice);
}

fn either_string_or_literal(s: &str) {
    println!("{}", s)
}

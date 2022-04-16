fn main() {
    let s = "Hello World";
    println!("the first word of s is: {}", first(&s));
}

fn first(s : &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}
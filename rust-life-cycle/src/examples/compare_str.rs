fn main() {
    let s1 = String::from("string1");
    let s2 = String::from("string2");
    let result = max(&s1, &s2);
    println!("the greater one: {}", result);
}

fn max(s1: &str, s2: &str) -> &str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
fn say(s: String) {
    println!("{}", s);
}

fn main() {
    let s1 = String::from("hello");
    say(s1.clone());
    println!("{}", s1);
}

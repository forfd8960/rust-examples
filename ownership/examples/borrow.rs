fn main() {
    let book = String::from("book");
    let pl = pluralize(&book);

    println!("I have one: {}, you have two: {}", book, pl);
}

fn pluralize(s: &str) -> String {
    s.to_owned() + "s"
}

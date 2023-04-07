fn main() {
    let book = String::from("book");

    // pluralize take ownership of the cloned value, so the original value can still be used in main
    let pl = pluralize(book.clone());

    println!("I have one: {}, you have two: {}", book, pl);
}

fn pluralize(mut s: String) -> String {
    s.push_str("s");
    s
}

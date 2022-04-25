use serde::Deserialize;
use std::borrow::Cow;
use url::Url;


#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}

fn main() {
    parse_url();
    unmarshal_data();
}

fn parse_url() {
    let url = Url::parse("https://doc.rust-lang.org/stable/rust-by-example/generics.html?offset=1&page=10&key=hello%20world").unwrap();
    let mut pairs = url.query_pairs();

    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    println!("key: {}, val: {}", k, v); // k, v borrwed

    k.to_mut().push_str("_suffix"); // k 被修改，k 变成 Owned

    print_pairs((k, v));
    print_pairs(pairs.next().unwrap());
    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed: {}", v),
        Cow::Owned(v) => format!("Owned: {}", v),
    }
}

fn unmarshal_data() {
    let input = r#"{ "name": "forfd8960", "age": 22 }"#;
    let user: User = serde_json::from_str(input).unwrap();

    println!("user: {:?}", user);

    match user.name {
        Cow::Borrowed(v) => println!("borrowed: {}", v),
        Cow::Owned(v) => println!("owned: {}", v),
    }
}
use std::str::FromStr;
use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        let d = || Default::default();

        if let Some(captures) = re.captures(s) {
            captures.get(0).map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }
    }
}

#[test]
fn test_genetic_parse() {
    assert_eq!(u32::parse("123abc"), 123);
    assert_eq!(u32::parse("1234abcd"), 0);
    assert_eq!(f64::parse("1988.66abc"), 1988.66);
    assert_eq!(f64::parse("2320.123abc"), 2320.123);
}

fn main() {
    println!("parse: {} result: {}", "123.45abc", f64::parse("123.45abc"))
}
use std::borrow::Cow;

fn mod_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Reminder is 0".into(),
        1 => "Reminder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into(),
    }
}

fn main() {
    for number in 1..=6 {
        match mod_3(number) {
            Cow::Borrowed(message) => println!("mod_3 borrowed msg: {}", message),
            Cow::Owned(message) => println!("mod_3 owned msg: {}", message),
        }
    }
}
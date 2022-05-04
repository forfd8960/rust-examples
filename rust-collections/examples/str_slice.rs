use std::fmt;

fn main() {
    let s = String::from("hello");

    // String can be deref to &str
    print_slice(&s);
    print_slice(&s[..]); // &s[..] and s.as_str() => &str

    // String support AsRef<str>
    print_slice1(&s);
    print_slice1(&s[..]);
    print_slice1(s.clone());

    // String also impl AsRef<[u8]>
    print_slice2(&s);
    print_slice2(&s[..]);
    print_slice2(s);
}

fn print_slice(s: &str) {
    println!("{:?}", s);
}

fn print_slice1<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

fn print_slice2<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

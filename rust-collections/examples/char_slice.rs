use std::iter::FromIterator;

fn main() {
    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");

    let s1 = &arr[1..3];
    let s2 = &vec[1..3];
    let s3 = &s[1..3];
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    println!("compare 2 char slice: ");
    assert_eq!(s1, s2);

    println!("cnovert s3 to char slice to compare: ");
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());

    println!("cnovert s2 to str slice to compare: ");
    assert_eq!(String::from_iter(s2), s3);
}

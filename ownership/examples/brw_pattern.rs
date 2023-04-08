fn main() {
    let mut list = vec![1, 2, 3];

    let mut first_num = list.first_mut().expect("list empty");
    *first_num += 1;

    let first = list.first();
    let last = list.last();

    println!("first: {:?}, last: {:?}", first, last);
}

fn main() {
    let mut list = vec![1, 2, 3];

    {
        let first = list.first();
        let last = list.last();
        println!("first: {:?}, last: {:?}", first, last);
    }

    *list.first_mut().expect("list empty") += 1;
}

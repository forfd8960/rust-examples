use crate::List::{Cons, Nil};

// https://doc.rust-lang.org/book/ch15-01-box.html
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// error[E0072]: recursive type `List` has infinite size
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);
}
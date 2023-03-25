use crate::List::{Cons, Nil};

// https://doc.rust-lang.org/book/ch15-01-box.html
enum List {
    Cons(i32, List),
    Nil,
}

// error[E0072]: recursive type `List` has infinite size
fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
    println!("list: {:?}", list);
}
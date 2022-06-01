use std::marker::PhantomData;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Ident<T> {
    inner: u64,
    _tag: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Ident<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product {
    id: Ident<Self>
}

fn main() {
    let user = User::default();
    let product = Product::default();

    /*
    mismatched types
    expected struct `Ident<User>`
   found struct `Ident<Product>`
    */
    // let is_equal = user.id == product.id;

    let is_equal = user.id.inner == product.id.inner;
    println!("is user id equals product id: {}", is_equal);
}

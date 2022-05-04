use std::ops::Deref;

fn main() {
    let mut vec = vec![1,2,3,4];
    println!("vec capacity: {}", vec.capacity());
    vec.push(5);
    println!("cap should be 8: {}", vec.capacity());

    let b1 = vec.into_boxed_slice(); // into_boxed_slice: Vec<T> convert to Box<T>
    let mut b2 = b1.clone();

    let v2 = b1.into_vec(); // into_vec: Box<T> convert to Vec<T>
    println!("capacity should be 5: {}", v2.capacity());

    assert!(b2.deref() == v2);

    // box<T> can change data, but can not do push
    b2[0] = 100;
    println!("b2: {:?}", b2);

    let  b3 = Box::new([100,2,3,4]);
    println!("b3: {:?}", b3);
    assert!(b2 == b3);
    // assert!(b3.deref() == v2);
}
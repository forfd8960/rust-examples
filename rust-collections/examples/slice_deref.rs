use std::fmt;

fn main() {
    let v = vec![5, 8, 7, 6];

    println!("------ print slice -----");

    // Vec<T> impl Deref trait, so it can be auto deref as &[T]
    print_slice(&v); // [5, 8, 7, 6]
    print_slice(&v[..]); // &vec[..] is &[T]

    print_slice1(&v); // &Vec<T> support AsRef<[T]>
    print_slice1(&v[..]); // &[T] support AsRef<[T]>
    print_slice1(v); // Vec<T> support AsRef<[T]>

    let arr = [1, 2, 3, 6];
    println!("------ print arr -----");
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);
}

fn print_slice<T: fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}

// AsRef: auto-dereferences if the inner type is a reference or a mutable reference
fn print_slice1<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

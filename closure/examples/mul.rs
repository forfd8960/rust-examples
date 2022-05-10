use std::ops::Mul;

/*
    5 * 2 = 10
    pi multiply 4^2 = 50.24
*/
fn main() {
    let c1 = curry(5);
    println!("5 * 2 = {}", c1(2));

    let c2 = curry(3.14);
    println!("pi multiply 4^2 = {}", c2(4. * 4.));
}

fn curry<T>(x: T) -> impl Fn(T) -> T where T: Mul<Output=T> + Copy {
    move |y| x * y
}
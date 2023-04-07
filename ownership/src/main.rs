fn main() {
    let mut s = String::from("Hello");
    s.push_str(" Rust");

    //Moving s to t, after this s can not no longer be used.
    let new_str = s;
    println!("{}", s);
}

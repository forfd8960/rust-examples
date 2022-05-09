
fn main() {
    let name = String::from("test-closure");
    let c = move |greeting: String| (greeting, name);
    let result = c("hello".to_string());
    println!("RESULT: {:?}", result);

    // use of moved value: `c`
    // let result = c("hi".to_string());

    // value borrowed here after moverustc(E0382)
    // fn_once.rs(3, 9): move occurs because `name` has type `std::string::String`, which does not implement the `Copy` trait
    // fn_once.rs(4, 13): value moved into closure here
    // fn_once.rs(4, 48): variable moved due to use in closure
    // println!("access name: {}", name);
}
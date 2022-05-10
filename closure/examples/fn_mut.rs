// FnMut
// fn mut can be called many times
// FnOnce is FnMut's super trait 
fn main() {
    let mut hello = String::from("Bob");
    let mut alice = String::from("Alice");

    let mut c = || {
        hello.push_str(" Bob");
        println!("name: {}", hello);
    };

    let mut c1 = move || {
        alice.push_str("~");
        println!("alice: {}", alice);
    };

    /*
    name: Bob Bob
    alice: Alice~
    */
    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);

    // FnOnce is super trait for FnMur, So FnMut can be pass as FnOnce
    /*
    call_mut
    name: Bob Bob Bob
    call_mut
    alice: Alice~~
    call_once
    name: Bob Bob Bob Bob
    call_once
    alice: Alice~~~
    */
    call_once(c);
    call_once(c1);
}

fn call_mut(c: &mut impl FnMut()) {
    println!("call_mut");
    c();
}

fn call_once(c: impl FnOnce()) {
    println!("call_once");
    c();
}
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let t1 = thread::spawn(move || {
        println!("vec: {:?} ", v);
    });

    t1.join().unwrap();
}
// Just like any owned value, when a box goes out of scope, 
// as b does at the end of main, it will be deallocated. 
// The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).

fn main() {
    // The Box<T> type is a smart pointer because it implements the Deref trait, 
    // which allows Box<T> values to be treated like references
    let b = Box::new(5);
    println!("b: {}", b);
}
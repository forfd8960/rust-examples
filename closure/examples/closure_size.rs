use std::{mem::size_of_val, collections::HashMap};

// 闭包的大小跟参数、局部变量都无关，只跟捕获的变量有关
fn main() {
    let c1 = || println!("hello, closure");
    println!("size of c1: {}", size_of_val(&c1)); // size of c1: 0
    
    let c2 = |i: i32| println!("closure with args: {}", i);
    println!("size of c2: {}", size_of_val(&c2)); // size of c2: 0
    
    let name = String::from("closure");
    let c3 = || println!("closure cap outer ref: {}", name);
    println!("size of c3: {}", size_of_val(&c3)); // size of c3: 8
    
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    let c4 = move || println!("closure cap 2 outer ref: {}, {:?}", name1, table);
    println!("size of c4: {}", size_of_val(&c4)); // size of c4: 72
    
    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("closre-test");
        println!("x: {}, namee2: {}, name3: {}", x, name2, name3);
    };
    println!("size of c5: {}", size_of_val(&c5)); // size of c5: 24
}
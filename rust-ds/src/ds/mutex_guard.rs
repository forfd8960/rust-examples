#[macro_use]
extern crate lazy_static;

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

lazy_static! {
    // Mutex 和 Arc 一起在多线程环境下提供对共享内存的使用
    // Mutext 声明为 'static, 生命周期是静态，不需 Arc
    static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> = Mutex::new(HashMap::new());
}

fn main()  {
    // Arc 提供并发环境下的共享所有权
    let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..32 {
        let m = metrics.clone();
        thread::spawn(move || {
            // 只有获取到 MutexGuard 的线程才有才可以访问 HashMap
            let mut g = m.lock().unwrap();
            let data = &mut *g;
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
            // MutexGuard Drop, 锁释放
        });
    }

    thread::sleep(Duration::from_millis(100));
    println!("metrics: {:?}", metrics.lock().unwrap());
}
# rust 生命周期 

* 如果一个值的生命周期贯穿整个进程的生命周期， 这种生命周期为 `静态生命中周期`. 用 `'static` 来表示.
* 如果一个值是在某个作用域定义的，被创建在 栈或者堆上，生命周期是动态的.
* 当值的作用域结束时，值的生命周期也随之结束。

## cargo run strsplit

* from: https://www.youtube.com/watch?v=rAl-9HwD858

```sh
cargo run --bin strsplit
   Compiling rust-life-cycle v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/strsplit`
strsplit: StrSplit { remain: Some("hello, rust, and, good, morning"), token: ',' }
words: ["hello", " rust", " and", " good", " morning"]
```
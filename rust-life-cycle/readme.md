# rust 生命周期 

* 如果一个值的生命周期贯穿整个进程的生命周期， 这种生命周期为 `静态生命中周期`. 用 `'static` 来表示.
* 如果一个值是在某个作用域定义的，被创建在 栈或者堆上，生命周期是动态的.
* 当值的作用域结束时，值的生命周期也随之结束。

## cargo run strsplit

```sh
cargo run --bin strsplit
   Compiling rust-life-cycle v0.1.0
warning: field is never read: `remain`
 --> src/examples/str_split.rs:3:5
  |
3 |     remain: Option<&'a str>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default
note: `StrSplit` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
 --> src/examples/str_split.rs:1:10
  |
1 | #[derive(Debug)]
  |          ^^^^^
  = note: this warning originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: field is never read: `token`
 --> src/examples/str_split.rs:4:5
  |
4 |     token: TK
  |     ^^^^^^^^^
  |
note: `StrSplit` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
 --> src/examples/str_split.rs:1:10
  |
1 | #[derive(Debug)]
  |          ^^^^^
  = note: this warning originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `rust-life-cycle` (bin "strsplit") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
     Running `target/debug/strsplit`
strsplit: StrSplit { remain: Some("hello, rust"), token: ',' }
```
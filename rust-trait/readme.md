# trait

* rust trait 定义了类型使用这个接口的行为.



### run code

```sh
cargo run --bin generic_parse_err
   Compiling rust-trait v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.44s
     Running `target/debug/generic_parse_err`
parse: 123.45abc result: Ok(123.45)
```

```sh
cargo run --bin add_complex
   Compiling rust-trait v0.1.0 
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/add_complex`
c1+c2: Complex { real: 3.0, img: 4.0 }
c1+6.66: Complex { real: 7.66, img: 1.0 }
c1+c2: Complex { real: 3.0, img: 4.0 }
```
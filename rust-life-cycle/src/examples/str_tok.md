
## err msg

```sh
cargo run --bin strtok 
```

```sh
Compiling rust-life-cycle v0.1.0 (/Users/zhiru.chen/Documents/code/rust/rust-examples/rust-life-cycle)
error[E0106]: missing lifetime specifier
 --> src/examples/str_tok.rs:1:45
  |
1 | fn strtok(s: &mut &str, delimiter: char) -> &str {
  |              ---------                      ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say which one of `s`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
  |
1 | fn strtok<'a>(s: &'a mut &str, delimiter: char) -> &'a str {
  |          ++++     ++                                ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `rust-life-cycle` due to previous error
```
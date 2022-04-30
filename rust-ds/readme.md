## cargo run

* run box

```sh
     Running `target/debug/box`
ALLOC: 0x7ff3c3c05b60, size: 5
ALLOC: 0x7ff3c3c05b70, size: 64
ALLOC: 0x7ff3c3c05bb0, size: 48
ALLOC: 0x7ff3c3c05be0, size: 80
ALLOC: 0x7ff3c3c05cc0, size: 24
ALLOC: 0x7ff3c3c05ce0, size: 64
ALLOC: 0x7ff3c3c05d20, size: 505
ALLOC: 0x7ff3c4008800, size: 1024
---- allocated memory: 0x7ff3c3c05d20, len: 505 -----
FREE: 0x7ff3c3c05d20, size: 505
FREE: 0x7ff3c4008800, size: 1024
FREE: 0x7ff3c3c05b60, size: 5
FREE: 0x7ff3c3c05b70, size: 64
FREE: 0x7ff3c3c05bb0, size: 48
FREE: 0x7ff3c3c05be0, size: 80
FREE: 0x7ff3c3c05ce0, size: 64
FREE: 0x7ff3c3c05cc0, size: 24
```

### cargo run cow

```sh
rust-ds git:(main) ✗ cargo run --bin cow
   Compiling rust-ds v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/cow`
key: offset, val: 1
key: Owned: offset_suffix, value: Borrowed: 1
key: Borrowed: page, value: Borrowed: 10
key: Borrowed: key, value: Owned: hello world
user: User { name: "forfd8960", age: 22 }
borrowed: forfd8960
```

### use serde error

```
note: `Deserialize` is imported here, but it is only a trait, without a derive macro
 --> src/ds/cow.rs:3:5
  |
3 | use serde::Deserialize;
```

fix cargo.toml

```toml
from
serde = "1.0.136"
to
serde = { version = "1.0.136", features = ["derive"] }
```

### cargo run --bin mutexguard

```sh
cargo run --bin mutexguard
   Compiling lazy_static v1.4.0
   Compiling rust-ds v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.75s
     Running `target/debug/mutexguard`
metrics: {"hello": 32}
```
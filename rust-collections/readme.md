# collections

## 集合容器

把相同类型的数据放到一起，统一处理.

## 切片

描述同一类型，长度不确定，在内存中连续存放的数据结构.

- &[T]: 只读的切片引用
- &mut [T]: 可写的引用
- Box<[T]>: 在堆上分配的切片。

```sh
cargo run --bin slicederef
   Compiling rust-collections v0.1.0 rust-collections)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/slicederef`
------ print slice -----
[5, 8, 7, 6]
[5, 8, 7, 6]
[5, 8, 7, 6]
[5, 8, 7, 6]
[5, 8, 7, 6]
------ print arr -----
[1, 2, 3, 6]
[1, 2, 3, 6]
[1, 2, 3, 6]
[1, 2, 3, 6]
[1, 2, 3, 6]
```
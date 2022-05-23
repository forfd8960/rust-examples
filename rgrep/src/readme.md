
## rgrep

```sh
rgrep git:(main) ✗ ./target/debug/rgrep -p Hello -f src/test.txt
Opt { pattern: "Hello", file_path: "src/test.txt" }
q: Query { pattern: "Hello", file_name: "src/test.txt" }
> "src/test.txt"
>>>> "Hello, abc"
>>>> "Time, Hello"
```
fn main() {
    let v = vec![2, 3, 5, 6, 9];
    let result = &v
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 16)
        .take(1)
        .collect::<Vec<_>>();
    println!("result: {:?}", result);
}

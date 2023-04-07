
fn main() {
    let n1 = 1;
    let n2 = 5;

    match (n1, n2) {
        (1, 1) => println!("matched (1, 1)"),
        (5, _) | (_, 5) => {
            println!("matched (5, 1) or (_, 5)");
        },
        _ => println!("matched everything else"),
    }
}
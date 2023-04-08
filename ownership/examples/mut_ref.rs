#[derive(Debug)]
struct Bucket {
    liters: u32,
}

fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}

fn main() {
    let mut source = Bucket { liters: 20 };
    let mut target = Bucket { liters: 10 };

    pour(&mut source, &mut target, 5);

    println!("soruce: {:?}, target: {:?}", source, target);
}

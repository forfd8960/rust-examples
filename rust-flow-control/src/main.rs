fn fib_loop(n: u32)  {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u32;
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is: {}", b);
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u32) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is : {}", b);

        i+=1;
    }
}

fn fib_for(n: u32) {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is: {}", b);
    }
}

fn main() {
    println!("rust for loop");
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}

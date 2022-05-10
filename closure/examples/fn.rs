
/*
    direct call c(2): 2048
    direct call c1(2): 2046
    call fn c(2): 3072
    call fn c1(2): 3069
    call fn mut c(2): 4096
    call fn mut c1(2): 4092
    call fn once c(2): 5120
    call fn once c1(2): 5115
*/
fn main() {
    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];

    let mut c = |x: u64| v.len() as u64 *x;
    let mut c1 = move |x: u64| v1.len() as u64 * x;

    println!("direct call c(2): {}", c(2));
    println!("direct call c1(2): {}", c1(2));

    println!("call fn c(2): {}", call_fn(3, &c));
    println!("call fn c1(2): {}", call_fn(3, &c1));

    println!("call fn mut c(2): {}", call_mut(4, &mut c));
    println!("call fn mut c1(2): {}", call_mut(4, &mut c1));

    println!("call fn once c(2): {}", call_once(5, &c));
    println!("call fn once c1(2): {}", call_once(5, &c1));
}

fn call_fn(arg: u64, c: &impl Fn(u64) -> u64) -> u64  {
    c(arg)
}

fn call_mut(arg: u64, c: &mut impl FnMut(u64)->u64) -> u64 {
    c(arg)
}

fn call_once(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}
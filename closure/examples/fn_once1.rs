
/*
call c 1st time: ("first call", "test-closure")
call c 2st time: ("second call", "test-closure")
call once: ("once call", "test-closure")
call FnOnce: ("pass Fn as FnOnce", "HiHi")
*/
fn main() {
    let name = String::from("test-closure");

    //c clonse name, so c is not a fn once type closure
    let c = move |greeting: String| (greeting, name.clone());
    println!("call c 1st time: {:?}", c("first call".to_string()));
    println!("call c 2st time: {:?}", c("second call".to_string()));

    println!("call fn c: {:?}", call_fn("callFn".into(), &c));

    /*
    borrow of moved value: `name`
    value borrowed here after move
    */
    // println!("name: {}", name);
    
    // after called as FnOnce, can not be called again
    println!("call once: {:?}", call_once("once call".into(), c));
    /*
    fn_once1.rs(11, 63): value moved here
    fn_once1.rs(12, 33): value borrowed here after move
    */
    // println!("call once: {:?}", c("HI".into()));

    println!("call FnOnce: {:?}", call_once("pass Fn as FnOnce".into(), not_closure));
}

fn call_fn(arg: String, c: &impl Fn(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "HiHi".into())
}
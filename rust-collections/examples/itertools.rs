use itertools::Itertools;

fn main() {
    let err_str = "error happend";
    let input = vec![Ok(21), Err(err_str), Ok(7)];
    // filter_map_ok: Return an iterator adaptor that filters and transforms every Result::Ok value with the provided closure.
    // Result::Err values are unchanged.
    let it = input
        .into_iter()
        .filter_map_ok(|i| if i > 10 { Some(i * 2) } else { None });
    let result = it.collect::<Vec<_>>();
    println!("filter result: {:?}", result);
    itertools::assert_equal(result, vec![Ok(42), Err(err_str)]);
}

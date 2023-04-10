use std::collections::HashMap;

fn main() {
    let words = "hello rust hello rust ha";
    let mut freqs = HashMap::new();

    for word in words.split_whitespace() {
        // match freqs.get_mut(word) {
        //     Some(value) => {
        //         *value += 1;
        //     }
        //     None => {
        //         freqs.insert(word, 1);
        //     }
        // }

        // also works
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("{:#?}", freqs);
}

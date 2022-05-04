fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];

    let s1 = &arr[..2];
    let s2 = &vec[..2];
    println!("s1: {:?}, s2: {:?}", s1, s2);

    assert_eq!(s1, s2); // &[T] 和 &[T] 相等：长度和内容相等
    assert_eq!(&arr[..], vec);
    assert_eq!(&vec[..], arr);
}
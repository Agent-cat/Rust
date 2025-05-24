fn main() {
    let vec = vec![20, 30, 12, 6];
    let mut iter = vec.iter();
    while let Some(val) = iter.next() {
        println!("{}", val)
    }
}

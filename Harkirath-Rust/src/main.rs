fn main() {
    let x = String::from("vishnu");
    drop(x);
    print!("{}", x); // we will get error Here because the string is droprd from the Heap
}

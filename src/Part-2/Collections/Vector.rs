fn main() {
    let mut vec = Vec::new();
    vec.push("hi");
    vec.push("vishnu");
    // vec.push(1); // givess an error because vectors store values of the same type
    print!("{}{}", vec[0], vec[1]);
    // this prints the first and second element of the vector
    print!("{:?}", vec);
    // this prints the entire vector
}

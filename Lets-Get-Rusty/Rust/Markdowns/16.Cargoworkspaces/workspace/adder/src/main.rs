use add_one;
fn main() {
    let num = 10;
    println!("{} plus one is {}", num, add_one::add_one(num));
}
//  cargo run -p adder  -> to run the package

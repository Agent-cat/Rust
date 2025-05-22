use std::thread;
use std::time::Duration;
fn main() {
    let x = 1;
    {
        let vec = vec![1, 2, 34, 4];
        let thread1 = thread::spawn(move || print!("{:?}", vec));
        //we will get an error because we dont know when will the thread start exexution if it prints x then it goes out of scope and becomes dangling pointer
    }
    print!("{x}");
}

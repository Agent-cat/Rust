fn main() {
    let mut num = 10;
    let x = &num as *const i32; // It is raw pointer
    let y = &mut num as *mut i32; // It is raw pointer

    unsafe {
        println!("{}", *x);
        println!("{}", *y);
    }
}

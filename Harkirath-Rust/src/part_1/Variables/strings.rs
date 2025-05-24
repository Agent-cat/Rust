fn main() {
    // ==========  String concat Type1  =================
    // let str = String::from("Vishnu reddy - ");
    // let str2 = String::from("Vardhan");
    // let con = format!("{} {}", str, str2);
    // println!("{}", con);
    //  ==========  String concat Type2  =================

    // let con1 = str.clone() + &str2;
    // println!("{}", con1);
    // ========Update String============================
    let mut str1 = String::from("Vishnu");
    str1.push_str("Vardhan");
    print!("{}", str1)
}

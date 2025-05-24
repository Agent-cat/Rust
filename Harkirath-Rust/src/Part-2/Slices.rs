//Q: Write a function that takes a string as an input And returns the ﬁrst word from it
fn main() {
    let mut word = String::from("Vis hnu Vardhan ");
    word = first_word(&word).to_string();
    println!("{}", word);
}
fn first_word(word: &String) -> &str {
    let mut index = 0;

    for i in word.chars() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    return &word[0..index];
}

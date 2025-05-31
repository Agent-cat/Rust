use minigrep::Config;
use std::env;
use std::process; // Used to exit without panicking the thread
fn main() {
    let args: Vec<String> = env::args().collect();
    // it gets the arguments passed from ``cargo run one two three ``
    // Here we are modulearising our application
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {}", err);
        process::exit(1)
    });
    // This exit wit proper error message

    println!("Searching for : {}", config.query);
    println!("In File : {}", config.filename);
    let result = Config::run(&config).expect("Error while finding"); // made a new method run
    let ans = Config::search(&config.query, &result);
    for i in ans {
        println!("{}", i);
    }
}

use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}
// To show the relation btw two strings we used structs
impl Config {
    //Here we are returning a Result Enum With Config at Ok and str at Err
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }

    pub fn run(config: &Config) -> Result<String, &str> {
        let result =
            fs::read_to_string(&config.filename).expect("Error Occoured While Reading file ");
        Ok(result)
    }
    pub fn search<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {
        let q = query.to_string().to_lowercase();
        let mut vec = Vec::new();
        for i in contents.lines() {
            if i.contains(&q) {
                vec.push(i);
            }
        }
        vec
    }
}

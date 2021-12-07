use std::error::Error;
use std::fs;
pub struct Config {
    query: String,
    filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query: query.to_owned(),
            filename: filename.to_owned(),
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let query = &config.query;
    let filename = &config.filename;

    println!("Searching {} for lines containing: '{}'", filename, query);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong!");
    let lines = search(query, &contents);

    for line in lines {
        println!("-> {}", line);
    }

    Ok(())
}
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn search_returns_matches() {
        let query = "you";
        let contents = "
Hello,
How are you?
I hope you are doing ok
        ";

        assert_eq!(
            vec!["How are you?", "I hope you are doing ok"],
            search(query, contents)
        );
    }

    #[test]
    pub fn parse_config() -> Result<(), Box<dyn Error>> {
        let args = vec![
            "./name-of-binary".to_owned(),
            "test.txt".to_owned(),
            "today".to_owned(),
        ];
        let config = Config::new(&args)?;

        assert_eq!("test.txt", &config.filename);
        assert_eq!("today", &config.query);

        Ok(())
    }

    #[test]
    pub fn run_with_config() -> Result<(), Box<dyn Error>> {
        let config = Config {
            filename: "test.txt".to_owned(),
            query: "today".to_owned(),
        };

        run(config)?;

        Ok(())
    }
}

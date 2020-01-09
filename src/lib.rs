use std::fs;
use std::process;
use std::error::Error;

pub struct Config {
    search_string: String,
    file_name: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(
            Config {
                search_string: String::from(&args[1]),
                file_name: String::from(&args[2])
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&config.file_name)?;
    let search_results = if config.case_sensitive {
        search(&config.search_string, &file_contents)
    } else {
        case_insensitive_search(&config.search_string, &file_contents)
    };

    for line in search_results{
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line)
        }
    }
    
    matches
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches: Vec<&str> = vec![];

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            matches.push(line)
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents ="
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            case_insensitive_search(query, contents)
        )
    }
}
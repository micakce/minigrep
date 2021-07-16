use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

        args.next();

        let query  = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query argument")
        };

        let filename  = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename argument")
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive})
    }
}


pub fn run( config: Config ) -> Result<(),Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };


    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut matched_lines = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}


///////////////////////////////////////////////////////////////////////////////////////////////
//                                             Tests                                         //
///////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_has_enough_arguments() {
        let args = vec![String::from("exec"), String::from("word"), String::from("filename")];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query , "word");
    }

    #[test]
    #[should_panic]
    fn it_does_not_have_enough_arguments() {
        let args = vec![ String::from("word"), String::from("filename")];
        let _config = Config::new(&args).unwrap_or_else(|err| {
            panic!("Error running config: {}", err);
        });
    }

    // TODO: test run method

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search_test() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}

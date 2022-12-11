use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub flag: Option<String>, //TODO: Turn into vector, so multiple flags/options can be passed
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool, //TODO: Move into options
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let flag: Option<String>;
        let query: String;
        let file_path: String;

        if args[1].starts_with("-") {
            if args[1] == "-n" {
                flag = Some(args[1].clone());
                query = args[2].clone();
                file_path = args[3].clone();
            } else {
                return Err("Invalid flag!");
            }
        } else {
            flag = None;
            query = args[1].clone();
            file_path = args[2].clone();
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            flag,
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let add_line_numbers = !config.flag.is_none();

    let results = search(&config.query, &contents, config.ignore_case);

    output(results, add_line_numbers);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<(i32, &'a str)> {
    let mut results = Vec::new();
    let mut line_number = 1;

    for line in contents.lines() {
        if (case_sensitive && line.contains(&query))
            || (!case_sensitive && line.to_lowercase().contains(&query.to_lowercase()))
        {
            results.push((line_number, line));
        }
        line_number += 1;
    }
    results
}

pub fn output(lines: Vec<(i32, &str)>, line_numbers: bool) {
    for line in lines {
        if line_numbers {
            println!("{}", line_with_line_number(line.1, line.0));
        } else {
            println!("{}", line.1)
        }
    }
}

pub fn line_with_line_number(line: &str, line_number: i32) -> String {
    let mut line_number = format!("{}: ", line_number);
    line_number.push_str(line);
    line_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        let case_sensitive = true;

        assert_eq!(
            vec![(2, "safe, fast, productive.")],
            search(query, contents, case_sensitive)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let case_sensitive = false;

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search(query, contents, case_sensitive)
        );
    }
}
#[test]
fn line_number() {
    let line_number = 4;
    let line = "This is a test line.";
    assert_eq!(
        "4: This is a test line.",
        line_with_line_number(line, line_number)
    )
}

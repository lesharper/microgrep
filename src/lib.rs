use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        let highlighted = highlight_matches(line, &config.query, config.ignore_case);
        println!("{highlighted}");
    }

    Ok(())
}

fn highlight_matches(line: &str, query: &str, ignore_case: bool) -> String {
    let mut result = String::new();
    let mut last = 0;
    let line_lower = line.to_lowercase();
    let query_lower = query.to_lowercase();
    let query_len = query.len();

    let haystack = if ignore_case { &line_lower } else { line };
    let needle = if ignore_case { &query_lower } else { query };

    while let Some(start) = haystack[last..].find(needle) {
        let start = last + start;
        let end = start + query_len;
        result.push_str(&line[last..start]);
        result.push_str(&line[start..end]);
        last = end;
    }
    result.push_str(&line[last..]);
    result
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

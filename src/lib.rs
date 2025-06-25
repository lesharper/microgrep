use std::env;
use std::error::Error;
use std::fs; // <--- Добавляем для работы с переменными окружения

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // По умолчанию устанавливаем ignore_case в true, как было в оригинале
        let mut ignore_case = true;

        // Пытаемся прочитать переменную окружения с именем "IGNORE_CASE"
        // env::var("IGNORE_CASE") вернет Ok(String), если переменная установлена,
        // или Err(VarError), если нет.
        if let Ok(var_value) = env::var("IGNORE_CASE") {
            // Если переменная установлена, проверяем её значение.
            // .eq_ignore_ascii_case("false") позволяет сравнивать без учета регистра.
            if var_value.eq_ignore_ascii_case("false") {
                ignore_case = false; // Отключаем case-insensitive поиск
            }
            // Если значение переменной "true" или что-то другое,
            // ignore_case останется true, сохраняя поведение по умолчанию.
        }
        // Если переменная окружения "IGNORE_CASE" не установлена (получен Err),
        // то блок if let не выполнится, и ignore_case останется true,
        // что тоже соответствует поведению по умолчанию.

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    println!("Find:\n");

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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

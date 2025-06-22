use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("В файле {}", config.file_path);
    
    let contents = fs::read_to_string(config.file_path).expect("Ошибка чтения файла");
    
    println!("Текст:\n{contents}");

}


struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
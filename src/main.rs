use clap::Parser;
use microgrep::{Config, run};
use std::process;

/*Теперь программа запускается так:

cargo run -- "query" "file.txt"

С флагом игнорирования регистра:

cargo run -- "query" "file.txt" --ignore-case

Для справки:

cargo run -- --help

*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Шаблон для поиска
    query: String,

    /// Путь к файлу для чтения
    file_path: String,

    /// Игнорировать регистр при поиске
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    let config = Config {
        query: args.query,
        file_path: args.file_path,
        ignore_case: args.ignore_case,
    };

    if let Err(e) = run(config) {
        eprintln!("Ошибка приложения: {e}");
        process::exit(1);
    }
}

use microgrep::{Config, run};
use std::env;
use std::process;

// Для запуска с выводом в файл: cargo run -- to poem.txt > output.txt
// Для запуска: cargo run -- to poem.txt
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

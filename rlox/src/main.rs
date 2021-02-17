use std::io::{self, BufRead, Write};
use std::process::exit;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    match args.len() {
        0 => repl(),
        1 => run_file(args.first().unwrap()),
        _ => usage(),
    }
}

fn run_file(path: &str) {
    let source = std::fs::read_to_string(path).unwrap();
    rlox::interpret(&source)
}

fn repl() {
    let stdin = io::stdin();

    let prompt = || {
        print!("> ");
        io::stdout().flush().unwrap();
    };

    prompt();
    for line in stdin.lock().lines() {
        rlox::interpret(&line.unwrap());
        prompt();
    }
}

fn usage() {
    eprintln!("Usage: rlox [path]");
    exit(64);
}

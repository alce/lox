use std::io::{self, BufRead, Write};
use std::process::exit;

use rlox::LoxError;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    match args.len() {
        0 => repl(),
        1 => run_file(args.first().unwrap()),
        _ => usage(),
    }
}

fn run_file(path: &str) {
    let src = std::fs::read_to_string(path).unwrap();

    if let Err(e) = rlox::interpret(src.trim()) {
        match e {
            LoxError::Compile(_) => exit(65),
            LoxError::Runtime => exit(70),
        }
    }
}

fn repl() {
    let prompt = || {
        print!("> ");
        io::stdout().flush().unwrap();
    };

    prompt();
    for line in io::stdin().lock().lines() {
        rlox::interpret(&line.unwrap()).unwrap();
        prompt();
    }
}

fn usage() {
    eprintln!("Usage: rlox [path]");
    exit(64);
}

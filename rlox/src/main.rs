use std::io::{self, BufRead, Write};
use std::process::exit;

use rlox::{Interpreter, LoxError};

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
            LoxError::Compile(e) => {
                eprintln!("{}", e);
                exit(65);
            }
            LoxError::Runtime(msg, line) => {
                eprintln!("{}", msg);
                eprintln!("[line {}]", line);
                exit(70)
            }
            _ => panic!("RET leaked"),
        }
    }
}

fn repl() {
    let prompt = || {
        print!("> ");
        io::stdout().flush().unwrap();
    };

    let mut interpreter = Interpreter::new();

    prompt();
    for line in io::stdin().lock().lines() {
        match rlox::parse(&line.unwrap()) {
            Ok(stmts) => {
                if let Err(e) = interpreter.interpret(stmts) {
                    println!("{}", e)
                }
            }
            Err(e) => println!("{}", e),
        }

        prompt();
    }
}

fn usage() {
    eprintln!("Usage: rlox [path]");
    exit(64);
}

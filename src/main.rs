mod ast;
mod ast_printer;
mod lox;
pub mod parser;
mod scanner;
mod token;

use lox::Lox;
use std::{env, process};

fn main() {
    let mut lox: Lox = Lox::new();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: rust-lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}

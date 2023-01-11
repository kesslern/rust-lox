use std::{
    fs::File,
    io::{Read, Write},
    process,
};

use crate::error::{Error, Builder, ErrorType};
use crate::interpreter::interpret;
use crate::parser::parse;
use crate::scanner::scan;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) {
        // Open the file
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                println!("Could not open file: {}", error);
                process::exit(65);
            }
        };

        // Load the file into a string
        let contents: &mut String = &mut String::new();
        match file.read_to_string(contents) {
            Ok(_) => (),
            Err(error) => {
                println!("Could not read file: {}", error);
                process::exit(66);
            }
        };

        self.run(contents);
        if self.had_error {
            process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            match std::io::stdout().flush() {
                Ok(_) => (),
                Err(_) => panic!("Error writing to stdout"),
            }
            let mut line = String::new();

            match std::io::stdin().read_line(&mut line) {
                Ok(_) => (),
                Err(_) => panic!("Error reading user input"),
            };

            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            self.run(line);
            self.had_error = false;
        }
    }

    pub fn run(&mut self, source: &str) {
        let tokens = scan(source);
        match tokens {
            Ok(tokens) => {
                let expr = parse(tokens);
                match expr {
                    Ok(expr) => {
                        // TODO: InterpreterError enum type
                        interpret(&expr).unwrap_or_else(|e| self.error(&Error {
                            error_type: ErrorType::Runtime,
                            message: e,
                            token: None,
                            line: None,
                        }));
                    }
                    Err(error) => {
                        self.error(&error);
                    }
                }
            }
            Err(error) => {
                self.error(&Builder::new(ErrorType::Parse, error).build());
            }
        }
    }

    pub fn error(&mut self, error: &Error) {
        Lox::report(error.line, error.token.as_deref(), &error.message);
        self.had_error = true;
    }

    pub fn report(line: Option<usize>, token: Option<&str>, message: &str) {
        if let Some(line) = line {
            eprint!("[line {}] Error", line);
        } else {
            eprint!("Error");
        }

        if let Some(token) = token {
            eprint!(" at '{}': ", token);
        } else {
            eprint!(": ");
        }

        eprintln!("{}", message);
    }
}

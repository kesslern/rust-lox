use std::{
    fs::File,
    io::{Read, Write},
    process,
};

use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&self, path: &str) {
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
                Err(_) => panic!("idk"),
            };

            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            self.run(line);
            self.had_error = false;
        }
    }

    pub fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token);
        }
    }

    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    pub fn report(line: usize, where_: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, where_, message);
    }
}

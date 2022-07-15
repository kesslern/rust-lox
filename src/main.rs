use std::{
    env,
    fs::File,
    io::{Read, Write},
    process,
};

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

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { had_error: false }
    }

    fn run_file(&self, path: &str) {
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

        self.run(&contents);
        if self.had_error {
            process::exit(65);
        }
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            self.run(&line);
            self.had_error = false;
        }
    }

    fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(line: i32, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: i32, where_: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, where_, message);
    }
}

struct Scanner {}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {}
    }

    pub fn scan_tokens(&mut self) -> Vec<&str> {
        return Vec::new();
    }
}

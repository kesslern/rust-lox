use std::{
    env,
    fmt::Display,
    fs::File,
    io::{Read, Write},
    process,
};

use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

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
        scanner.scan_tokens();

        // for token in tokens {
        //     println!("{}", token);
        // }
    }

    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, where_: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, where_, message);
    }
}

struct Scanner<'a> {
    source: GraphemeIndices<'a>,
    tokens: Vec<Token>,
    current: String,
    line: usize,
}

enum ScanStatus {
    Working,
    Finished,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        Scanner {
            source: source.grapheme_indices(true),
            tokens: Vec::new(),
            current: String::new(),
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        match self.scan_token() {
            ScanStatus::Working => (),
            ScanStatus::Finished => {
                self.tokens.push(Token::new(
                    TokenType::Eof,
                    String::from(""),
                    None,
                    self.line,
                ));
            }
        }
    }

    fn scan_token(&mut self) -> ScanStatus {
        let c = self.advance();

        match c {
            None => ScanStatus::Finished,
            Some(s) => {
                match s.as_str() {
                    "(" => self.add_token(TokenType::LeftParen),
                    ")" => self.add_token(TokenType::RightParen),
                    "{" => self.add_token(TokenType::LeftBrace),
                    "}" => self.add_token(TokenType::RightBrace),
                    "," => self.add_token(TokenType::Comma),
                    "." => self.add_token(TokenType::Dot),
                    "-" => self.add_token(TokenType::Minus),
                    "+" => self.add_token(TokenType::Plus),
                    ";" => self.add_token(TokenType::Semicolon),
                    "*" => self.add_token(TokenType::Star),
                    _ => Lox::error(self.line, &format!("Unexpected character: {}", s)),
                };
                ScanStatus::Working
            }
        }
    }

    fn advance(&mut self) -> Option<String> {
        let result = self.source.next();
        return match result {
            Some((_, r)) => Some(r.to_string()),
            None => None,
        };
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            self.current.clone(),
            None,
            self.line,
        ));
        self.current = String::new();
    }

    fn match_next(&mut self) {}
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        return Token {
            token_type,
            lexeme,
            literal,
            line,
        };
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self.literal {
            Some(l) => write!(f, "{:?} {} {}", self.token_type, self.lexeme, l),
            None => write!(f, "{:?} {}", self.token_type, self.lexeme),
        };
    }
}

#[derive(Clone, Copy)]
union Literal {
    number: usize,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "{}", self.number) }
    }
}

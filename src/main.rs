use std::{
    env,
    fmt::Display,
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
            println!("{}", token);
        }
    }

    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, where_: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, where_, message);
    }
}

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));

        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token);
            }
            '=' => {
                let token = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token);
            }
            '<' => {
                let token = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(token);
            }
            '>' => {
                let token = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token);
            }
            ' ' | '\r' | '\t' => (), // Ignore
            '\n' => self.line += 1,
            '"' => self.string(),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number(),
            _ => {
                if Scanner::is_alpha(Some(c)) {
                    self.indentifier();
                } else {
                    Lox::error(self.line, &format!("Unexpected character: {}", c))
                }
            }
        };
    }

    fn advance(&mut self) -> char {
        let result = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        result
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source.as_bytes()[self.current] as char)
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source.as_bytes()[self.current + 1] as char)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];

        self.tokens
            .push(Token::new(token_type, text.to_string(), None, self.line));
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal<'a>) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            Some(literal),
            self.line,
        ));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            if self.is_at_end() {
                Lox::error(self.line, "Unterminated string.");
                return;
            }

            self.advance();
        }

        // Consume the closing quote
        self.advance();

        let literal = Literal {
            literal_type: LiteralType::String,
            value: LiteralValue {
                string: &self.source[self.start + 1..self.current - 1],
            },
        };

        self.add_token_literal(TokenType::String, literal);
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let literal = Literal {
            literal_type: LiteralType::Number,
            value: LiteralValue {
                number: self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            },
        };

        self.add_token_literal(TokenType::Number, literal);
    }

    fn is_digit(c: Option<char>) -> bool {
        match c {
            Some('0'..='9') => true,
            _ => false,
        }
    }

    fn is_alpha(c: Option<char>) -> bool {
        match c {
            Some('a'..='z') | Some('A'..='Z') | Some('_') => true,
            _ => false,
        }
    }

    fn is_alphanumeric(c: Option<char>) -> bool {
        Scanner::is_digit(c) || Scanner::is_alpha(c)
    }

    fn indentifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let token_type = match Scanner::match_keyword(text) {
            Some(token_type) => token_type,
            None => TokenType::Identifier,
        };

        self.add_token(token_type);
    }

    fn match_keyword(text: &str) -> Option<TokenType> {
        match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
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

struct Token<'a> {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal<'a>>,
    line: usize,
}

impl Token<'_> {
    fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        return Token {
            token_type,
            lexeme,
            literal,
            line,
        };
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self.literal {
            Some(l) => write!(f, "{:?} {} {}", self.token_type, self.lexeme, l),
            None => write!(f, "{:?} {}", self.token_type, self.lexeme),
        };
    }
}

enum LiteralType {
    String,
    Number,
}

struct Literal<'a> {
    literal_type: LiteralType,
    value: LiteralValue<'a>,
}

union LiteralValue<'a> {
    number: f64,
    string: &'a str,
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self.literal_type {
                LiteralType::String => write!(f, "\"{}\"", self.value.string),
                LiteralType::Number => write!(f, "{}", self.value.number),
            }
        }
    }
}

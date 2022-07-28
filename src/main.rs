mod ast;
mod ast_printer;
mod lox;
mod scanner;
mod token;

use ast::{Expr, Visitor};
use ast_printer::AstPrinter;
use lox::Lox;
use std::{env, process};
use token::{Literal, Token, TokenType};

fn main() {
    let mut lox: Lox = Lox::new();

    let ex = Expr::Binary(
        Box::new(Expr::Unary(
            Box::new(Token::new(TokenType::Minus, "-".to_owned(), None, 1)),
            Box::new(Expr::Literal(Box::new(Literal::Number(123.0)))),
        )),
        Token::new(TokenType::Star, "*".to_owned(), None, 1),
        Box::new(Expr::Grouping(Box::new(Expr::Literal(Box::new(
            Literal::Number(45.67),
        ))))),
    );
    let printer = AstPrinter;
    println!("{}", printer.visit_expr(&ex));

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

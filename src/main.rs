extern crate core;

use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use logos::{Logos, Span};
use crate::token::{Token, TokenKind};

mod token;
mod parser;
mod error;
mod node;
mod printer;

#[derive(Parser, Debug)]
#[command(name = "mish")]
struct Args {
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut file = OpenOptions::new()
        .read(true)
        .open(args.file)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    println!("\r0/3");
    
    let start = Instant::now();
    
    let mut lex = TokenKind::lexer(source.as_str());

    println!("\r1/3");
    
    let mut tokens = Vec::new();
    while let Some(Ok(kind)) = lex.next() {
        tokens.push(Token{
            kind,
            span: lex.span(),
        });
    }
    tokens.push(Token{
        kind: TokenKind::EOF,
        span: Span { start: 0, end: 0 }
    });

    println!("\r2/3");
    
    let mut parse = parser::Parser::new(tokens.as_slice(), &source);
    let nodes = parse.run();
    if nodes.is_err() {
        std::process::exit(1);
    }

    println!("\r3/3");
    
    printer::ASTPrinter::new(&printer::ASTContext { tokens: tokens.as_slice(), source: &source });

    let time = start.elapsed();
    println!("{}", time.as_nanos());

    Ok(())
}

use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use logos::Logos;
use crate::token::{Token, TokenKind};

mod token;
mod parser;
mod error;

#[derive(Parser, Debug)]
#[command(name = "talon")]
struct Args {
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut file = OpenOptions::new()
        .read(true)
        .open(args.file).expect("could not open file");
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    
    let start = Instant::now();
    
    let mut lex = TokenKind::lexer(source.as_str());
    let mut tokens = Vec::new();
    while let Some(Ok(kind)) = lex.next() {
        tokens.push(Token{
            kind,
            span: lex.span(),
        });
    }
    
    let time = start.elapsed();
    println!("{}", time.as_nanos());

    Ok(())
}

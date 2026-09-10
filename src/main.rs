use std::path::PathBuf;
use clap::Parser;

mod token;

#[derive(Parser, Debug)]
#[command(name = "talon")]
struct Args {
    file: PathBuf,
}

fn main() {
    let args = Args::parse();


}

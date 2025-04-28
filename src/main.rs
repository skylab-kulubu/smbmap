use clap::Parser;
mod parser;
use crate::parser::Cli;
fn main() {
    let args = Cli::parse();
    if args.user == "null" || args.password == "null" {
        println!("null user");
    } else {
        println!("Username: {}", args.user);
        println!("Password: {}", args.password);
    }
}

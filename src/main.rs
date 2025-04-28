use clap::Parser;
mod parser;
use crate::parser::Cli;
fn main() {
    let args = Cli::parse();
    match args.user {
        Some(user) => {
            println!("User: {}", user);
            println!("Password: {}", args.password);
        },
        None => {
            print!("null user")
        }
    }
    match args.share {
        Some(share) => {
            println!("Share: {}", share);
        },
        None => {
            print!("no share enum")
        }
        
    }
}

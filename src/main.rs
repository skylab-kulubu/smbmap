use core::panic;

use clap::Parser;
mod parser;
use crate::parser::Cli;
fn main() {
    let args = Cli::parse();
    match args.user {
        Some(user) => {
            println!("User: {}", user);
            match args.password {
                Some(password ) => {
                    println!("Password: {}",password)
                },
                None => {
                    panic!("No password provided!")
                }
            }
        },
        None => {
            println!("null user")
        }
    }
    match args.share {
        Some(share) => {
            println!("Share: {}", share);
        },
        None => {
            println!("no share enum")
        }
        
    }
}

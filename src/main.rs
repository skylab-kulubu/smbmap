use clap::Parser;
mod parser;
use crate::parser::Cli;
use log::{
    error,
    info,
    warn,
};
fn main() {
    let args = Cli::parse();
    env_logger::init();
    info!("Target: {}", args.target);
    info!("Port: {}", args.port);
    match args.domain {
        Some(domain) => {
            info!("Domain: {}", domain);
        },
        None => {
            info!("No domain provided.");
        }
    }
    match args.user {
        Some(user) => {
            info!("User: {}", user);
            match args.password {
                Some(password ) => {
                    info!("Password: {}", password);
                },
                None => {
                    error!("No password provided");
                }
            }
        },
        None => {
            info!("No user provided, using null user.");
        }
    }
    match args.share {
        Some(share) => {
            info!("Share: {}", share);
        },
        None => {
            info!("No share provided, no share enum.");
        }
        
    }
}

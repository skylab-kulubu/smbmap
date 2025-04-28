use std::process::exit;

use clap::Parser;
mod parser;
use crate::parser::Cli;
#[allow(unused_imports)]
use log::{
    error,
    info,
    warn,
};
use std::env;
fn main() {
    let args = Cli::parse();
    dbg!(&args);
    if args.verbose == 0 {
        unsafe {
        env::set_var("RUST_LOG", "error");
        }
    }else if args.verbose == 1 {
        unsafe {
        env::set_var("RUST_LOG", "warn");
        }
    }else if args.verbose > 2 {
        unsafe {
        env::set_var("RUST_LOG", "info");
        }
    }
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
                    exit(1)
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

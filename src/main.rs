use clap::Parser;
mod parser;
mod smbshares;
use crate::parser::Cli;
use crate::smbshares::*;
#[allow(unused_imports)]
use log::{error, info, warn};
use std::env;
fn main() {
    let args = Cli::parse();
    dbg!(&args);
    set_verbosity(&args);
    env_logger::init();
    info!("Target: {}", args.target);
    info!("Port: {}", args.port);
    print_infos(&args);
    list_shares(&args);
    let client = match (&args.user, &args.password, &args.share) {
        (Some(user), Some(password), Some(share)) => {
            get_smbclient_with_login(&args, user, password, share)
        }
        (None, None, _) => get_smbclient_guest(&args),
        (None, Some(_), _) => get_smbclient_guest(&args),
        (Some(_), None, _) => get_smbclient_guest(&args),
        (Some(_), Some(_), None) => get_smbclient_guest(&args),
    };
    dir_share(&client, &args.path.unwrap_or("".to_string()));
}

fn set_verbosity(args: &Cli) {
    if args.verbose == 0 {
        unsafe {
            env::set_var("RUST_LOG", "error");
        }
    } else if args.verbose == 1 {
        unsafe {
            env::set_var("RUST_LOG", "warn");
        }
    } else if args.verbose > 2 {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
}

fn print_infos(args: &Cli) {
    match &args.domain {
        Some(domain) => {
            info!("Domain: {}", domain);
        }
        None => {
            info!("No domain provided.");
        }
    }
    match &args.user {
        Some(user) => {
            info!("User: {}", user);
            match &args.password {
                Some(password) => {
                    info!("Password: {}", password);
                }
                None => {
                    warn!("No password provided with user.");
                }
            }
        }
        None => {
            info!("No user provided, using null user.");
        }
    }
    match &args.share {
        Some(share) => {
            info!("Share: {}", share);
        }
        None => {
            info!("No share provided, no share enum.");
        }
    }
}

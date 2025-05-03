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
    #[cfg(debug_assertions)]
    dbg!(&args);
    set_verbosity(&args);
    env_logger::init();
    info!("Target: {}", args.target);

    match args.command {
        parser::Commands::Smb {
            user,
            password,
            port,
            domain,
            share,
            path,
        } => {
            info!("Command: Smb");
            info!("User: {:?}", user);
            info!("Password: {:?}", password);
            info!("Port: {}", port);
            info!("Domain: {:?}", domain);
            info!("Share: {:?}", share);
            info!("Path: {:?}", path);

            match share {
                Some(share) => {
                    let client = match (user, password, share) {
                        (Some(user), Some(password), share) => get_smbclient_with_login(
                            &args.target,
                            &port,
                            Some(&user),
                            Some(&password),
                            Some(&share),
                        ),
                        (None, None, _) => get_smbclient_guest(&args.target, &port),
                        (None, Some(_), _) => get_smbclient_guest(&args.target, &port),
                        (Some(_), None, _) => get_smbclient_guest(&args.target, &port),
                    };
                    dir_share(Some(&client), &path.unwrap_or("".to_string()));
                }
                None => list_shares(
                    Some(&user.clone().unwrap()),
                    Some(&password.clone().unwrap()),
                    Some(&"".to_string()),
                    &args.target,
                    &port,
                ),
            }
        }
    }
    {}
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

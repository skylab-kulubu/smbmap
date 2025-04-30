use clap::Parser;
use pavao::{SmbClient, SmbCredentials, SmbOpenOptions, SmbOptions};
use std::process::exit;
mod parser;
use crate::parser::Cli;
#[allow(unused_imports)]
use log::{error, info, warn};
use prettytable::{Cell, Row, Table};
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
}

fn list_shares(args: &Cli) {
    match (&args.user, &args.password) {
        (Some(user), Some(password)) => {
            let client = SmbClient::new(
                SmbCredentials::default()
                    .server(format!("smb://{}:{}", &args.target, &args.port))
                    .share("")
                    .password(password.to_string())
                    .username(user.to_string()),
                SmbOptions::default()
                    .case_sensitive(true)
                    .one_share_per_server(true),
            )
            .unwrap();
            let shares = match client.list_dir("") {
                Ok(shares) => shares,
                Err(e) => {
                    error!("Error listing shares: {}", e);
                    exit(1);
                }
            };
            let mut table = Table::new();
            table.add_row(Row::new(vec![
                Cell::new("Share"),
                Cell::new("Type"),
                Cell::new("Comment"),
            ]));
            for share in shares {
                table.add_row(Row::new(vec![
                    Cell::new(share.name()),
                    Cell::new(&format!("{:?}", share.get_type())),
                    Cell::new(share.comment()),
                ]));
            }
            table.printstd();
        }
        (None, None) => {
            info!("No user provided, using null user.");
        }
        (None, Some(_)) => info!("No user provided, using null user."),
        (Some(_), None) => info!("No password provided, using null user."),
    }
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
                    error!("No password provided");
                    exit(1)
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

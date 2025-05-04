use clap::Parser;
mod parser;
mod smbshares;
use crate::parser::Cli;
use crate::smbshares::*;
#[allow(unused_imports)]
use log::{error, info, warn};
use std::{env, process::exit};
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    #[cfg(debug_assertions)]
    dbg!(&args);
    let _ = set_verbosity(&args);
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
            tree,
        } => {
            info!("Command: Smb");
            info!("User: {:?}", user);
            info!("Password: {:?}", password);
            info!("Port: {}", port);
            info!("Domain: {:?}", domain);
            info!("Share: {:?}", share);
            info!("Path: {:?}", path);
            warn!("Tree View: {:?}", tree);
            match tree {
                true => match share {
                    Some(share) => {
                        let client = match (user, password, share) {
                            (Some(user), Some(password), share) => {
                                get_smbclient_with_login(
                                    &args.target,
                                    &port,
                                    Some(&user),
                                    Some(&password),
                                    Some(&share),
                                )
                                .await
                            }
                            (None, None, _) => get_smbclient_guest(&args.target, &port).await,
                            (None, Some(_), _) => get_smbclient_guest(&args.target, &port).await,
                            (Some(_), None, _) => get_smbclient_guest(&args.target, &port).await,
                        };
                        print_tree_view(
                            Some(&client),
                            &path.unwrap_or("".to_string()),
                            0,
                        )
                        .await;
                    }
                    None => {
                        error!("Share is required for tree view");
                        error!("Please provide a share name with -S or --share");
                        exit(1)
                    }
                },
                false => match share {
                    Some(share) => {
                        let client = match (user, password, share) {
                            (Some(user), Some(password), share) => {
                                get_smbclient_with_login(
                                    &args.target,
                                    &port,
                                    Some(&user),
                                    Some(&password),
                                    Some(&share),
                                )
                                .await
                            }
                            (None, None, _) => get_smbclient_guest(&args.target, &port).await,
                            (None, Some(_), _) => get_smbclient_guest(&args.target, &port).await,
                            (Some(_), None, _) => get_smbclient_guest(&args.target, &port).await,
                        };
                        dir_share(Some(&client), &path.unwrap_or("".to_string())).await;
                    }
                    None => {
                        info!("No share provided, listing all shares");
                        list_shares(
                            Some(&user.clone().unwrap()),
                            Some(&password.clone().unwrap()),
                            Some(&"".to_string()),
                            &args.target,
                            &port,
                        )
                        .await
                    }
                },
            }
        }
    }
    {}
}

async fn set_verbosity(args: &Cli) {
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

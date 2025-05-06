use clap::Parser;
mod parser;
mod smb;
mod utils;
use crate::parser::Cli;
use crate::smb::*;
use crate::utils::print_banner;
#[allow(unused_imports)]
use log::{error, info, warn};
use std::{env, process::exit};
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if args.disable_banner {
        tokio::spawn(print_banner());
    }
    #[cfg(debug_assertions)]
    dbg!(&args);
    tokio::spawn(set_verbosity(args.verbose.clone()));
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
            mut read_file,
            write_file,
        } => {
            info!("Command: Smb");
            info!("User: {:?}", user);
            info!("Password: {:?}", password);
            info!("Port: {}", port);
            info!("Domain: {:?}", domain);
            info!("Share: {:?}", share);
            info!("Path: {:?}", path);
            warn!("Tree View: {:?}", tree);
            if let Some(ref path) = path {
                if path.contains('.') {
                    read_file = Some(path.clone());
                }
            }
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
                        print_tree_view(Some(&client), &path.unwrap_or("".to_string()), 0).await;
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
                        if read_file != None {
                            read_file_func(&client, &read_file.unwrap()).await;
                            if write_file != None {
                                write_file_func(
                                    &client,
                                    &path.expect("You have to provide a path to install a file!"),
                                    &write_file.unwrap(),
                                )
                                .await;
                            }
                        } else {
                            dir_share(Some(&client), &path.unwrap_or("".to_string())).await;
                        }
                    }
                    None => {
                        info!("No share provided, listing all shares");
                        list_shares(
                            Some(&user.clone().unwrap_or("".to_string())),
                            Some(&password.clone().unwrap_or("".to_string())),
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
}

async fn set_verbosity(verbose: u8) {
    let verbose = verbose as u32;
    if verbose == 0 {
        unsafe {
            env::set_var("RUST_LOG", "error");
        }
    } else if verbose == 1 {
        unsafe {
            env::set_var("RUST_LOG", "warn");
        }
    } else if verbose > 2 {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
}

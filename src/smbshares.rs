use crate::parser::Cli;
use log::warn;
use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbOptions};
use prettytable::{Cell, Row, Table};

pub fn dir_share(client: &SmbClient, path: &str) {
    let files= match client.list_dir(path) {
        Ok(shares) => shares,
        Err(_) => {
            let shares: Vec<SmbDirent> = Vec::new();
            shares
        }
    };
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Type"),
        Cell::new("Comment"),
    ]));
    for file in files {
        table.add_row(Row::new(vec![
            Cell::new(file.name()),
            Cell::new(&format!("{:?}", file.get_type())),
            Cell::new(file.comment()),
        ]));
    }
    table.printstd();
}

pub fn list_shares(args: &Cli) {
    match (&args.user, &args.password) {
        (Some(user), Some(password)) => {
            let client = get_smbclient_with_login(args, user, password,args.share.as_ref().unwrap());
            let shares = match client.list_dir("") {
                Ok(shares) => shares,
                Err(_) => {
                    let shares: Vec<SmbDirent> = Vec::new();
                    shares
                }
            };
            print_shares_table(shares);
        }
        (None, None) => {
            list_shares_without_login(args);
        }
        (None, Some(_)) => list_shares_without_login(args),
        (Some(_), None) => list_shares_without_login(args),
    }
}

pub fn get_smbclient_with_login(args: &Cli, user: &String, password: &String,share: &String) -> SmbClient {
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{}:{}", &args.target, &args.port))
            .share(share)
            .password(password.to_string())
            .username(user.to_string()),
        SmbOptions::default()
            .case_sensitive(true)
            .one_share_per_server(true),
    )
    .unwrap();
    client
}

fn list_shares_guest(args: &Cli) {
    let client = get_smbclient_guest(args);
    let shares = match client.list_dir("") {
        Ok(shares) => shares,
        Err(_) => {
            let shares: Vec<SmbDirent> = Vec::new();
            shares
        }
    };
    print_shares_table(shares);
}

pub fn get_smbclient_guest(args: &Cli) -> SmbClient {
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{}:{}", &args.target, &args.port))
            .share("")
            .password(" ")
            .username(" "),
        SmbOptions::default()
            .case_sensitive(true)
            .one_share_per_server(true),
    )
    .unwrap();
    client
}

fn list_shares_without_login(args: &Cli) {
    warn!("No user or password provided, using null user.");
    let client = get_smbclient_null(args);
    let shares = match client.list_dir("") {
        Ok(shares) => shares,
        Err(_) => {
            let shares: Vec<SmbDirent> = Vec::new();
            shares
        }
    };
    print_shares_table(shares);
    list_shares_guest(args);
}

pub fn get_smbclient_null(args: &Cli) -> SmbClient {
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{}:{}", &args.target, &args.port))
            .share("")
            .password("")
            .username(""),
        SmbOptions::default()
            .case_sensitive(true)
            .one_share_per_server(true),
    )
    .unwrap();
    client
}

fn print_shares_table(shares: Vec<pavao::SmbDirent>) {
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

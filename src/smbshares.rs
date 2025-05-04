use log::warn;
use pavao::{SmbClient, SmbCredentials, SmbDirent, SmbDirentType, SmbOptions};
use prettytable::{Cell, Row, Table};
use std::future::Future;
use std::pin::Pin;

pub async fn dir_share(client: Option<&SmbClient>, path: &str) {
    let path = ensure_leading_slash(path).await;
    let files = match client.unwrap().list_dir(path) {
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

async fn ensure_leading_slash(path: &str) -> String {
    if !path.starts_with("/") {
        format!("{}{}", "/", path)
    } else {
        path.to_string()
    }
}

pub async fn list_shares(
    user: Option<&String>,
    password: Option<&String>,
    share: Option<&String>,
    target: &String,
    port: &u16,
) {
    match (user, password) {
        (Some(user), Some(password)) => {
            let client =
                get_smbclient_with_login(&target, port, Some(&user), Some(&password), share).await;
            let shares = match client.list_dir("") {
                Ok(shares) => shares,
                Err(_) => {
                    let shares: Vec<SmbDirent> = Vec::new();
                    shares
                }
            };
             print_shares_table(&shares).await;
        }
        (None, None) => {
             list_shares_without_login(target, port).await;
        }
        (None, Some(_)) => list_shares_without_login(&target, port).await,
        (Some(_), None) => list_shares_without_login(&target, port).await,
    }
}

pub async fn get_smbclient_with_login(
    target: &String,
    port: &u16,
    user: Option<&String>,
    password: Option<&String>,
    share: Option<&String>,
) -> SmbClient {
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{}:{}", target, port))
            .share(share.unwrap())
            .password(password.unwrap())
            .username(user.unwrap()),
        SmbOptions::default()
            .case_sensitive(true)
            .one_share_per_server(true),
    )
    .unwrap();
    client
}

pub async fn get_smbclient_guest(target: &String, port: &u16) -> SmbClient {
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{:?}:{:?}", target, port))
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

pub async fn list_shares_without_login(target: &String, port: &u16) {
    warn!("No user or password provided, using null user.");
    let client = get_smbclient_null(target, port).await;
    let shares = match client.list_dir("") {
        Ok(shares) => shares,
        Err(_) => {
            let shares: Vec<SmbDirent> = Vec::new();
            shares
        }
    };
    print_shares_table(&shares).await;
}

pub async fn get_smbclient_null(target: &String, port: &u16) -> SmbClient {
    let client = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{:?}:{:?}", target, port))
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

pub async fn print_shares_table(shares: &Vec<pavao::SmbDirent>) {
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

pub fn print_tree_view<'a>(
    client: Option<&'a SmbClient>,
    path: &'a String,
    depth: usize,
) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        let path = ensure_leading_slash(path).await;
        let files = match client.unwrap().list_dir(&path) {
            Ok(entries) => entries,
            Err(_) => {
                warn!("Failed to list directory: {}", &path);
                return;
            }
        };

        for file in files {
            let indent = " -".repeat(depth);
            println!("{}- {}", indent, file.name());

            if file.get_type() == SmbDirentType::Dir {
                let sub_path = format!("{}/{}", path.trim_end_matches('/'), file.name());
                print_tree_view(client, &sub_path, depth + 1).await;
            }
        }
    })
}

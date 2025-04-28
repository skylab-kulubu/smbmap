use clap::Parser;
#[derive(Parser)]
#[command(name = "AD-Pentest")]
pub struct Cli {
    #[arg(short = 'u', long = "user", default_value = "null")]
    pub user: String,

    #[arg(short = 'p', long = "password", default_value= "null")]
    pub password: String,
}
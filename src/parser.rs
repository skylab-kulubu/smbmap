use clap::Parser;
#[derive(Parser,Debug)]
#[command(name = "AD-Pentest")]
pub struct Cli {
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(short = 'u', long = "user")]
    pub user: Option<String>,

    #[arg(short = 'p', long = "password")]
    pub password: Option<String>,

    #[arg(short = 't', long = "target")]
    pub target: String,

    #[arg(short = 'P', long = "port", default_value = "445")]
    pub port: u16,

    #[arg(short = 'D', long = "domain")]
    pub domain: Option<String>,

    #[arg(short = 'S',long = "share")]
    pub share: Option<String>,

    #[arg(long)]
    pub path: Option<String>,
}
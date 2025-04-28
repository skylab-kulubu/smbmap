use clap::Parser;
#[derive(Parser)]
#[command(name = "AD-Pentest")]
pub struct Cli {
    #[arg(short = 'u', long = "user")]
    pub user: Option<String>,

    #[arg(short = 'p', long = "password")]
    pub password: Option<String>,

    #[arg(short = 't', long = "target")]
    pub target: String,

    #[arg(long)]
    pub share: Option<String>,
}
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "AD-Pentest")]
pub struct Cli {
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8, // Genel bir argüman olarak taşındı

    #[arg(short = 't', long = "target")]
    pub target: String,

    #[command(subcommand)]
    pub command: Commands,

    #[arg(long = "disable-banner",action = clap::ArgAction::SetFalse)]
    pub disable_banner: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Smb {
        #[arg(short = 'u', long = "user")]
        user: Option<String>,

        #[arg(short = 'p', long = "password")]
        password: Option<String>,

        #[arg(short = 'P', long = "port", default_value = "445")]
        port: u16,

        #[arg(short = 'D', long = "domain")]
        domain: Option<String>,

        #[arg(short = 'S', long = "share")]
        share: Option<String>,

        #[arg(long)]
        path: Option<String>,

        #[arg(long, action = clap::ArgAction::SetTrue)]
        tree: bool,

        /// You can provide a files to this paramater or just write the path of the file to --path paramater
        #[arg(long = "read-file")]
        read_file: Option<String>,

        /// Install the file provived in --path or --read-file
        #[arg(long = "write-file",action = clap::ArgAction::SetTrue)]
        write_file: Option<String>,

        #[arg(long = "less-info",action = clap::ArgAction::SetFalse)]
        less_info: bool,
    },
}

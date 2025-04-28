# Creating New Argument Parser

## Packages

```toml name=Cargo.toml
[dependencies]
clap = { version = "4.1", features = ["derive"] }
```

## Sample

```rust name=main.rs
use clap::Parser;
#[derive(Parser,Debug)]
#[command(name = "AD-Pentest")]
pub struct Cli {
    // Vaule increases with letter count, -v -vv -vvv etc.
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8,
    
    // Optional
    #[arg(short = 'u', long = "user")]
    pub user: Option<String>,

    // Required
    #[arg(short = 't', long = "target")]
    pub target: String,

    // Has default value if it is not setted
    #[arg(short = 'P', long = "port", default_value = "445")]
    pub port: u16,
}
```
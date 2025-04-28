# Creating New Argument Parser

## Packages

```toml name=Cargo.toml
[dependencies]
clap = { version = "4.1", features = ["derive"] }
```

## Sample

```rust name=main.rs
use clap::Parser;

/// Program to process user credentials
#[derive(Parser)]
#[command(name = "myprogram")]
#[command(about = "A simple program to handle user credentials", long_about = None)]
struct Cli {
    /// Username to authenticate
    #[arg(short = 'u', long = "user")]
    user: String,

    /// Password to authenticate
    #[arg(short = 'p', long = "password")]
    password: String,
}

fn main() {
    let args = Cli::parse();

    println!("Username: {}", args.user);
    println!("Password: {}", args.password);
}
```

## Usage

### Development

```bash
cargo run -- -u alice -p wonderland123
```

```bash
cargo run -- --user alice --password wonderland123
```


### Release

```bash
./ad-pentest --user alice --pasword wonderland123
```

```bash
./ad-pentest -u alice -p wonderland123
```
## Modules and Imports

### External Crates

- **`clap::Parser`**: Used for parsing command-line arguments.
- **`log::{error, info, warn}`**: Provides logging functionality at different levels (error, info, warning).
- **`tokio::main`**: Marks the `main` function as asynchronous, enabling the use of asynchronous operations.
- **`std::{env, process::exit}`**: Provides access to environment variables and process termination.

### Internal Modules

- **`parser`**: Contains the `Cli` struct and command definitions for parsing user input.
- **`smbshares`**: Provides functions for interacting with SMB shares, such as connecting to shares, listing directories, and displaying tree views.

---

## Key Functions

### `main()`

The asynchronous entry point of the application. It performs the following tasks:

1. **Parse Command-Line Arguments**: Uses the `Cli` struct from the `parser` module to parse user input.
2. **Set Logging Verbosity**: Calls `set_verbosity` to configure the logging level based on the `--verbose` flag.
3. **Initialize Logger**: Initializes the `env_logger` for logging.
4. **Execute Commands**: Matches the parsed command and executes the corresponding functionality.

#### Command Handling

- **`Smb` Command**: Handles operations related to SMB shares. The command supports the following options:
  - `user`: Optional username for authentication.
  - `password`: Optional password for authentication.
  - `port`: Port number for the SMB service.
  - `domain`: Optional domain for authentication.
  - `share`: Name of the SMB share.
  - `path`: Path within the SMB share.
  - `tree`: Boolean flag to display a tree view of the share.

#### SMB Command Logic

- **Tree View (`tree = true`)**:
  - Requires a `share` to be specified.
  - Connects to the SMB share using the provided credentials or as a guest.
  - Calls `print_tree_view` to display the directory structure recursively.
- **Directory Listing (`tree = false`)**:
  - If a `share` is provided:
    - Connects to the SMB share and lists the contents of the specified `path` using `dir_share`.
  - If no `share` is provided:
    - Lists all available shares using `list_shares`.

### `set_verbosity(args: &Cli)`

Configures the logging level based on the `--verbose` flag:

- `0`: Logs only errors.
- `1`: Logs warnings and errors.
- `2+`: Logs informational messages, warnings, and errors.

---

## Key Functions from `smbshares` Module

- **`get_smbclient_with_login`**: Connects to an SMB share using the provided credentials.
- **`get_smbclient_guest`**: Connects to an SMB share as a guest.
- **`print_tree_view`**: Recursively displays the directory structure of an SMB share.
- **`dir_share`**: Lists the contents of a specific directory in an SMB share.
- **`list_shares`**: Lists all available SMB shares on the target.

---

## Logging

The application uses the `log` crate for logging and `env_logger` for log output. The verbosity level is controlled by the `--verbose` flag.

---

## Error Handling

- If a required parameter (e.g., `share` for tree view) is missing, the application logs an error and terminates using `process::exit(1)`.

---

## Example Usage

### Help

```bash
Usage: ad-pentest [OPTIONS] --target <TARGET> <COMMAND>

Commands:
  smb   
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...       
  -t, --target <TARGET>  
  -h, --help             Print help
```

```bash
Usage: ad-pentest --target <TARGET> smb [OPTIONS]

Options:
  -u, --user <USER>          
  -p, --password <PASSWORD>  
  -P, --port <PORT>          [default: 445]
  -D, --domain <DOMAIN>      
  -S, --share <SHARE>        
      --path <PATH>          
      --tree                 
  -h, --help                 Print help
```

1. **List All Shares**:

   ```bash
   ./ad-pentest smb --target 192.168.1.1
   ```

2. **Display Tree View of a Share**:

   ```bash
   ./ad-pentest smb --target 192.168.1.1 --share myshare --tree
   ```

3. **List Directory Contents**:

   ```bash
   ./ad-pentest smb --target 192.168.1.1 --share myshare --path /documents
   ```

4. **Specify Credentials**:

   ```bash
   ./ad-pentest smb --target 192.168.1.1 --user admin --password secret --share myshare
   ```

---

## Notes

- The application requires the `tokio` runtime for asynchronous operations.
- Ensure that the target SMB server is reachable and accessible from the host running this application.


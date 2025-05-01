# SMB Shares Module

This module provides functionality for interacting with SMB (Server Message Block) shares. It includes methods for listing shares, accessing directories, and handling SMB clients with or without authentication.

## Functions

### `dir_share(client: &SmbClient, path: &str)`

Lists the contents of a specific directory in an SMB share and prints the results in a table format.

- **Parameters**:
  - `client`: An instance of `SmbClient` used to interact with the SMB server.
  - `path`: The path to the directory to list.

---

### `list_shares(args: &Cli)`
To list shares you have to declare share "" and declare path "", and it lists shares.
Lists all available shares on the SMB server. Handles both authenticated and unauthenticated access based on the provided arguments.

- **Parameters**:
  - `args`: A `Cli` struct containing user-provided arguments such as target, user, password, and share.

---

### `get_smbclient_with_login(args: &Cli, user: &String, password: &String, share: &String) -> SmbClient`

Creates an authenticated `SmbClient` instance using the provided credentials.

- **Parameters**:
  - `args`: A `Cli` struct containing the target and port information.
  - `user`: The username for authentication.
  - `password`: The password for authentication.
  - `share`: The share to connect to.

- **Returns**: An authenticated `SmbClient` instance.

---

### `get_smbclient_guest(args: &Cli) -> SmbClient`

Creates an `SmbClient` instance for guest access (unauthenticated).

- **Parameters**:
  - `args`: A `Cli` struct containing the target and port information.

- **Returns**: An unauthenticated `SmbClient` instance.

---

### `get_smbclient_null(args: &Cli) -> SmbClient`

Creates an `SmbClient` instance with null credentials (empty username and password).

- **Parameters**:
  - `args`: A `Cli` struct containing the target and port information.

- **Returns**: An `SmbClient` instance with null credentials.

---

### `print_shares_table(shares: Vec<SmbDirent>)`

Prints a table of SMB shares, including their name, type, and comment.

- **Parameters**:
  - `shares`: A vector of `SmbDirent` objects representing the shares.

---

### `list_shares_without_login(args: &Cli)`

Lists SMB shares without authentication, using null credentials or guest access.

- **Parameters**:
  - `args`: A `Cli` struct containing user-provided arguments.

---

## Dependencies

This module relies on the following crates:

- `pavao`: For interacting with SMB shares.
- `prettytable`: For displaying share information in a table format.
- `log`: For logging warnings and information.

## Usage

To use this module, ensure that the `Cli` struct from the `parser` module is properly populated with the required arguments. The module handles both authenticated and unauthenticated SMB access, making it versatile for various use cases.

Example usage:

```rust
use crate::smbshares::*;
use crate::parser::Cli;

let args = Cli::parse();
list_shares(&args);
```

# SMB Utilities Documentation

This module provides utilities for interacting with SMB (Server Message Block) shares and files. It includes functions for listing shares, reading files, writing files, and printing directory structures in a tree view format.

## Functions

### `dir_share`
```rust
pub async fn dir_share(client: Option<&SmbClient>, path: &str)
```
Lists the contents of a directory on an SMB share and prints them in a table format.

- **Parameters**:
  - `client`: An optional reference to an `SmbClient` instance.
  - `path`: The path to the directory to list.

---

### `list_shares`
```rust
pub async fn list_shares(
    user: Option<&String>,
    password: Option<&String>,
    share: Option<&String>,
    target: &String,
    port: &u16,
)
```
Lists all available shares on an SMB server.

- **Parameters**:
  - `user`: Optional username for authentication.
  - `password`: Optional password for authentication.
  - `share`: Optional share name.
  - target: Target SMB server address.
  - `port`: Port number of the SMB server.

---

### `get_smbclient_with_login`
```rust
pub async fn get_smbclient_with_login(
    target: &String,
    port: &u16,
    user: Option<&String>,
    password: Option<&String>,
    share: Option<&String>,
) -> SmbClient
```
Creates and returns an authenticated `SmbClient` instance.

- **Parameters**:
  - target: Target SMB server address.
  - `port`: Port number of the SMB server.
  - `user`: Optional username for authentication.
  - `password`: Optional password for authentication.
  - `share`: Optional share name.

- **Returns**: An `SmbClient` instance.

---

### `get_smbclient_guest`
```rust
pub async fn get_smbclient_guest(target: &String, port: &u16) -> SmbClient
```
Creates and returns an `SmbClient` instance using guest credentials.

- **Parameters**:
  - target: Target SMB server address.
  - `port`: Port number of the SMB server.

- **Returns**: An `SmbClient` instance.

---

### `list_shares_without_login`
```rust
async fn list_shares_without_login(target: &String, port: &u16)
```
Lists shares on an SMB server without authentication.

- **Parameters**:
  - target: Target SMB server address.
  - `port`: Port number of the SMB server.

---

### `print_shares_table`
```rust
async fn print_shares_table(shares: &Vec<pavao::SmbDirent>)
```
Prints a table of SMB shares.

- **Parameters**:
  - `shares`: A vector of `SmbDirent` objects representing the shares.

---

### `print_tree_view`
```rust
pub fn print_tree_view<'a>(
    client: Option<&'a SmbClient>,
    path: &'a String,
    depth: usize,
) -> Pin<Box<dyn Future<Output = ()> + 'a>>
```
Prints the directory structure of an SMB share in a tree view format.

- **Parameters**:
  - `client`: An optional reference to an `SmbClient` instance.
  - `path`: The path to the directory to print.
  - `depth`: The current depth in the directory tree.

---

### `get_file_string`
```rust
async fn get_file_string<'a>(client: &'a SmbClient, file_name: &String) -> SmbFile<'a>
```
Retrieves a file from an SMB share as an `SmbFile` object.

- **Parameters**:
  - `client`: A reference to an `SmbClient` instance.
  - `file_name`: The name of the file to retrieve.

- **Returns**: An `SmbFile` object.

---

### `read_file_func`
```rust
pub async fn read_file_func(client: &SmbClient, file_name: &String)
```
Reads the content of a file from an SMB share and prints it.

- **Parameters**:
  - `client`: A reference to an `SmbClient` instance.
  - `file_name`: The name of the file to read.

---

### `write_file_func`
```rust
pub async fn write_file_func(
    client: &SmbClient,
    remote_file_name: &String,
    local_file_name: &String,
)
```
Downloads a file from an SMB share and writes it to a local file.

- **Parameters**:
  - `client`: A reference to an `SmbClient` instance.
  - `remote_file_name`: The name of the file on the SMB share.
  - `local_file_name`: The name of the local file to write to.

---

### Error Handling
The code uses `panic!` for error handling in several places. This is not ideal for production code and should be replaced with proper error propagation using `Result` or `anyhow`.

---

### Logging
The module uses the `log` crate for logging. Ensure that a logger is initialized in your application to capture log messages.

---

### Dependencies
- `pavao`: Provides SMB client functionality.
- `prettytable`: Used for printing tables.
- `log`: Used for logging.

---

This documentation provides an overview of the module's functionality and usage. For more details, refer to the code comments and the respective library documentation.
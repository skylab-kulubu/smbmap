## Table of Contents

- Imports
- Functions
  - dir_share
  - ensure_leading_slash
  - list_shares
  - get_smbclient_with_login
  - get_smbclient_guest
  - list_shares_without_login
  - get_smbclient_null
  - print_shares_table
  - print_tree_view

---

## Imports

The file imports the following modules and crates:

- `log::warn`: For logging warnings.
- `pavao`: Provides SMB-related functionality, including `SmbClient`, `SmbCredentials`, `SmbDirent`, `SmbDirentType`, and `SmbOptions`.
- `prettytable`: Used for creating and printing tables.
- `std::future::Future` and `std::pin::Pin`: For handling asynchronous operations.

---

## Functions

### `dir_share`

```rust
pub async fn dir_share(client: Option<&SmbClient>, path: &str)
```

- **Description**: Lists the contents of a directory in an SMB share and prints them in a table format.
- **Parameters**:
  - `client`: An optional reference to an `SmbClient` instance.
  - `path`: The path to the directory to list.
- **Behavior**:
  - Ensures the path starts with a leading slash.
  - Fetches the directory contents using the `list_dir` method.
  - Prints the directory contents in a table with columns: `Name`, `Type`, and `Comment`.

---

### `ensure_leading_slash`

```rust
async fn ensure_leading_slash(path: &str) -> String
```

- **Description**: Ensures that a given path starts with a leading slash (`/`).
- **Parameters**:
  - `path`: The input path as a string slice.
- **Returns**: A `String` with a leading slash if it was missing.

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

- **Description**: Lists SMB shares on a target server, with or without authentication.
- **Parameters**:
  - `user`: Optional username for authentication.
  - `password`: Optional password for authentication.
  - `share`: Optional share name.
  - target: The target server's address.
  - `port`: The port number for the SMB service.
- **Behavior**:
  - If both `user` and `password` are provided, it authenticates and lists shares.
  - If no credentials are provided, it attempts to list shares as a guest or null user.

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

- **Description**: Creates an authenticated `SmbClient` instance.
- **Parameters**:
  - target: The target server's address.
  - `port`: The port number for the SMB service.
  - `user`: The username for authentication.
  - `password`: The password for authentication.
  - `share`: The share name to connect to.
- **Returns**: An `SmbClient` instance.

---

### `get_smbclient_guest`

```rust
pub async fn get_smbclient_guest(target: &String, port: &u16) -> SmbClient
```

- **Description**: Creates an `SmbClient` instance for guest access.
- **Parameters**:
  - target: The target server's address.
  - `port`: The port number for the SMB service.
- **Returns**: An `SmbClient` instance configured for guest access.

---

### `list_shares_without_login`

```rust
pub async fn list_shares_without_login(target: &String, port: &u16)
```

- **Description**: Lists SMB shares on a target server without authentication.
- **Parameters**:
  - target: The target server's address.
  - `port`: The port number for the SMB service.
- **Behavior**:
  - Logs a warning about using a null user.
  - Attempts to list shares using a null user.

---

### `get_smbclient_null`

```rust
pub async fn get_smbclient_null(target: &String, port: &u16) -> SmbClient
```

- **Description**: Creates an `SmbClient` instance for null user access.
- **Parameters**:
  - target: The target server's address.
  - `port`: The port number for the SMB service.
- **Returns**: An `SmbClient` instance configured for null user access.

---

### `print_shares_table`

```rust
pub async fn print_shares_table(shares: &Vec<pavao::SmbDirent>)
```

- **Description**: Prints a list of SMB shares in a table format.
- **Parameters**:
  - `shares`: A vector of `SmbDirent` objects representing the shares.
- **Behavior**:
  - Creates a table with columns: `Share`, `Type`, and `Comment`.
  - Populates the table with share details and prints it.

---

### `print_tree_view`

```rust
pub fn print_tree_view<'a>(
    client: Option<&'a SmbClient>,
    path: &'a String,
    depth: usize,
) -> Pin<Box<dyn Future<Output = ()> + 'a>>
```

- **Description**: Recursively prints the directory structure of an SMB share in a tree view format.
- **Parameters**:
  - `client`: An optional reference to an `SmbClient` instance.
  - `path`: The root path to start the tree view.
  - `depth`: The current depth of recursion (used for indentation).
- **Behavior**:
  - Lists the contents of the directory.
  - Prints each file or directory with indentation based on the depth.
  - Recursively calls itself for subdirectories.

---

## Notes

- The file uses the `pavao` library for SMB interactions, which provides abstractions for SMB clients, credentials, and directory entries.
- Error handling is minimal, with most errors logged as warnings.
- The `prettytable` crate is used extensively for formatting output in a tabular format.
- The asynchronous nature of the functions ensures non-blocking operations, suitable for network-based tasks.

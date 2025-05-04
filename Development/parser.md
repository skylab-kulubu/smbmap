This file defines the command-line interface (CLI) structure for the `AD-Pentest` tool using the `clap` crate. It specifies the main CLI arguments and subcommands that the tool supports.

## Overview

The parser.rs file uses the `clap` crate to define and parse command-line arguments. It consists of two main components:

1. **`Cli` Struct**: Represents the top-level CLI structure, including global arguments and subcommands.
2. **`Commands` Enum**: Defines the subcommands and their specific arguments.

---

## `Cli` Struct

The `Cli` struct is the entry point for parsing command-line arguments. It includes global options and a subcommand.

### Fields

- **`verbose`** (`u8`):
  - **Description**: Controls the verbosity level of the tool. The verbosity increases with each occurrence of the `-v` or `--verbose` flag.
  - **Usage**: `-v`, `--verbose`
  - **Type**: Counted flag (e.g., `-vv` for higher verbosity).

- **target** (`String`):
  - **Description**: Specifies the target for the pentest operation.
  - **Usage**: `-t <TARGET>`, `--target <TARGET>`
  - **Type**: Required argument.

- **`command`** (`Commands`):
  - **Description**: Specifies the subcommand to execute. The subcommands are defined in the `Commands` enum.

---

## `Commands` Enum

The `Commands` enum defines the subcommands supported by the tool. Currently, it includes the `Smb` subcommand.

### Subcommands

#### `Smb`

The `Smb` subcommand is used for operations related to SMB (Server Message Block) protocol.

##### Arguments

- **`user`** (`Option<String>`):
  - **Description**: Specifies the username for SMB authentication.
  - **Usage**: `-u <USER>`, `--user <USER>`
  - **Type**: Optional.

- **`password`** (`Option<String>`):
  - **Description**: Specifies the password for SMB authentication.
  - **Usage**: `-p <PASSWORD>`, `--password <PASSWORD>`
  - **Type**: Optional.

- **`port`** (`u16`):
  - **Description**: Specifies the port for the SMB connection.
  - **Usage**: `-P <PORT>`, `--port <PORT>`
  - **Default Value**: `445`

- **`domain`** (`Option<String>`):
  - **Description**: Specifies the domain for SMB authentication.
  - **Usage**: `-D <DOMAIN>`, `--domain <DOMAIN>`
  - **Type**: Optional.

- **`share`** (`Option<String>`):
  - **Description**: Specifies the SMB share to access.
  - **Usage**: `-S <SHARE>`, `--share <SHARE>`
  - **Type**: Optional.

- **`path`** (`Option<String>`):
  - **Description**: Specifies the path within the SMB share.
  - **Usage**: `--path <PATH>`
  - **Type**: Optional.

- **`tree`** (`bool`):
  - **Description**: Enables tree listing of the SMB share.
  - **Usage**: `--tree`
  - **Type**: Boolean flag (set to `true` if provided).

---

## Example Usage

Here are some example commands for using the `AD-Pentest` tool:

1. **Basic SMB Command**:

   ```bash
   ad-pentest -t 192.168.1.1 smb --user admin --password secret
   ```

2. **SMB Command with Tree Listing**:

   ```bash
   ad-pentest -t 192.168.1.1 smb --user admin --password secret --tree
   ```

3. **Verbose Output**:

   ```bash
   ad-pentest -vv -t 192.168.1.1 smb --share public
   ```

---

## Notes

- The `clap` crate is used to handle argument parsing and validation.
- The `Commands` enum can be extended to include additional subcommands in the future.

---

This documentation provides a clear understanding of the CLI structure and usage for the `AD-Pentest` tool.


farukerdem34: write documentation

GitHub Copilot: Here is the documentation for the provided code:

---

# AD-Pentest CLI Documentation

This CLI tool, **AD-Pentest**, is designed to facilitate Active Directory penetration testing. It provides a set of commands and arguments to interact with SMB services and perform various operations.

## Global Arguments

The following arguments are available globally and apply to all commands:

- **`-v, --verbose`**  
  Increases the verbosity of the output. Use multiple times to increase verbosity further.  
  *Type*: `u8` (count of occurrences)

- **`-t, --target`**  
  Specifies the target system to interact with.  
  *Type*: `String`  
  *Required*: Yes

## Commands

### `smb`

The `smb` command is used to interact with SMB services. It supports the following arguments:

- **`-u, --user`**  
  Specifies the username for authentication.  
  *Type*: `Option<String>`  
  *Required*: No

- **`-p, --password`**  
  Specifies the password for authentication.  
  *Type*: `Option<String>`  
  *Required*: No

- **`-P, --port`**  
  Specifies the port to connect to.  
  *Type*: `u16`  
  *Default*: `445`

- **`-D, --domain`**  
  Specifies the domain for authentication.  
  *Type*: `Option<String>`  
  *Required*: No

- **`-S, --share`**  
  Specifies the SMB share to interact with.  
  *Type*: `Option<String>`  
  *Required*: No

- **`--path`**  
  Specifies the path to a file or directory on the SMB share.  
  *Type*: `Option<String>`  
  *Required*: No

- **`--tree`**  
  Enables tree listing of the specified SMB share.  
  *Type*: `bool`  
  *Default*: `false`

- **`--read-file`**  
  Specifies a file to read from the SMB share. Alternatively, you can use the `--path` parameter to specify the file path.  
  *Type*: `Option<String>`  
  *Required*: No

- **`--write-file`**  
  Installs the file provided in `--path` or `--read-file` to the SMB share.  
  *Type*: `Option<String>`  
  *Required*: No

---

### Example Usage

1. **Basic SMB Command**  
   ```bash
   ad-pentest -t 192.168.1.10 smb --user admin --password secret --share public --tree
   ```

2. **Read a File**  
   ```bash
   ad-pentest -t 192.168.1.10 smb --user admin --password secret --read-file example.txt
   ```

3. **Write a File**  
   ```bash
   ad-pentest -t 192.168.1.10 smb --user admin --password secret --path /local/path/to/file --write-file
   ```

---

This documentation provides an overview of the CLI's functionality and usage. For further details, refer to the source code or additional documentation.
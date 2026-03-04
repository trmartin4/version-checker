# Bitwarden Version Compatibility Checker

A Rust tool for checking version compatibility based on Bitwarden's versioning policy.

## Bitwarden Versioning Rules

- **Major version**: The second component of the semantic version (e.g., `11` in `2024.11.0`)
- **Compatibility window**: Each version is compatible with the same major version, plus 2 major versions before and after

### Server Compatibility
Each server version is compatible with:
- Clients of the same major version
- The previous 2 major client versions
- The subsequent 2 major client versions

### Client Compatibility
Each client version is compatible with:
- Servers of the same major version
- The previous 2 major server versions
- The subsequent 2 major server versions

### Breaking Change Calculation
To determine when a breaking server change can be introduced:
1. Find the last major client version that breaks with the change
2. Find the corresponding major server version from that time
3. Add 3 to the second SemVer component
4. That's the first server version where the breaking change can be introduced

## Building & Installation

### Install locally
```bash
cargo install --path .
```

This compiles an optimized binary and installs it to `~/.cargo/bin/version-checker`.

### Build release binary
```bash
cargo build --release
```

The binary will be located at `target/release/version-checker`.

## Usage

### Command-line usage
```bash
# Check compatibility between server and client versions
version-checker --server 2024.11.0 --client 2024.10.0

# Show compatibility window for a server version
version-checker --server 2024.11.0

# Show compatibility window for a client version
version-checker --client 2024.10.0

# Show examples (no arguments)
version-checker
```

### Development usage

```bash
# Run the examples
cargo run

# Run tests
cargo test
```

## API

```rust
use std::str::FromStr;

// Parse versions
let server = Version::from_str("2024.11.0").unwrap();
let client = Version::from_str("2024.9.0").unwrap();

// Check compatibility
let compatible = is_server_compatible_with_client(&server, &client);

// Calculate when breaking changes can be introduced
let first_breaking = calculate_first_compatible_server_version(
    &last_incompatible_client,
    &corresponding_server
);
```

## Example Output

### Check compatibility between server and client

```
Bitwarden Version Compatibility Checker

Server version: 2024.11.0
Client version: 2024.10.0

Compatible: true
```

### Show compatibility window for a server version

```
Bitwarden Version Compatibility Checker

Server version: 2024.11.0

Must be compatible with client version range:
  2024.9.x through 2024.13.x
```

### Show compatibility window for a client version

```
Bitwarden Version Compatibility Checker

Client version: 2024.10.0

Must be compatible with server version range:
  2024.8.x through 2024.12.x
```

### No arguments (shows examples)

```
Bitwarden Version Compatibility Checker

No versions specified. Run with --help for usage.

Example usage:
  version-checker --server 2024.11.0 --client 2024.10.0
```

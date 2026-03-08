use clap::Parser;
use semver::Version;

/// Converts a Bitwarden version to a linear month index so that cross-year
/// boundaries are handled correctly (e.g. 2025.12 → 2026.1 is a difference of 1).
fn bitwarden_major(v: &Version) -> u64 {
    v.major * 12 + v.minor
}

/// Converts a linear month index back to (year, month).
fn from_linear(idx: u64) -> (u64, u64) {
    let year = (idx - 1) / 12;
    let month = ((idx - 1) % 12) + 1;
    (year, month)
}

fn is_server_compatible_with_client(server: &Version, client: &Version) -> bool {
    let server_major = bitwarden_major(server);
    let client_major = bitwarden_major(client);

    let diff = if server_major >= client_major {
        server_major - client_major
    } else {
        client_major - server_major
    };

    diff <= 2
}

fn is_client_compatible_with_server(client: &Version, server: &Version) -> bool {
    is_server_compatible_with_client(server, client)
}

fn calculate_first_compatible_server_version(
    _last_incompatible_client_version: &Version,
    corresponding_server_version: &Version,
) -> Version {
    let linear = bitwarden_major(corresponding_server_version) + 3;
    let (year, month) = from_linear(linear);
    Version::new(year, month, corresponding_server_version.patch)
}

#[derive(Parser, Debug)]
#[command(name = "version-checker")]
#[command(about = "Check Bitwarden version compatibility between server and client", long_about = None)]
struct Cli {
    /// Server version (e.g., 2024.11.0)
    #[arg(short, long)]
    server: Option<String>,

    /// Client version (e.g., 2024.10.0)
    #[arg(short, long)]
    client: Option<String>,
}

fn print_support_window(version: &Version, version_type: &str) {
    let major = bitwarden_major(version);
    let min_linear = major.saturating_sub(2).max(1);
    let max_linear = major + 2;
    let (min_year, min_month) = from_linear(min_linear);
    let (max_year, max_month) = from_linear(max_linear);

    let opposite_type = if version_type == "Server" {
        "client"
    } else {
        "server"
    };

    println!("{} version: {}", version_type, version);
    println!("\nMust be compatible with {} version range:", opposite_type);
    println!(
        "  {}.{}.x through {}.{}.x",
        min_year, min_month, max_year, max_month
    );
}

fn main() {
    let cli = Cli::parse();

    println!("Bitwarden Version Compatibility Checker\n");

    match (&cli.server, &cli.client) {
        (Some(server_str), Some(client_str)) => {
            let server = match server_str.parse::<Version>() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing server version: {}", e);
                    std::process::exit(1);
                }
            };

            let client = match client_str.parse::<Version>() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing client version: {}", e);
                    std::process::exit(1);
                }
            };

            println!("Server version: {}", server);
            println!("Client version: {}", client);
            println!(
                "\nCompatible: {}",
                is_server_compatible_with_client(&server, &client)
            );
        }
        (Some(server_str), None) => {
            let server = match server_str.parse::<Version>() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing server version: {}", e);
                    std::process::exit(1);
                }
            };
            print_support_window(&server, "Server");
        }
        (None, Some(client_str)) => {
            let client = match client_str.parse::<Version>() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing client version: {}", e);
                    std::process::exit(1);
                }
            };
            print_support_window(&client, "Client");
        }
        (None, None) => {
            println!("No versions specified. Run with --help for usage.\n");
            println!("Example usage:");
            println!("  version-checker --server 2024.11.0 --client 2024.10.0\n");

            println!("--- Example Compatibility Checks ---");
            let server = "2024.11.0".parse::<Version>().unwrap();
            let client1 = "2024.11.0".parse::<Version>().unwrap();
            let client2 = "2024.9.0".parse::<Version>().unwrap();
            let client3 = "2024.8.0".parse::<Version>().unwrap();
            let client4 = "2024.7.0".parse::<Version>().unwrap();

            println!("Server version: {}", server);
            println!("\nClient compatibility checks:");
            println!(
                "  {} compatible: {}",
                client1,
                is_server_compatible_with_client(&server, &client1)
            );
            println!(
                "  {} compatible: {}",
                client2,
                is_server_compatible_with_client(&server, &client2)
            );
            println!(
                "  {} compatible: {}",
                client3,
                is_server_compatible_with_client(&server, &client3)
            );
            println!(
                "  {} compatible: {} (outside 2-version window)",
                client4,
                is_server_compatible_with_client(&server, &client4)
            );

            println!("\n--- Breaking Change Calculation ---");
            let last_incompatible_client = "2024.8.0".parse::<Version>().unwrap();
            let corresponding_server = "2024.8.0".parse::<Version>().unwrap();

            let first_breaking_server = calculate_first_compatible_server_version(
                &last_incompatible_client,
                &corresponding_server,
            );

            println!(
                "Last incompatible client version: {}",
                last_incompatible_client
            );
            println!("Corresponding server version: {}", corresponding_server);
            println!(
                "First server version where breaking change can be introduced: {}",
                first_breaking_server
            );

            println!("\n--- Client Perspective ---");
            let client = "2024.10.0".parse::<Version>().unwrap();
            let server1 = "2024.10.0".parse::<Version>().unwrap();
            let server2 = "2024.12.0".parse::<Version>().unwrap();
            let server3 = "2024.13.0".parse::<Version>().unwrap();

            println!("Client version: {}", client);
            println!("\nServer compatibility checks:");
            println!(
                "  {} compatible: {}",
                server1,
                is_client_compatible_with_server(&client, &server1)
            );
            println!(
                "  {} compatible: {}",
                server2,
                is_client_compatible_with_server(&client, &server2)
            );
            println!(
                "  {} compatible: {} (outside 2-version window)",
                server3,
                is_client_compatible_with_server(&client, &server3)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let v: Version = "2024.11.0".parse().unwrap();
        assert_eq!(v.major, 2024);
        assert_eq!(v.minor, 11);
        assert_eq!(v.patch, 0);
    }

    #[test]
    fn test_bitwarden_major() {
        let v = Version::new(2024, 11, 0);
        assert_eq!(bitwarden_major(&v), 2024 * 12 + 11);
    }

    #[test]
    fn test_server_client_compatibility() {
        let server: Version = "2024.11.0".parse().unwrap();

        // Same version - compatible
        assert!(is_server_compatible_with_client(
            &server,
            &"2024.11.0".parse().unwrap()
        ));

        // Previous 2 versions - compatible
        assert!(is_server_compatible_with_client(
            &server,
            &"2024.10.0".parse().unwrap()
        ));
        assert!(is_server_compatible_with_client(
            &server,
            &"2024.9.0".parse().unwrap()
        ));

        // Next 2 versions - compatible
        assert!(is_server_compatible_with_client(
            &server,
            &"2024.12.0".parse().unwrap()
        ));
        assert!(is_server_compatible_with_client(
            &server,
            &"2025.1.0".parse().unwrap()
        ));

        // Outside window - not compatible
        assert!(!is_server_compatible_with_client(
            &server,
            &"2024.8.0".parse().unwrap()
        ));
        assert!(!is_server_compatible_with_client(
            &server,
            &"2025.2.0".parse().unwrap()
        ));
    }

    #[test]
    fn test_cross_year_compatibility() {
        let client: Version = "2025.12.0".parse().unwrap();

        // Within window across year boundary
        assert!(is_server_compatible_with_client(
            &"2026.1.0".parse().unwrap(),
            &client
        ));
        assert!(is_server_compatible_with_client(
            &"2026.2.0".parse().unwrap(),
            &client
        ));

        // Outside window
        assert!(!is_server_compatible_with_client(
            &"2026.3.0".parse().unwrap(),
            &client
        ));
    }

    #[test]
    fn test_breaking_change_calculation() {
        let last_incompatible_client: Version = "2024.8.0".parse().unwrap();
        let corresponding_server: Version = "2024.8.0".parse().unwrap();

        let result = calculate_first_compatible_server_version(
            &last_incompatible_client,
            &corresponding_server,
        );

        assert_eq!(result.major, 2024);
        assert_eq!(result.minor, 11); // 8 + 3
        assert_eq!(result.patch, 0);
    }
}

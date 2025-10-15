use serde::{Deserialize, Serialize};
use std::fs;

// Unified macro for creating config structs (handles both simple and nested cases)
macro_rules! config_struct {
    (
        $(#[$meta:meta])*
        struct $name:ident {
            // First, all regular fields
            $(
                $field_name:ident: $field_type:ty
            ),* $(,)?
            // Then, separated by semicolon, all nested config struct references
            $(;
            $(
                $nested_field_name:ident: ref $nested_type:ident
            ),* $(,)?)?
        }
    ) => {
        paste::paste! {
            // Define the base struct (without Serialize)
            $(#[$meta])*
            #[derive(Debug, Deserialize)]
            struct $name {
                $(
                    $field_name: $field_type,
                )*
                $($(
                    $nested_field_name: $nested_type,
                )*)?
            }

            // Define the serializable version with correct nested types
            $(#[$meta])*
            #[derive(Debug, Serialize, Deserialize)]
            struct [<$name Serializable>] {
                $(
                    $field_name: $field_type,
                )*
                $($(
                    $nested_field_name: [<$nested_type Serializable>],
                )*)?
            }
        }
    };
}

// Define our configuration structs using the unified macro
config_struct! {
    struct DatabaseConfig {
        host: String,
        port: u16,
        username: String,
        password: String,
        database_name: String
    }
}

config_struct! {
    struct ServerConfig {
        listen_address: String,
        listen_port: u16,
        max_connections: usize,
        timeout_seconds: u64
    }
}

config_struct! {
    struct LogConfig {
        level: String,
        output_file: String
    }
}

// Now AppConfig can be defined using the macro with nested references
config_struct! {
    struct AppConfig {
        app_name: String,
        version: String,
        debug_mode: bool,
        features: Vec<String>;
        database: ref DatabaseConfig,
        server: ref ServerConfig
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Configuration File Parsing Demo with Serde ===\n");

    // Read the configuration file
    let config_content = fs::read_to_string("config.json")?;
    println!("Raw JSON content:");
    println!("{}\n", config_content);

    // Parse the JSON into our AppConfig struct (non-serializable version)
    let config: AppConfig = serde_json::from_str(&config_content)?;

    println!("Parsed configuration (non-serializable version):");
    println!("{:#?}\n", config);

    // Demonstrate accessing individual fields
    println!("=== Accessing Configuration Values ===");
    println!("App Name: {}", config.app_name);
    println!("Version: {}", config.version);
    println!("Debug Mode: {}", config.debug_mode);
    println!("Database Host: {}", config.database.host);
    // println!("Database Port: {}", config.database.port);
    println!("Database Username: {}", config.database.username);
    println!("Database Password: {}", config.database.password);
    println!("Database Name: {}", config.database.database_name);
    println!("Server Listen Address: {}", config.server.listen_address);
    println!("Server Listen Port: {}", config.server.listen_port);
    println!("Max Connections: {}", config.server.max_connections);
    println!("Timeout Seconds: {}", config.server.timeout_seconds);
    println!("Features: {:?}", config.features);

    // Parse into serializable version for demonstration
    println!("\n=== Using Serializable Version ===");
    let config_serializable: AppConfigSerializable = serde_json::from_str(&config_content)?;
    println!("Parsed configuration (serializable version):");
    println!("{:#?}\n", config_serializable);

    // Demonstrate serialization back to JSON (this works with the Serializable version)
    println!("=== Serializing Back to JSON ===");
    let json_output = serde_json::to_string_pretty(&config_serializable)?;
    println!("{}", json_output);

    // Demonstrate LogConfig - both versions
    println!("\n=== LogConfig Examples ===");
    let log_config_json = r#"{"level": "info", "output_file": "/var/log/app.log"}"#;

    let log_config: LogConfig = serde_json::from_str(log_config_json)?;
    println!("Log Config (non-serializable): {:?}", log_config);
    println!(
        "Log Level: {}, Output File: {}",
        log_config.level, log_config.output_file
    );

    let log_config_serializable: LogConfigSerializable = serde_json::from_str(log_config_json)?;
    println!("Log Config (serializable): {:?}", log_config_serializable);

    // This works with the serializable version:
    let log_json = serde_json::to_string(&log_config_serializable)?;
    println!("Log Config as JSON: {}", log_json);

    println!("\n=== Macro Usage Summary ===");
    println!("✓ Each struct definition creates TWO versions:");
    println!("  - AppConfig (non-serializable): Debug + Deserialize only");
    println!("  - AppConfigSerializable: Debug + Serialize + Deserialize");
    println!("✓ Use non-serializable for memory efficiency when you only need to read config");
    println!("✓ Use serializable when you need to write config back to JSON/files");

    Ok(())
}

use serde::{Deserialize};
use std::fs;

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database_name: String,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    listen_address: String,
    listen_port: u16,
    max_connections: usize,
    timeout_seconds: u64,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    app_name: String,
    version: String,
    unused: String,
    debug_mode: bool,
    database: DatabaseConfig,
    server: ServerConfig,
    features: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Configuration File Parsing Demo with Serde ===\n");

    // Read the configuration file
    let config_content = fs::read_to_string("config.json")?;
    println!("Raw JSON content:");
    println!("{}\n", config_content);

    // Parse the JSON into our AppConfig struct
    let config: AppConfig = serde_json::from_str(&config_content)?;

    println!("Parsed configuration:");
    println!("{:#?}\n", config);

    // Demonstrate accessing individual fields
    println!("=== Accessing Configuration Values ===");
    println!("App Name: {}", config.app_name);
    println!("Version: {}", config.version);
    println!("Debug Mode: {}", config.debug_mode);
    println!("Database Host: {}", config.database.host);
    println!("Database Port: {}", config.database.port);
    println!("Server Listen Address: {}", config.server.listen_address);
    println!("Max Connections: {}", config.server.max_connections);
    println!("Features: {:?}", config.features);

    Ok(())
}

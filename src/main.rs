use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct AppConfig {
    app_name: String,
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
    // println!("App Name: {}", config.app_name);

    Ok(())
}

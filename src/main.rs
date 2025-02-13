use std::path::Path;
use std::process::{self, Command};
use workspace_aggregator::cli::help::{print_help, print_short_help, print_version};
use workspace_aggregator::version;
use workspace_aggregator::{Config, FileProcessor};

fn print_update_status(new_version: &str) {
    println!("✅ Updated to version {}", new_version);
    println!("🔍 Changes in this version:");
    if let Ok(output) = Command::new("git")
        .args(["log", "--oneline", "-1"])
        .output()
    {
        if let Ok(msg) = String::from_utf8(output.stdout) {
            println!("{}", msg);
        }
    }
}

fn update_self() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Updating workspace-aggregator...");

    // Get current directory
    let _current_dir = std::env::current_dir()?;

    // Check if we're in the project directory
    if !Path::new("Cargo.toml").exists() {
        return Err("Not in the project directory. Please run from the project root.".into());
    }

    // Store current version
    let old_version = version::get_version();

    // Run git pull to get latest changes
    println!("📥 Fetching latest changes...");
    let pull_output = Command::new("git")
        .args(["pull", "origin", "main"])
        .output()?;

    if !pull_output.status.success() {
        return Err("Failed to pull latest changes".into());
    }

    // Install latest version
    println!("⚙️ Installing latest version...");
    let install_output = Command::new("cargo")
        .args(["install", "--path", ".", "--force"])
        .output()?;

    if !install_output.status.success() {
        return Err("Installation failed".into());
    }

    // Get new version
    let new_version = version::get_version();

    if old_version != new_version {
        print_update_status(&new_version);
    } else {
        println!("✅ Already at latest version {}", new_version);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Handle command-line arguments before config parsing
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--version" | "-v" => {
                print_version();
                process::exit(0);
            }
            "--update" | "-u" => match update_self() {
                Ok(_) => {
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("❌ Update failed: {}", e);
                    process::exit(1);
                }
            },
            _ => {}
        }
    }

    // Parse configuration
    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            // Check if this is a help or version display
            if e.kind() == std::io::ErrorKind::Other {
                process::exit(0);
            }
            // Otherwise it's an error
            eprintln!("❌ Error parsing configuration: {}", e);
            print_short_help();
            process::exit(1);
        }
    };

    // Create and run processor
    let mut processor = FileProcessor::new(config);

    if let Err(err) = processor.process() {
        eprintln!("❌ Error during processing: {}", err);
        process::exit(1);
    }

    // Successful completion
    println!("✨ Processing completed successfully!");
}

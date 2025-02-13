use std::fs;
use std::process::Command;
use std::env;
use vergen::EmitBuilder;


fn main() {
    let description = "Workspace Aggregator project";  // Replace with your description
    fs::write(".git/description", description)?;
    // Generate build info
    EmitBuilder::builder()
        .build_timestamp()
        .cargo_profile()
        .emit()
        .unwrap_or_else(|e| eprintln!("Failed to generate build info: {}", e));

    // Only increment version on release builds
    if env::var("PROFILE").unwrap() == "release" {
        // Check if compilation is triggered by `cargo build` or `cargo run`
        if let Ok(cmd) = env::var("CARGO") {
            if cmd.contains("cargo") {
                increment_version().unwrap_or_else(|e| {
                    eprintln!("Failed to increment version: {}", e);
                });
            }
        }
    }
}

fn increment_version() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let mut doc = cargo_toml.parse::<toml_edit::Document>()?;

    let version_str = doc["package"]["version"]
        .as_str()
        .ok_or("Version not found")?;

    let mut version_parts: Vec<u32> = version_str
        .split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    match increment_type {
        "major" => {
            version_parts[0] += 1;
            version_parts[1] = 0;
            version_parts[2] = 0;
        },
        "minor" => {
            version_parts[1] += 1;
            version_parts[2] = 0;
        },
        "patch" => {
            version_parts[2] += 1;
        },
        _ => return Err("Invalid increment type".into()),
    }

    // Update version in document
    doc["package"]["version"] = toml_edit::value(new_version.clone());

    // Write back to Cargo.toml
    fs::write("Cargo.toml", doc.to_string())?;

    // Optional: Create git commit for version bump
    if is_git_repo()? {
        Command::new("git")
            .args(&["add", "Cargo.toml"])
            .output()?;

        Command::new("git")
            .args(&["commit", "-m", &format!("chore: bump version to {}", new_version)])
            .output()?;
    }

    println!("cargo:warning=Version bumped to {}", new_version);
    Ok(())
}

fn is_git_repo() -> Result<bool, std::io::Error> {
    Ok(Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .output()?
        .status
        .success())
}

fn update_changelog(new_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut changelog = String::from(format!("## [{}] - {}\n",
        new_version,
        Local::now().format("%Y-%m-%d")
    ));

    // Get git commits since last tag
    let output = Command::new("git")
        .args(&["log", "--pretty=format:- %s", "HEAD...HEAD^"])
        .output()?;

    changelog.push_str(&String::from_utf8_lossy(&output.stdout));
    changelog.push_str("\n\n");

    // Prepend to CHANGELOG.md
    let existing = fs::read_to_string("CHANGELOG.md").unwrap_or_default();
    fs::write("CHANGELOG.md", format!("{}{}", changelog, existing))?;

    Ok(())
}


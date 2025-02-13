use crate::version;
use colored::*;

pub fn print_help() {
    let help_text = format!(
        r#"
{} v{}
{}

A powerful tool for analyzing and documenting directory contents

{}
{}
$ git clone https://github.com/gbaufake/workspace-aggregator
$ cd workspace-aggregator
$ cargo install --path .

{}
{}
$ workspace-aggregator .

$ workspace-aggregator . --generate workspace,tree,summary

$ workspace-aggregator . --exclude md

{}
📊 Analysis Tools
  • Language detection and statistics
  • Code complexity metrics
  • Directory structure visualization

📝 Documentation
  • Aggregated workspace content
  • Directory tree generation
  • Project statistics and summaries
  • JSON metadata export
  • LLM-optimized format

🔍 Smart Filtering
  • Extension-based filtering
  • Directory exclusions
  • Pattern matching
  • .gitignore integration

{}
{}
$ workspace-aggregator . \
    --generate workspace,files,tree,summary,meta \
    --output-dir ./docs

$ workspace-aggregator ./src \
    --exclude js,css \
    --exclude-dir test,vendor \
    --respect-gitignore \
    --verbose

$ workspace-aggregator . \
    --generate llm \
    --output-dir ./analysis

{}
{}
workspace    Project content aggregation     workspace_YYYYMMDD_HHMMSS.txt
files       List of processed files         files_YYYYMMDD_HHMMSS.txt
tree        Directory structure             tree_YYYYMMDD_HHMMSS.txt
summary     Project overview                summary_YYYYMMDD_HHMMSS.txt
meta        JSON metadata                   meta_YYYYMMDD_HHMMSS.json
llm         LLM-optimized format           llm_YYYYMMDD_HHMMSS.md

{}
{}
# Output Control
$ workspace-aggregator . --output-dir ./docs
$ workspace-aggregator . --generate workspace,tree

# Filtering
$ workspace-aggregator . --exclude md,txt
$ workspace-aggregator . --exclude-dir test,temp
$ workspace-aggregator . --respect-gitignore

# Display Options
$ workspace-aggregator . --verbose
$ workspace-aggregator . --quiet

{}
{}
project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── integration_tests.rs
└── Cargo.toml

{}

Made with ❤️  in Rust by Guilherme Baufaker Rêgo
Email: baufaker@protonmail.com
"#,
        "Workspace Aggregator".bright_green().bold(),
        version::get_version(), // Use version module here
        "=========================".bright_green(),
        "Installation 🚀".yellow().bold(),
        "$ # Install from source".bright_black(),
        "Basic Usage 📝".yellow().bold(),
        "$ # Basic examples".bright_black(),
        "Features 🌟".yellow().bold(),
        "Advanced Examples 🔧".yellow().bold(),
        "$ # Advanced usage".bright_black(),
        "Output Types 📋".yellow().bold(),
        "Type        Description                    Default Filename".bright_black(),
        "Command Options 🎯".yellow().bold(),
        "$ # Available commands".bright_black(),
        "Example Output 💡".yellow().bold(),
        "Directory Tree Example:".bright_black(),
        "==========".bright_green()
    );

    println!("{}", help_text);
}

pub fn print_version() {
    println!("\n{}", "=".repeat(50).bright_green());
    println!("{}", "📦 workspace-aggregator".bright_green().bold());
    println!("🔖 Version: {}", version::get_version().bright_yellow());
    println!("🦀 Built with Rust 2021 Edition");
    println!("👤 Author: Guilherme Baufaker Rêgo");
    println!("📧 Contact: baufaker@protonmail.com");
    println!("{}\n", "=".repeat(50).bright_green());
}

pub fn print_short_help() {
    println!("\n{}", "❌ Invalid usage!".red().bold());
    println!("\n{}", "📋 Basic usage:".yellow().bold());
    println!("  workspace-aggregator <directory> [options]");
    println!("\n{}:", "Common options".yellow().bold());
    println!(
        "  --generate <types>    Specify outputs (workspace,files,tree,stats,summary,meta,llm)"
    );
    println!("  --output-dir <path>   Set output directory");
    println!("  --exclude <exts>      Exclude file extensions");
    println!("  --verbose            Enable detailed output");
    println!("\n❓ For more information:");
    println!("  workspace-aggregator --help\n");
}

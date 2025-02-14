use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use workspace_aggregator::{CodeIndex, DependencyAnalyzer, MetricsAnalyzer};

#[derive(StructOpt)]
#[structopt(name = "workspace-query", about = "Query workspace analysis results")]
enum Command {
    /// Find symbol references
    Symbol {
        /// Symbol name to search for
        name: String,
        /// Analysis directory
        #[structopt(long, default_value = "docs")]
        analysis_dir: PathBuf,
    },
    /// Get metrics for a file
    Metrics {
        /// File path to analyze
        file: PathBuf,
        /// Analysis directory
        #[structopt(long, default_value = "docs")]
        analysis_dir: PathBuf,
    },
    /// Show dependencies
    Dependencies {
        /// File to show dependencies for
        file: PathBuf,
        /// Analysis directory
        #[structopt(long, default_value = "docs")]
        analysis_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cmd = Command::from_args();

    match cmd {
        Command::Symbol { name, analysis_dir } => {
            let index = CodeIndex::new(&analysis_dir.join("indexes"))?;
            if let Some(references) = index.find_symbol(&name)? {
                println!("References to '{}':", name);
                for reference in references {
                    println!("- {}:{}", reference.file.display(), reference.line);
                }
            } else {
                println!("No references found for '{}'", name);
            }
        }

        Command::Metrics { file, analysis_dir } => {
            let metrics = MetricsAnalyzer::load_file_metrics(
                &analysis_dir.join("metrics/code_quality.json"),
                &file,
            )?;

            println!("Metrics for {}:", file.display());
            println!("Total Lines: {}", metrics.total_lines);
            println!("Code Lines: {}", metrics.code_lines);
            println!("Comment Lines: {}", metrics.comment_lines);
            println!("Blank Lines: {}", metrics.blank_lines);
        }

        Command::Dependencies { file, analysis_dir } => {
            let deps = DependencyAnalyzer::load_dependencies(
                &analysis_dir.join("dependencies/internal_deps.json"),
            )?;

            println!("Dependencies for {}:", file.display());
            println!("\nRequired by:");
            for dep in deps.get_dependents(&file) {
                println!("- {}", dep);
            }

            println!("\nDepends on:");
            for dep in deps.get_dependencies(&file) {
                println!("- {}", dep);
            }
        }
    }

    Ok(())
}

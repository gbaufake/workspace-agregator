use chrono::Local;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, Write};

use crate::config::Config;
use crate::processor::types::EnhancedFileStats;

pub struct MetaGenerator;

impl Default for MetaGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_metadata(
        &self,
        writer: &mut impl Write,
        stats: &EnhancedFileStats,
        config: &Config,
    ) -> io::Result<()> {
        // Calculate total lines
        let total_lines: usize = stats.language_stats.values().map(|s| s.lines).sum();

        // Get total size
        let total_size: u64 = stats.file_statistics.values().map(|s| s.size).sum();

        // Calculate complexity metrics
        let complexities: Vec<f64> = stats
            .file_statistics
            .values()
            .map(|s| s.complexity.cyclomatic_complexity)
            .collect();

        let (avg, max, min, std_dev) = if !complexities.is_empty() {
            let avg = complexities.iter().sum::<f64>() / complexities.len() as f64;
            let max = complexities
                .iter()
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let min = complexities.iter().fold(f64::INFINITY, |a, &b| a.min(b));

            // Calculate standard deviation
            let variance = complexities
                .iter()
                .map(|x| {
                    let diff = x - avg;
                    diff * diff
                })
                .sum::<f64>()
                / complexities.len() as f64;
            let std_dev = variance.sqrt();

            (avg, max, min, std_dev)
        } else {
            (0.0, 0.0, 0.0, 0.0)
        };

        // Get largest files
        let mut largest_files: Vec<_> = stats
            .file_statistics
            .iter()
            .map(|(path, stat)| {
                json!({
                    "path": path.to_string_lossy(),
                    "size_bytes": stat.size,
                })
            })
            .collect();
        largest_files.sort_by(|a, b| {
            b.get("size_bytes")
                .unwrap()
                .as_u64()
                .cmp(&a.get("size_bytes").unwrap().as_u64())
        });
        largest_files.truncate(10);

        let metadata = json!({
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": Local::now().to_rfc3339(),
            "project": {
                "path": config.dir_path.to_string_lossy(),
                "files": {
                    "total": stats.file_statistics.len(),
                    "size_bytes": total_size,
                    "lines": total_lines,
                }
            },
            "languages": stats.language_stats.iter().map(|(lang, stats)| {
                (lang.clone(), json!({
                    "files": stats.files,
                    "lines": stats.lines,
                    "code_lines": stats.code_lines,
                    "comment_lines": stats.comment_lines,
                    "blank_lines": stats.blank_lines,
                    "complexity": {
                        "cyclomatic": stats.complexity.cyclomatic_complexity,
                        "comment_ratio": stats.complexity.comment_ratio,
                    }
                }))
            }).collect::<HashMap<String, Value>>(),
            "complexity_metrics": {
                "average": avg,
                "maximum": max,
                "minimum": min,
                "standard_deviation": std_dev,
            },
            "file_sizes": {
                "largest": largest_files,
            },
            "configuration": {
                "exclude_extensions": config.exclude_extensions,
                "exclude_directories": config.exclude_directories,
                "exclude_patterns": config.exclude_patterns,
                "respect_gitignore": config.respect_gitignore,
                "generated_types": config.generated_types.iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<String>>(),
            }
        });

        serde_json::to_writer_pretty(writer, &metadata)?;
        Ok(())
    }
}

use std::path::Path;

use crate::processor::types::{EnhancedFileStats, FileStatistics};

pub struct StatsAnalyzer {
    stats: EnhancedFileStats,
}

impl Default for StatsAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsAnalyzer {
    pub fn new() -> Self {
        Self {
            stats: EnhancedFileStats::default(),
        }
    }

    pub fn update_stats(&mut self, path: &Path, file_stats: FileStatistics) {
        // Update extension counts
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                *self
                    .stats
                    .extension_counts
                    .entry(ext_str.to_lowercase())
                    .or_insert(0) += 1;
            }
        }

        // Update size statistics
        self.stats.total_size += file_stats.size;
        self.stats.total_lines += file_stats.lines;

        // Update largest files
        self.stats
            .largest_files
            .push((path.to_path_buf(), file_stats.size));
        self.stats.largest_files.sort_by(|a, b| b.1.cmp(&a.1));
        self.stats.largest_files.truncate(20);

        // Store file statistics
        self.stats
            .file_statistics
            .insert(path.to_path_buf(), file_stats);
    }

    pub fn calculate_metrics(&mut self) {
        let complexities: Vec<f64> = self
            .stats
            .file_statistics
            .values()
            .map(|stats| stats.complexity.cyclomatic_complexity)
            .collect();

        if !complexities.is_empty() {
            // Calculate average
            self.stats.complexity_metrics.average =
                complexities.iter().sum::<f64>() / complexities.len() as f64;

            // Calculate min/max
            self.stats.complexity_metrics.maximum = complexities
                .iter()
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            self.stats.complexity_metrics.minimum =
                complexities.iter().fold(f64::INFINITY, |a, &b| a.min(b));

            // Calculate standard deviation
            let variance = complexities
                .iter()
                .map(|x| {
                    let diff = x - self.stats.complexity_metrics.average;
                    diff * diff
                })
                .sum::<f64>()
                / complexities.len() as f64;
            self.stats.complexity_metrics.standard_deviation = variance.sqrt();
        }
    }

    pub fn get_stats(&self) -> &EnhancedFileStats {
        &self.stats
    }
}

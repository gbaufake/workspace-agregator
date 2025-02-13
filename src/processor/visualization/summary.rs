use chrono::Local;
use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use terminal_size::{terminal_size, Width};

use crate::processor::types::EnhancedFileStats;

#[derive(Default)]
struct ProjectMetrics {
    total_files: usize,
    total_size: u64,
    total_lines: usize,
    avg_file_size: f64,
    avg_lines_per_file: f64,
    max_file_size: u64,
    max_file_lines: usize,
    code_to_comment_ratio: f64,
    complexity_distribution: HashMap<String, usize>, // Complexity buckets
    language_distribution: HashMap<String, usize>,   // Language stats
}

pub struct SummaryGenerator {
    width: usize,
    use_color: bool,
}

impl Default for SummaryGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SummaryGenerator {
    pub fn new() -> Self {
        let width = terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(80);

        Self {
            width,
            use_color: true,
        }
    }

    pub fn generate_summary(
        &self,
        writer: &mut impl Write,
        stats: &EnhancedFileStats,
        base_path: &str,
    ) -> io::Result<()> {
        let metrics = self.calculate_project_metrics(stats);

        self.write_header(writer)?;
        self.write_project_info(writer, base_path)?;
        self.write_key_metrics(writer, stats, &metrics)?;
        self.write_language_breakdown(writer, stats, &metrics)?;
        self.write_complexity_analysis(writer, stats, &metrics)?;
        self.write_file_insights(writer, stats, &metrics)?;
        self.write_recommendations(writer, stats, &metrics)?;
        self.write_footer(writer)?;
        Ok(())
    }

    fn calculate_project_metrics(&self, stats: &EnhancedFileStats) -> ProjectMetrics {
        let mut metrics = ProjectMetrics::default();

        // Basic metrics
        metrics.total_files = stats.file_statistics.len();
        metrics.total_size = stats.total_size;
        metrics.total_lines = stats.total_lines;

        // Calculate averages
        if metrics.total_files > 0 {
            metrics.avg_file_size = stats.total_size as f64 / metrics.total_files as f64;
            metrics.avg_lines_per_file = stats.total_lines as f64 / metrics.total_files as f64;
        }

        // Track largest file and most lines
        for file_stat in stats.file_statistics.values() {
            metrics.max_file_size = metrics.max_file_size.max(file_stat.size);
            metrics.max_file_lines = metrics.max_file_lines.max(file_stat.lines);

            // Calculate code to comment ratio
            if file_stat.comments > 0 {
                metrics.code_to_comment_ratio = file_stat.code as f64 / file_stat.comments as f64;
            }
        }

        // Language distribution
        for (lang, stats) in &stats.language_stats {
            metrics
                .language_distribution
                .insert(lang.clone(), stats.files);
        }

        metrics
    }

    fn write_header(&self, writer: &mut impl Write) -> io::Result<()> {
        let separator = "=".repeat(self.width);
        writeln!(writer, "\n{}", separator.blue())?;
        writeln!(
            writer,
            "{:^width$}",
            "Project Analysis Summary".bold(),
            width = self.width
        )?;
        writeln!(
            writer,
            "{:^width$}",
            format!("Generated: {}", Local::now().format("%Y-%m-%d %H:%M:%S")).cyan(),
            width = self.width
        )?;
        writeln!(writer, "{}\n", separator.blue())?;
        Ok(())
    }

    fn write_project_info(&self, writer: &mut impl Write, base_path: &str) -> io::Result<()> {
        writeln!(writer, "{}", "📁 Project Location".bold())?;
        writeln!(writer, "{}", "-".repeat(40))?;
        writeln!(writer, "Base Path: {}", base_path)?;
        writeln!(writer)?;
        Ok(())
    }

    fn write_key_metrics(
        &self,
        writer: &mut impl Write,
        stats: &EnhancedFileStats,
        metrics: &ProjectMetrics,
    ) -> io::Result<()> {
        writeln!(writer, "{}", "📊 Key Metrics".bold())?;
        writeln!(writer, "{}", "-".repeat(40))?;

        // File statistics
        writeln!(writer, "{}", "File Statistics:".yellow())?;
        writeln!(writer, "  Total Files:        {:>8}", metrics.total_files)?;
        writeln!(
            writer,
            "  Total Size:         {:>8.2} MB",
            metrics.total_size as f64 / (1024.0 * 1024.0)
        )?;
        writeln!(
            writer,
            "  Average File Size:  {:>8.2} KB",
            metrics.avg_file_size / 1024.0
        )?;
        writeln!(
            writer,
            "  Largest File:       {:>8.2} MB",
            metrics.max_file_size as f64 / (1024.0 * 1024.0)
        )?;
        writeln!(writer)?;

        // Code statistics
        let total_lines: usize = stats.language_stats.values().map(|s| s.lines).sum();
        let total_comments: usize = stats.language_stats.values().map(|s| s.comment_lines).sum();

        writeln!(writer, "{}", "Code Statistics:".yellow())?;
        writeln!(writer, "  Total Lines:        {:>8}", total_lines)?;
        writeln!(
            writer,
            "  Average Lines:      {:>8.1}",
            metrics.avg_lines_per_file
        )?;
        writeln!(
            writer,
            "  Code/Comment Ratio: {:>8.1}",
            if total_comments > 0 {
                (total_lines - total_comments) as f64 / total_comments as f64
            } else {
                0.0
            }
        )?;
        writeln!(
            writer,
            "  Languages:          {:>8}",
            stats.language_stats.len()
        )?;
        writeln!(writer)?;

        Ok(())
    }

    fn write_language_breakdown(
        &self,
        writer: &mut impl Write,
        stats: &EnhancedFileStats,
        _metrics: &ProjectMetrics,
    ) -> io::Result<()> {
        writeln!(writer, "{}", "🗣️ Language Distribution".bold())?;
        writeln!(writer, "{}", "-".repeat(40))?;

        let total_lines: usize = stats.language_stats.values().map(|s| s.lines).sum();

        let mut languages: Vec<_> = stats.language_stats.iter().collect();
        languages.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));

        for (lang, stats) in &languages {
            let percentage = if total_lines > 0 {
                (stats.lines as f64 / total_lines as f64) * 100.0
            } else {
                0.0
            };

            let bar = self.generate_bar(percentage, 30);
            writeln!(
                writer,
                "{:>15}: {:>6.1}% {} ({} files)",
                lang, percentage, bar, stats.files
            )?;
        }
        writeln!(writer)?;

        Ok(())
    }

    fn write_complexity_analysis(
        &self,
        writer: &mut impl Write,
        stats: &EnhancedFileStats,
        metrics: &ProjectMetrics,
    ) -> io::Result<()> {
        writeln!(writer, "{}", "🎯 Complexity Analysis".bold())?;
        writeln!(writer, "{}", "-".repeat(40))?;

        // Complexity distribution
        writeln!(writer, "{}", "Complexity Distribution:".yellow())?;
        for (complexity, count) in &metrics.complexity_distribution {
            let percentage = (*count as f64 / metrics.total_files as f64) * 100.0;
            let bar = self.generate_bar(percentage, 30);
            writeln!(
                writer,
                "  {:<15} {:>3}% {}",
                complexity, percentage as u32, bar
            )?;
        }
        writeln!(writer)?;

        // Most complex files
        writeln!(writer, "{}", "Most Complex Files:".yellow())?;
        let mut complex_files: Vec<_> = stats.file_statistics.iter().collect();
        complex_files.sort_by(|a, b| {
            b.1.complexity
                .cyclomatic_complexity
                .partial_cmp(&a.1.complexity.cyclomatic_complexity)
                .unwrap()
        });

        for (path, stats) in complex_files.iter().take(5) {
            writeln!(
                writer,
                "  {:.1} - {}",
                stats.complexity.cyclomatic_complexity,
                path.display()
            )?;
        }
        writeln!(writer)?;

        Ok(())
    }

    fn write_file_insights(
        &self,
        writer: &mut impl Write,
        stats: &EnhancedFileStats,
        _metrics: &ProjectMetrics,
    ) -> io::Result<()> {
        writeln!(writer, "{}", "💡 File Insights".bold())?;
        writeln!(writer, "{}", "-".repeat(40))?;

        // Largest files
        writeln!(writer, "{}", "Largest Files:".yellow())?;
        let mut largest: Vec<_> = stats.file_statistics.iter().collect();
        largest.sort_by(|a, b| b.1.size.cmp(&a.1.size));

        for (path, stats) in largest.iter().take(5) {
            writeln!(
                writer,
                "  {:>8.2} MB - {}",
                stats.size as f64 / (1024.0 * 1024.0),
                path.display()
            )?;
        }
        writeln!(writer)?;

        // Time-based analysis
        writeln!(writer, "{}", "Recent Changes:".yellow())?;
        let mut recent_files: Vec<_> = stats.file_statistics.iter().collect();
        recent_files.sort_by_key(|(_, stats)| std::cmp::Reverse(stats.last_modified));

        for (path, stats) in recent_files.iter().take(5) {
            writeln!(
                writer,
                "  {} - {}",
                stats.last_modified.format("%Y-%m-%d %H:%M:%S"),
                path.display()
            )?;
        }
        writeln!(writer)?;

        // File extensions
        let mut extension_stats: HashMap<String, (usize, u64)> = HashMap::new();
        for (path, size) in &stats.largest_files {
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_string();
                let entry = extension_stats.entry(ext).or_insert((0, 0));
                entry.0 += 1; // Count
                entry.1 += size; // Total size
            }
        }

        if !extension_stats.is_empty() {
            writeln!(writer, "{}", "File Types:".yellow())?;
            let mut stats_vec: Vec<_> = extension_stats.iter().collect();
            stats_vec.sort_by(|a, b| b.1 .1.cmp(&a.1 .1)); // Sort by total size

            for (ext, (count, total_size)) in stats_vec {
                writeln!(
                    writer,
                    "  .{:<8} {:>4} files {:>8.2} MB",
                    ext,
                    count,
                    *total_size as f64 / (1024.0 * 1024.0)
                )?;
            }
            writeln!(writer)?;
        }

        // Directory statistics
        let mut dir_stats: HashMap<PathBuf, (usize, u64)> = HashMap::new();
        for (path, size) in &stats.largest_files {
            if let Some(parent) = path.parent() {
                let entry = dir_stats.entry(parent.to_path_buf()).or_insert((0, 0));
                entry.0 += 1; // Count
                entry.1 += size; // Total size
            }
        }

        if !dir_stats.is_empty() {
            writeln!(writer, "{}", "Directory Distribution:".yellow())?;
            let mut stats_vec: Vec<_> = dir_stats.iter().collect();
            stats_vec.sort_by(|a, b| b.1 .1.cmp(&a.1 .1)); // Sort by total size

            for (dir, (count, total_size)) in stats_vec.iter().take(5) {
                writeln!(
                    writer,
                    "  {:>4} files {:>8.2} MB: {}",
                    count,
                    *total_size as f64 / (1024.0 * 1024.0),
                    dir.display()
                )?;
            }
            writeln!(writer)?;
        }

        Ok(())
    }

    fn write_recommendations(
        &self,
        writer: &mut impl Write,
        stats: &EnhancedFileStats,
        _metrics: &ProjectMetrics,
    ) -> io::Result<()> {
        writeln!(writer, "{}", "💡 Recommendations".bold())?;
        writeln!(writer, "{}", "-".repeat(40))?;

        let mut recommendations = Vec::new();

        // Complexity recommendations
        let complex_files = stats
            .file_statistics
            .iter()
            .filter(|(_, s)| s.complexity.cyclomatic_complexity > 20.0)
            .count();

        if complex_files > 0 {
            recommendations.push(format!(
                "• Consider refactoring {} files with high complexity",
                complex_files
            ));
        }

        // Documentation recommendations
        let poorly_documented = stats
            .file_statistics
            .iter()
            .filter(|(_, s)| (s.comments as f64) / (s.lines as f64) < 0.1)
            .count();

        if poorly_documented > 0 {
            recommendations.push(format!(
                "• Add documentation to {} files with low comment coverage",
                poorly_documented
            ));
        }

        // File size recommendations
        let large_files = stats
            .file_statistics
            .iter()
            .filter(|(_, s)| s.size > 100 * 1024) // Files larger than 100KB
            .count();

        if large_files > 0 {
            recommendations.push(format!(
                "• Consider splitting {} large files (>100KB)",
                large_files
            ));
        }

        if recommendations.is_empty() {
            writeln!(writer, "✅ No immediate improvements needed")?;
        } else {
            for rec in recommendations {
                writeln!(writer, "{}", rec)?;
            }
        }

        writeln!(writer)?;
        Ok(())
    }

    fn write_footer(&self, writer: &mut impl Write) -> io::Result<()> {
        writeln!(writer, "{}", "=".repeat(self.width).blue())
    }

    fn generate_bar(&self, percentage: f64, max_width: usize) -> String {
        let width = ((percentage / 100.0) * max_width as f64) as usize;
        let bar = "█".repeat(width);

        if self.use_color {
            match percentage as u32 {
                0..=33 => bar.green(),
                34..=66 => bar.yellow(),
                _ => bar.red(),
            }
            .to_string()
        } else {
            bar
        }
    }
}

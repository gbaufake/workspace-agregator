use chrono::{DateTime, Local};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::config::{Config, OutputType, VerbosityLevel};
use crate::filters::gitignore::GitignoreFilter;
use crate::filters::patterns::{should_ignore, should_process_file};
use crate::output::TreeOutput;
use crate::processor::analysis::complexity::ComplexityAnalyzer;
use crate::processor::analysis::language::LanguageDetector;
use crate::processor::analysis::stats::StatsAnalyzer;
use crate::processor::types::*;
use crate::processor::visualization::charts::ChartGenerator;
use crate::processor::visualization::llm::LLMGenerator;
use crate::processor::visualization::meta::MetaGenerator;
use crate::processor::visualization::summary::SummaryGenerator;

pub struct FileProcessor {
    // Processing state
    total_files: usize,
    processed_files: usize,
    total_size: u64,
    start_time: Instant,

    // Progress tracking
    progress_bar: ProgressBar,

    // Configuration
    config: Config,
    exclude_extensions: HashSet<String>,
    exclude_directories: HashSet<String>,
    exclude_patterns: HashSet<String>,
    verbose_level: VerbosityLevel,

    // File tracking
    processed_files_list: Vec<PathBuf>,

    // Filters
    gitignore_filter: GitignoreFilter,

    // Statistics and Analysis
    file_stats: EnhancedFileStats,

    // Analysis components
    complexity_analyzer: ComplexityAnalyzer,
    language_detector: LanguageDetector,
    stats_analyzer: StatsAnalyzer,

    // Visualization components
    chart_generator: ChartGenerator,
    summary_generator: SummaryGenerator,
    meta_generator: MetaGenerator,
    llm_generator: LLMGenerator,
}

impl FileProcessor {
    pub fn new(config: Config) -> Self {
        Self {
            total_files: 0,
            processed_files: 0,
            total_size: 0,
            start_time: Instant::now(),
            progress_bar: ProgressBar::new(0),
            config: config.clone(),
            exclude_extensions: config.exclude_extensions.clone(),
            exclude_directories: config.exclude_directories.clone(),
            exclude_patterns: config.exclude_patterns.clone(),
            verbose_level: config.verbosity.clone(),
            processed_files_list: Vec::new(),
            gitignore_filter: GitignoreFilter::new(
                &config.dir_path,
                config.respect_gitignore,
                matches!(
                    config.verbosity,
                    VerbosityLevel::Debug | VerbosityLevel::Trace
                ),
            ),
            file_stats: EnhancedFileStats::default(),
            complexity_analyzer: ComplexityAnalyzer::new(),
            language_detector: LanguageDetector::new(),
            stats_analyzer: StatsAnalyzer::new(),
            chart_generator: ChartGenerator::new(),
            summary_generator: SummaryGenerator::new(),
            meta_generator: MetaGenerator::new(),
            llm_generator: LLMGenerator::new(),
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        self.log(VerbosityLevel::Info, "🔍 Scanning directory...");
        let path_to_scan = self.config.dir_path.clone();
        self.count_files(&path_to_scan)?;

        if !self.config.quiet {
            self.progress_bar = ProgressBar::new(self.total_files as u64);
            let style = match self.config.progress_style.as_str() {
                "simple" => ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] {pos}/{len}")
                    .unwrap(),
                "detailed" => ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) - {msg}")
                    .unwrap(),
                _ => ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len}")
                    .unwrap(),
            };
            self.progress_bar.set_style(style);
        }

        self.log(
            VerbosityLevel::Info,
            &format!("Found {} files to process", self.total_files),
        );
        Ok(())
    }

    pub fn process(&mut self) -> io::Result<()> {
        // Initialize
        self.init()?;

        // Ensure all output directories exist
        self.ensure_output_directories()?;

        // Process files
        self.log(VerbosityLevel::Info, "🔄 Processing files...");
        let path_to_process = self.config.dir_path.clone();
        self.process_directory(&path_to_process)?;

        // Calculate final metrics
        self.log(VerbosityLevel::Info, "📊 Calculating metrics...");
        self.stats_analyzer.calculate_metrics();

        // Generate outputs
        self.log(VerbosityLevel::Info, "📝 Generating outputs...");
        for output_type in &self.config.generated_types {
            match output_type {
                OutputType::Workspace => self.generate_workspace()?,
                OutputType::Files => self.generate_files_list()?,
                OutputType::Tree => self.generate_tree()?,
                OutputType::Summary => self.generate_summary()?,
                OutputType::Meta => self.generate_meta()?,
                OutputType::LLMFormat => self.generate_llm_format()?,
            }
        }

        // Finish up
        self.finish();
        Ok(())
    }

    fn ensure_output_directories(&self) -> io::Result<()> {
        // Create output directory if specified
        if let Some(output_dir) = &self.config.output_config.output_dir {
            if !output_dir.exists() {
                self.log(
                    VerbosityLevel::Info,
                    &format!("Creating output directory: {}", output_dir.display()),
                );
                fs::create_dir_all(output_dir)?;
            }
        }

        // Create directories for specific output files
        for output_path in self.config.output_config.outputs.values() {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    self.log(
                        VerbosityLevel::Info,
                        &format!("Creating directory: {}", parent.display()),
                    );
                    fs::create_dir_all(parent)?;
                }
            }
        }

        Ok(())
    }

    fn should_skip(&self, path: &Path) -> bool {
        // First check standard ignore patterns
        if should_ignore(path) {
            return true;
        }

        // Check gitignore patterns
        if self.gitignore_filter.is_ignored(path) {
            return true;
        }

        // Check excluded directories
        if self.should_skip_directory(path) {
            return true;
        }

        // Check custom exclude patterns
        let path_str = path.to_string_lossy();
        for pattern in &self.exclude_patterns {
            if path_str.contains(pattern) {
                self.log(
                    VerbosityLevel::Debug,
                    &format!("Skipping matched pattern: {}", path.display()),
                );
                return true;
            }
        }

        false
    }

    fn process_directory(&mut self, dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if self.should_skip(&path) {
                self.log(
                    VerbosityLevel::Debug,
                    &format!("Skipping: {}", path.display()),
                );
                continue;
            }

            if path.is_dir() {
                self.process_directory(&path)?;
            } else {
                // Check if file should be processed
                let (should_process, reason) = should_process_file(&path, &self.exclude_extensions);

                if !should_process {
                    self.log(
                        VerbosityLevel::Debug,
                        &format!("Skipping {}: {}", path.display(), reason),
                    );
                    continue;
                }

                // Process the file if it passed all checks
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        if let Err(e) = self.try_process_file(&path, &content) {
                            self.log_error_with_context(&e, "processing file", &path);
                            self.file_stats
                                .processing_errors
                                .push((path.to_path_buf(), e.to_string()));
                        }
                    }
                    Err(e) => {
                        self.log_error_with_context(&e, "reading file", &path);
                        self.file_stats
                            .access_errors
                            .push((path.to_path_buf(), e.to_string()));
                    }
                }
            }
        }
        Ok(())
    }

    fn try_process_file(&mut self, path: &Path, content: &str) -> io::Result<()> {
        let start_time = Instant::now();

        // Get file metadata
        let metadata = match fs::metadata(path) {
            Ok(meta) => meta,
            Err(e) => {
                self.log_error_with_context(&e, "reading metadata", path);
                return Err(e);
            }
        };

        // Update total size
        self.total_size += metadata.len();

        // Analyze file complexity
        let complexity = self.complexity_analyzer.analyze_file(content, path);

        // Calculate file metrics
        let metrics = self.calculate_file_metrics(content);

        // Detect language
        let language = self.language_detector.detect_language(path, content);

        // Create and store file statistics
        let file_stats = self.create_file_statistics(path, &metrics, complexity.clone())?;
        self.file_stats
            .file_statistics
            .insert(path.to_path_buf(), file_stats);

        // Update language statistics
        self.update_language_stats(language, &metrics, &complexity);

        // Store processing time
        let processing_time = start_time.elapsed();
        self.file_stats
            .processing_times
            .push((path.to_path_buf(), processing_time));

        // Update progress
        self.processed_files += 1;
        self.processed_files_list.push(path.to_path_buf());

        if !self.config.quiet {
            self.progress_bar
                .set_message(format!("Processing: {}", path.display()));
            self.progress_bar.inc(1);
        }

        self.log(
            VerbosityLevel::Debug,
            &format!(
                "Processed {} ({} bytes) in {:?}",
                path.display(),
                metadata.len(),
                processing_time
            ),
        );

        Ok(())
    }
    fn count_files(&mut self, dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if should_ignore(&path) {
                self.log(
                    VerbosityLevel::Debug,
                    &format!("🚫 Ignoring (count): {}", path.display()),
                );
                continue;
            }

            if self.should_skip_directory(&path) {
                self.log(
                    VerbosityLevel::Debug,
                    &format!("Skipping excluded directory (count): {}", path.display()),
                );
                continue;
            }

            if path.is_dir() {
                self.count_files(&path)?;
            } else {
                let (should_process, reason) = should_process_file(&path, &self.exclude_extensions);
                if should_process {
                    self.total_files += 1;
                    if let Ok(metadata) = fs::metadata(&path) {
                        self.total_size += metadata.len();
                    }
                } else {
                    self.log(
                        VerbosityLevel::Debug,
                        &format!("❌ Skipping (count) {}: {}", path.display(), reason),
                    );
                }
            }
        }
        Ok(())
    }

    fn should_skip_directory(&self, path: &Path) -> bool {
        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
            self.exclude_directories.contains(dir_name)
        } else {
            false
        }
    }

    fn calculate_file_metrics(&self, content: &str) -> FileMetrics {
        let mut metrics = FileMetrics::default();

        for line in content.lines() {
            let trimmed = line.trim();
            metrics.total_lines += 1;

            if trimmed.is_empty() {
                metrics.blank_lines += 1;
            } else if self.is_comment_line(trimmed) {
                metrics.comment_lines += 1;
            } else {
                metrics.code_lines += 1;
            }

            // Calculate line length statistics
            let line_length = line.len();
            metrics.max_line_length = metrics.max_line_length.max(line_length);
            metrics.total_line_length += line_length;
        }

        metrics.average_line_length = if metrics.total_lines > 0 {
            metrics.total_line_length as f64 / metrics.total_lines as f64
        } else {
            0.0
        };

        metrics
    }

    fn is_comment_line(&self, line: &str) -> bool {
        line.starts_with("//")
            || line.starts_with("#")
            || line.starts_with("/*")
            || line.starts_with("*")
            || line.contains("*/")
            || line.starts_with("'''")
            || line.starts_with("\"\"\"")
    }

    fn create_file_statistics(
        &self,
        path: &Path,
        metrics: &FileMetrics,
        complexity: CodeComplexity,
    ) -> io::Result<FileStatistics> {
        let metadata = fs::metadata(path)?;

        Ok(FileStatistics {
            path: path.to_path_buf(),
            size: metadata.len(),
            lines: metrics.total_lines,
            comments: metrics.comment_lines,
            blanks: metrics.blank_lines,
            code: metrics.code_lines,
            complexity,
            last_modified: metadata.modified()?.into(),
            last_author: String::new(),
            commit_count: 0,
            average_line_length: metrics.average_line_length,
            max_line_length: metrics.max_line_length,
        })
    }

    fn update_language_stats(
        &mut self,
        language: String,
        metrics: &FileMetrics,
        complexity: &CodeComplexity,
    ) {
        let stats = self.file_stats.language_stats.entry(language).or_default();

        stats.files += 1;
        stats.lines += metrics.total_lines;
        stats.code_lines += metrics.code_lines;
        stats.comment_lines += metrics.comment_lines;
        stats.blank_lines += metrics.blank_lines;
        stats.complexity = complexity.clone();
    }

    fn generate_workspace(&self) -> io::Result<()> {
        let output_path = self.config.get_output_path(&OutputType::Workspace);
        self.log(
            VerbosityLevel::Info,
            &format!("📝 Creating workspace file: {}", output_path.display()),
        );

        // Ensure directory exists
        self.ensure_directory_exists(&output_path)?;

        let file = File::create(&output_path)?;
        let mut writer = BufWriter::new(file);

        // Write header
        self.write_workspace_header(&mut writer)?;

        // Process and write each file's content
        for path in &self.processed_files_list {
            self.write_file_content(&mut writer, path)?;
        }

        // Write summary
        self.write_workspace_summary(&mut writer)?;

        writer.flush()?;
        self.log(
            VerbosityLevel::Info,
            "✅ Workspace file created successfully",
        );
        Ok(())
    }

    fn generate_files_list(&self) -> io::Result<()> {
        let output_path = self.config.get_output_path(&OutputType::Files);
        self.log(
            VerbosityLevel::Info,
            &format!("📑 Creating files list: {}", output_path.display()),
        );

        // Ensure directory exists
        self.ensure_directory_exists(&output_path)?;

        let file = File::create(&output_path)?;
        let mut writer = BufWriter::new(file);

        // Write header
        writeln!(writer, "# Processed Files List")?;
        writeln!(
            writer,
            "# Generated: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;
        writeln!(writer, "# Base Path: {}", self.config.dir_path.display())?;
        writeln!(writer, "# Total Files: {}", self.processed_files_list.len())?;
        writeln!(writer)?;

        // Group files by extension
        let mut files_by_type: HashMap<String, Vec<&Path>> = HashMap::new();
        for path in &self.processed_files_list {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_string();
            files_by_type.entry(ext).or_default().push(path);
        }

        // Write files by type
        for (ext, files) in files_by_type {
            writeln!(writer, "\n## {} files", ext.to_uppercase())?;
            for path in files {
                if let Ok(relative) = path.strip_prefix(&self.config.dir_path) {
                    writeln!(writer, "{}", relative.display())?;
                }
            }
        }

        writer.flush()?;
        self.log(VerbosityLevel::Info, "✅ Files list created successfully");
        Ok(())
    }

    fn generate_tree(&self) -> io::Result<()> {
        let output_path = self.config.get_output_path(&OutputType::Tree);
        self.log(
            VerbosityLevel::Info,
            &format!("🌳 Creating tree view: {}", output_path.display()),
        );

        // Ensure directory exists
        self.ensure_directory_exists(&output_path)?;

        let tree_output = TreeOutput::new(
            self.config.dir_path.clone(),
            self.verbose_level.clone(),
            self.config.respect_gitignore,
        );

        tree_output.generate(&output_path)?;

        self.log(VerbosityLevel::Info, "✅ Tree view created successfully");
        Ok(())
    }

    fn generate_summary(&self) -> io::Result<()> {
        let output_path = self.config.get_output_path(&OutputType::Summary);
        self.log(
            VerbosityLevel::Info,
            &format!("📋 Creating summary: {}", output_path.display()),
        );

        let file = File::create(&output_path)?;
        let mut writer = BufWriter::new(file);

        // Generate language distribution chart
        let language_data: Vec<(String, f64)> = self
            .file_stats
            .language_stats
            .iter()
            .map(|(lang, stats)| {
                let percentage = (stats.lines as f64 / self.file_stats.total_lines as f64) * 100.0;
                (lang.clone(), percentage)
            })
            .collect();

        self.chart_generator.generate_bar_chart(
            &mut writer,
            &language_data,
            "Language Distribution",
        )?;

        self.summary_generator.generate_summary(
            &mut writer,
            &self.file_stats,
            &self.config.dir_path.display().to_string(),
        )?;

        writer.flush()?;
        self.log(VerbosityLevel::Info, "✅ Summary created successfully");
        Ok(())
    }

    fn generate_meta(&self) -> io::Result<()> {
        let output_path = self.config.get_output_path(&OutputType::Meta);
        self.log(
            VerbosityLevel::Info,
            &format!("ℹ️ Creating metadata: {}", output_path.display()),
        );

        // Ensure directory exists
        self.ensure_directory_exists(&output_path)?;

        let file = File::create(&output_path)?;
        let mut writer = BufWriter::new(file);

        self.meta_generator
            .generate_metadata(&mut writer, &self.file_stats, &self.config)?;

        writer.flush()?;
        self.log(VerbosityLevel::Info, "✅ Metadata created successfully");
        Ok(())
    }
    fn write_workspace_header(&self, writer: &mut impl Write) -> io::Result<()> {
        writeln!(writer, "# Project Analysis Export")?;
        writeln!(
            writer,
            "Generated: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;
        writeln!(writer, "\n## Project Overview")?;
        writeln!(
            writer,
            "- Base Directory: {}",
            self.config.dir_path.display()
        )?;
        writeln!(writer, "- Total Files: {}", self.processed_files)?;
        writeln!(
            writer,
            "- Total Size: {}",
            self.format_file_size(self.total_size)
        )?;
        writeln!(writer, "\n## Language Distribution")?;

        // Add language statistics
        for (lang, stats) in &self.file_stats.language_stats {
            writeln!(writer, "\n### {}:", lang)?;
            writeln!(writer, "- Files: {}", stats.files)?;
            writeln!(writer, "- Total Lines: {}", stats.lines)?;
            writeln!(writer, "- Code Lines: {}", stats.code_lines)?;
            writeln!(writer, "- Comment Lines: {}", stats.comment_lines)?;
            writeln!(writer, "- Blank Lines: {}", stats.blank_lines)?;
        }

        writeln!(writer, "\n## File Contents")?;
        writeln!(
            writer,
            "Each file is separated by clear markers and includes metadata.\n"
        )?;
        Ok(())
    }

    fn write_file_content(&self, writer: &mut impl Write, path: &Path) -> io::Result<()> {
        writeln!(
            writer,
            "\n### File: {}",
            path.strip_prefix(&self.config.dir_path)
                .unwrap_or(path)
                .display()
        )?;

        // Write metadata
        if let Ok(metadata) = fs::metadata(path) {
            writeln!(writer, "#### Metadata")?;
            writeln!(writer, "- Size: {} bytes", metadata.len())?;
            if let Ok(modified) = metadata.modified() {
                let datetime: DateTime<Local> = modified.into();
                writeln!(
                    writer,
                    "- Modified: {}",
                    datetime.format("%Y-%m-%d %H:%M:%S")
                )?;
            }

            // Add file statistics if available
            if let Some(stats) = self.file_stats.file_statistics.get(path) {
                writeln!(writer, "- Lines of Code: {}", stats.code)?;
                writeln!(writer, "- Comment Lines: {}", stats.comments)?;
                writeln!(writer, "- Blank Lines: {}", stats.blanks)?;
                writeln!(
                    writer,
                    "- Cyclomatic Complexity: {:.2}",
                    stats.complexity.cyclomatic_complexity
                )?;
            }
        }

        // Write file content with language marker
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            writeln!(writer, "\n#### Content")?;
            writeln!(writer, "```{}", ext)?;
            if let Ok(content) = fs::read_to_string(path) {
                writeln!(writer, "{}", content)?;
            } else {
                writeln!(writer, "// Error: Could not read file content")?;
            }
            writeln!(writer, "```\n")?;
        }

        writeln!(writer, "---")?;
        Ok(())
    }

    fn write_workspace_summary(&self, writer: &mut impl Write) -> io::Result<()> {
        writeln!(writer, "\n## Project Summary")?;
        writeln!(writer, "### Statistics")?;
        writeln!(writer, "- Total Files Processed: {}", self.processed_files)?;
        writeln!(
            writer,
            "- Total Size: {}",
            self.format_file_size(self.total_size)
        )?;
        writeln!(
            writer,
            "- Processing Time: {}",
            self.format_duration(self.start_time.elapsed())
        )?;

        // Add complexity overview
        writeln!(writer, "\n### Complexity Overview")?;
        let mut complex_files: Vec<_> = self
            .file_stats
            .file_statistics
            .iter()
            .filter(|(_, stats)| stats.complexity.cyclomatic_complexity > 10.0)
            .collect();
        complex_files.sort_by(|a, b| {
            b.1.complexity
                .cyclomatic_complexity
                .partial_cmp(&a.1.complexity.cyclomatic_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        writeln!(writer, "\nMost Complex Files:")?;
        for (path, stats) in complex_files.iter().take(5) {
            writeln!(
                writer,
                "- {} (Complexity: {:.2})",
                path.strip_prefix(&self.config.dir_path)
                    .unwrap_or(path)
                    .display(),
                stats.complexity.cyclomatic_complexity
            )?;
        }

        // Add potential improvements section
        writeln!(writer, "\n### Potential Improvements")?;
        let files_needing_docs = self
            .file_stats
            .file_statistics
            .iter()
            .filter(|(_, stats)| (stats.comments as f64 / stats.lines as f64) < 0.1)
            .count();

        if files_needing_docs > 0 {
            writeln!(
                writer,
                "- {} files could benefit from additional documentation",
                files_needing_docs
            )?;
        }

        Ok(())
    }

    fn ensure_directory_exists(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }

    fn format_file_size(&self, size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{} B", size)
        }
    }

    fn format_duration(&self, duration: std::time::Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        let millis = duration.subsec_millis();

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else if seconds > 0 {
            format!("{}s {}ms", seconds, millis)
        } else {
            format!("{}ms", millis)
        }
    }

    fn log(&self, level: VerbosityLevel, message: &str) {
        if self.verbose_level >= level {
            match level {
                VerbosityLevel::Error => eprintln!("ERROR: {}", message),
                VerbosityLevel::Warn => eprintln!("WARN: {}", message),
                VerbosityLevel::Info => println!("INFO: {}", message),
                VerbosityLevel::Debug => println!("DEBUG: {}", message),
                VerbosityLevel::Trace => println!("TRACE: {}", message),
            }
        }
    }

    fn log_error_with_context(&self, error: &io::Error, context: &str, path: &Path) {
        self.log(
            VerbosityLevel::Error,
            &format!("Error during {} for {}: {}", context, path.display(), error),
        );
    }

    fn finish(&mut self) {
        if !self.config.quiet {
            self.progress_bar.finish_with_message("Complete!");
            let duration = self.start_time.elapsed();
            println!("\n✅ Processing completed:");
            println!("📁 Files processed: {}", self.processed_files);
            println!(
                "📊 Total size processed: {}",
                self.format_file_size(self.total_size)
            );
            println!("⏱️  Time taken: {}", self.format_duration(duration));

            if !self.file_stats.language_stats.is_empty() {
                println!("\n📊 Language Statistics:");
                let mut lang_stats: Vec<_> = self.file_stats.language_stats.iter().collect();
                lang_stats.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));

                for (lang, stats) in lang_stats {
                    println!(
                        "   {}: {} files ({} lines, {} code, {} comments, {} blank)",
                        lang,
                        stats.files,
                        stats.lines,
                        stats.code_lines,
                        stats.comment_lines,
                        stats.blank_lines
                    );
                }
            }

            if !self.exclude_extensions.is_empty() {
                println!("\n🚫 Excluded extensions: {:?}", self.exclude_extensions);
            }

            if let Some(error_count) = self.get_error_count() {
                if error_count > 0 {
                    println!("\n⚠️ Completed with {} errors", error_count);
                }
            }
        }
    }

    fn get_error_count(&self) -> Option<usize> {
        Some(
            self.file_stats.access_errors.len()
                + self.file_stats.processing_errors.len()
                + self.file_stats.output_errors.len(),
        )
    }

    fn generate_llm_format(&self) -> io::Result<()> {
        let output_path = self.config.get_output_path(&OutputType::LLMFormat);
        self.log(
            VerbosityLevel::Info,
            &format!("📊 Creating LLM format: {}", output_path.display()),
        );

        self.llm_generator
            .generate(&self.config.dir_path, &self.file_stats, &output_path)?;

        self.log(VerbosityLevel::Info, "✅ LLM format created successfully");
        Ok(())
    }
}

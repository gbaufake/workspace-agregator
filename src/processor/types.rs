use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Default)]
pub struct FileMetrics {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub total_line_length: usize,
    pub max_line_length: usize,
    pub average_line_length: f64,
}

#[derive(Clone, Serialize)]
pub struct CodeComplexity {
    pub lines_of_code: usize,
    pub cyclomatic_complexity: f64,
    pub comment_ratio: f64,
    pub depth_of_inheritance: usize,
    pub function_count: usize,
    pub class_count: usize,
}

impl Default for CodeComplexity {
    fn default() -> Self {
        Self {
            lines_of_code: 0,
            cyclomatic_complexity: 1.0,
            comment_ratio: 0.0,
            depth_of_inheritance: 0,
            function_count: 0,
            class_count: 0,
        }
    }
}

#[derive(Serialize)]
pub struct FileStatistics {
    pub path: PathBuf,
    pub size: u64,
    pub lines: usize,
    pub comments: usize,
    pub blanks: usize,
    pub code: usize,
    pub complexity: CodeComplexity,
    pub last_modified: DateTime<Utc>,
    pub last_author: String,
    pub commit_count: usize,
    pub average_line_length: f64,
    pub max_line_length: usize,
}

#[derive(Serialize, Default)]
pub struct LanguageStats {
    pub files: usize,
    pub lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub complexity: CodeComplexity,
}

#[derive(Serialize)]
pub struct ContributorStats {
    pub commits: usize,
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub files_modified: HashSet<PathBuf>,
    pub last_commit: DateTime<Utc>,
    pub first_commit: DateTime<Utc>,
}

#[derive(Default)]
pub struct EnhancedFileStats {
    pub extension_counts: HashMap<String, usize>,
    pub language_stats: HashMap<String, LanguageStats>,
    pub total_lines: usize,
    pub total_size: u64,
    pub largest_files: Vec<(PathBuf, u64)>,
    pub newest_files: Vec<(PathBuf, SystemTime)>,
    pub file_statistics: HashMap<PathBuf, FileStatistics>,
    pub commit_history: Vec<CommitInfo>,
    pub contributors: HashMap<String, ContributorStats>,
    pub dependencies: HashMap<String, Vec<String>>,
    pub complexity_metrics: ComplexityMetrics,
    pub processing_times: Vec<(PathBuf, Duration)>,
    pub access_errors: Vec<(PathBuf, String)>,
    pub processing_errors: Vec<(PathBuf, String)>,
    pub output_errors: Vec<(String, String)>,
}

#[derive(Serialize)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub message: String,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Default, Serialize)]
pub struct ComplexityMetrics {
    pub average: f64,
    pub maximum: f64,
    pub minimum: f64,
    pub standard_deviation: f64,
}

#[derive(Debug, Serialize)]
pub struct ProcessingMetrics {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_duration: Duration,
    pub files_per_second: f64,
    pub bytes_per_second: f64,
}

impl Default for ProcessingMetrics {
    fn default() -> Self {
        let now = DateTime::from(SystemTime::now());
        Self {
            start_time: now,
            end_time: now,
            total_duration: Duration::default(),
            files_per_second: 0.0,
            bytes_per_second: 0.0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorMetrics {
    pub total_errors: usize,
    pub access_errors: usize,
    pub processing_errors: usize,
    pub output_errors: usize,
    pub error_rate: f64,
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            access_errors: 0,
            processing_errors: 0,
            output_errors: 0,
            error_rate: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct PerformanceMetrics {
    pub processing: ProcessingMetrics,
    pub errors: ErrorMetrics,
    pub memory_usage: u64,
    pub peak_memory_usage: u64,
}

#[derive(Debug, Serialize)]
pub struct FileType {
    pub extension: String,
    pub mime_type: String,
    pub is_binary: bool,
    pub is_generated: bool,
    pub category: String,
}

#[derive(Debug, Serialize)]
pub struct FileTypeStats {
    pub file_type: FileType,
    pub count: usize,
    pub total_size: u64,
    pub average_size: f64,
}

impl Default for FileTypeStats {
    fn default() -> Self {
        Self {
            file_type: FileType {
                extension: String::new(),
                mime_type: String::new(),
                is_binary: false,
                is_generated: false,
                category: String::new(),
            },
            count: 0,
            total_size: 0,
            average_size: 0.0,
        }
    }
}

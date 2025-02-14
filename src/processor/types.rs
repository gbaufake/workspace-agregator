use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Default, Clone)]
pub struct EnhancedFileStats {
    pub extension_counts: HashMap<String, usize>,
    pub language_stats: HashMap<String, LanguageStats>,
    pub total_lines: usize,
    pub total_size: u64,
    pub file_statistics: HashMap<PathBuf, FileStatistics>,
    pub complexity_metrics: ComplexityMetrics,
    pub processing_times: Vec<(PathBuf, Duration)>,
    pub access_errors: Vec<(PathBuf, String)>,
    pub processing_errors: Vec<(PathBuf, String)>,
    pub output_errors: Vec<(String, String)>,
    pub largest_files: Vec<(PathBuf, u64)>, // Added this field
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LanguageStats {
    pub files: usize,
    pub lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub complexity: CodeComplexity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub average: f64,
    pub maximum: f64,
    pub minimum: f64,
    pub standard_deviation: f64,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceData {
    pub project: ProjectData,
    pub files: Vec<FileData>,
    pub analysis: AnalysisData,
}

#[derive(Debug, Serialize)]
pub struct ProjectData {
    pub timestamp: DateTime<Local>,
    pub base_directory: String,
    pub total_files: usize,
    pub total_size: u64,
    pub language_stats: HashMap<String, LanguageStats>,
}

#[derive(Debug, Serialize)]
pub struct FileData {
    pub path: String,
    pub size: u64,
    pub lines: usize,
    pub comments: usize,
    pub blanks: usize,
    pub code: usize,
    pub complexity: CodeComplexity,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AnalysisData {
    pub complexity_metrics: ComplexityMetrics,
    pub total_lines: usize,
    pub total_size: u64,
}

#[derive(Debug, Default)]
pub struct FileMetrics {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub total_line_length: usize,
    pub max_line_length: usize,
    pub average_line_length: f64,
}

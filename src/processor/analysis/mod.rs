pub mod complexity;
pub mod language;
pub mod stats;

pub use self::complexity::ComplexityAnalyzer;
pub use self::language::LanguageDetector;
pub use self::stats::StatsAnalyzer;

use crate::processor::types::FileMetrics;
use std::io;
use std::path::PathBuf;

pub struct CodeIndex {
    index_path: PathBuf,
}

impl CodeIndex {
    pub fn new(_path: &PathBuf) -> io::Result<Self> {
        Ok(Self {
            index_path: _path.clone(),
        })
    }

    pub fn find_symbol(&self, _name: &str) -> io::Result<Option<Vec<SymbolReference>>> {
        Ok(None)
    }
}

pub struct DependencyAnalyzer {
    dep_path: PathBuf,
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self {
            dep_path: PathBuf::new(),
        }
    }

    pub fn load_dependencies(_path: &PathBuf) -> io::Result<Self> {
        Ok(Self {
            dep_path: _path.clone(),
        })
    }

    pub fn get_dependencies(&self, _file: &PathBuf) -> Vec<String> {
        Vec::new()
    }

    pub fn get_dependents(&self, _file: &PathBuf) -> Vec<String> {
        Vec::new()
    }
}

pub struct MetricsAnalyzer {
    metrics_path: PathBuf,
}

impl MetricsAnalyzer {
    pub fn new() -> Self {
        Self {
            metrics_path: PathBuf::new(),
        }
    }

    pub fn load_file_metrics(_path: &PathBuf, _file: &PathBuf) -> io::Result<FileMetrics> {
        Ok(FileMetrics::default())
    }
}

#[derive(Debug)]
pub struct SymbolReference {
    pub file: PathBuf,
    pub line: usize,
    pub context: String,
}

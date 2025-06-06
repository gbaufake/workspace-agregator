pub mod cli;
pub mod config;
pub mod filters;
pub mod output;
pub mod processor;
pub mod version;

// Re-export commonly used items
pub use config::Config;
pub use processor::analysis::{CodeIndex, DependencyAnalyzer, MetricsAnalyzer};
pub use processor::types::*;
pub use processor::FileProcessor;
pub use version::*;

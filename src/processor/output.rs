use crate::processor::types::EnhancedFileStats;
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct EnhancedOutputGenerator {
    output_dir: PathBuf,
}

impl EnhancedOutputGenerator {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    pub fn generate(&self, _stats: &EnhancedFileStats) -> io::Result<()> {
        // Create output directory if it doesn't exist
        if !self.output_dir.exists() {
            fs::create_dir_all(&self.output_dir)?;
        }
        Ok(())
    }
}

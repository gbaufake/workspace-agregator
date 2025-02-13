use chrono::Local;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::config::VerbosityLevel;
use crate::processor::types::EnhancedFileStats;

pub struct FilesOutput {
    base_path: PathBuf,
    verbose_level: VerbosityLevel,
}

impl FilesOutput {
    pub fn new(base_path: PathBuf, verbose_level: VerbosityLevel) -> Self {
        Self {
            base_path,
            verbose_level,
        }
    }

    pub fn generate(&self, output_path: &Path, stats: &EnhancedFileStats) -> io::Result<()> {
        self.log(
            VerbosityLevel::Info,
            &format!("Generating files list: {}", output_path.display()),
        );

        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        // Write header
        self.write_header(&mut writer, stats)?;

        // Group files by type
        let mut files_by_type: HashMap<String, Vec<&Path>> = HashMap::new();

        for path in stats.file_statistics.keys() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_string();

            self.log(
                VerbosityLevel::Debug,
                &format!("Processing file: {} (type: {})", path.display(), ext),
            );
            files_by_type.entry(ext).or_default().push(path);
        }

        // Write each group
        for (ext, files) in files_by_type {
            self.log(
                VerbosityLevel::Debug,
                &format!("Writing {} files group", ext),
            );
            writeln!(writer, "\n## {} files", ext.to_uppercase())?;

            for path in files {
                if let Ok(relative) = path.strip_prefix(&self.base_path) {
                    writeln!(writer, "{}", relative.display())?;
                }
            }
        }

        // Write summary
        self.write_summary(&mut writer, stats)?;

        writer.flush()?;
        self.log(VerbosityLevel::Info, "Files list generated successfully");
        Ok(())
    }

    fn write_header(&self, writer: &mut impl Write, stats: &EnhancedFileStats) -> io::Result<()> {
        self.log(VerbosityLevel::Debug, "Writing header");
        writeln!(writer, "# Processed Files List")?;
        writeln!(
            writer,
            "# Generated: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;
        writeln!(writer, "# Base Path: {}", self.base_path.display())?;
        writeln!(writer, "# Total Files: {}", stats.file_statistics.len())?;
        writeln!(writer, "# Total Size: {} bytes", stats.total_size)?;
        writeln!(writer)?;
        Ok(())
    }

    fn write_summary(&self, writer: &mut impl Write, stats: &EnhancedFileStats) -> io::Result<()> {
        self.log(VerbosityLevel::Debug, "Writing summary");
        writeln!(writer, "\n## Summary")?;
        for (ext, count) in &stats.extension_counts {
            writeln!(writer, "{}: {} files", ext, count)?;
        }
        Ok(())
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
}

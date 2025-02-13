use chrono::{DateTime, Local};
use std::fs;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::config::VerbosityLevel;
use crate::processor::types::EnhancedFileStats;

pub struct WorkspaceOutput {
    base_path: PathBuf,
    verbose_level: VerbosityLevel,
}

impl WorkspaceOutput {
    pub fn new(base_path: PathBuf, verbose_level: VerbosityLevel) -> Self {
        Self {
            base_path,
            verbose_level,
        }
    }

    pub fn generate(&self, output_path: &Path, stats: &EnhancedFileStats) -> io::Result<()> {
        let file = fs::File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        // Write header
        self.write_header(&mut writer)?;

        // Process and write content
        self.write_content(&mut writer)?;

        // Write summary
        self.write_summary(&mut writer, stats)?;

        writer.flush()?;
        Ok(())
    }

    fn write_header(&self, writer: &mut impl Write) -> io::Result<()> {
        writeln!(writer, "{}", "=".repeat(100))?;
        writeln!(writer, "Workspace Content")?;
        writeln!(
            writer,
            "Generated: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;
        writeln!(writer, "Base Directory: {}", self.base_path.display())?;
        writeln!(writer, "{}\n", "=".repeat(100))?;
        Ok(())
    }

    fn write_content(&self, writer: &mut impl Write) -> io::Result<()> {
        self.process_directory(&self.base_path, writer)
    }

    fn process_directory(&self, dir: &Path, writer: &mut impl Write) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if self.should_skip(&path) {
                self.log(
                    VerbosityLevel::Debug,
                    &format!("🚫 Ignoring: {}", path.display()),
                );
                continue;
            }

            if path.is_dir() {
                self.log(
                    VerbosityLevel::Debug,
                    &format!("📁 Entering directory: {}", path.display()),
                );
                self.process_directory(&path, writer)?;
            } else {
                self.process_file(&path, writer)?;
            }
        }
        Ok(())
    }

    fn process_file(&self, path: &Path, writer: &mut impl Write) -> io::Result<()> {
        if let Ok(content) = fs::read_to_string(path) {
            self.write_file_content(path, &content, writer)?;
            self.log(
                VerbosityLevel::Debug,
                &format!("✅ Processed: {}", path.display()),
            );
        }
        Ok(())
    }

    fn write_file_content(
        &self,
        path: &Path,
        content: &str,
        writer: &mut impl Write,
    ) -> io::Result<()> {
        writeln!(writer, "\n{}", "=".repeat(100))?;
        writeln!(
            writer,
            "File: {}",
            path.strip_prefix(&self.base_path).unwrap_or(path).display()
        )?;

        if let Ok(metadata) = fs::metadata(path) {
            writeln!(writer, "Size: {} bytes", metadata.len())?;
            if let Ok(modified) = metadata.modified() {
                let datetime: DateTime<Local> = modified.into();
                writeln!(writer, "Modified: {}", datetime.format("%Y-%m-%d %H:%M:%S"))?;
            }
        }

        writeln!(writer, "{}", "=".repeat(100))?;
        writeln!(writer, "{}\n", content)?;
        Ok(())
    }

    fn write_summary(&self, writer: &mut impl Write, stats: &EnhancedFileStats) -> io::Result<()> {
        writeln!(writer, "\n{}", "=".repeat(100))?;
        writeln!(writer, "Summary")?;
        writeln!(writer, "{}", "=".repeat(100))?;
        writeln!(writer, "Total Files: {}", stats.file_statistics.len())?;
        writeln!(
            writer,
            "Total Size: {:.2} MB",
            stats.total_size as f64 / (1024.0 * 1024.0)
        )?;
        writeln!(writer, "Total Lines: {}", stats.total_lines)?;
        Ok(())
    }

    fn should_skip(&self, _path: &Path) -> bool {
        // Implementation of skip logic
        false
    }

    fn log(&self, level: VerbosityLevel, message: &str) {
        if level <= self.verbose_level {
            println!("{}", message);
        }
    }
}

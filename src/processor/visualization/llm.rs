use crate::processor::types::*;
use chrono::Local;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub struct LLMGenerator {
    chunk_size: usize,
}

#[derive(Debug)]
struct LLMChunk {
    sequence: usize,
    total_chunks: usize,
    content_type: String,
    content: String,
}

impl Default for LLMGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl LLMGenerator {
    pub fn new() -> Self {
        Self {
            chunk_size: 16000, // Approximately 4000 tokens
        }
    }

    pub fn generate(
        &self,
        base_path: &Path,
        stats: &EnhancedFileStats,
        output_path: &Path,
    ) -> io::Result<()> {
        // Create the main output file
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        // Write the main summary
        self.write_summary(&mut writer, stats)?;

        // Create the chunks directory
        let chunks_dir = output_path.with_extension("chunks");
        fs::create_dir_all(&chunks_dir)?;

        // Generate and write chunks
        let chunks = self.generate_chunks(base_path, stats);
        for chunk in chunks {
            let chunk_path = chunks_dir.join(format!(
                "chunk_{}_of_{}__{}.md",
                chunk.sequence, chunk.total_chunks, chunk.content_type
            ));
            let mut chunk_file = File::create(chunk_path)?;
            self.write_chunk(&mut chunk_file, &chunk)?;
        }

        Ok(())
    }

    fn write_summary(&self, writer: &mut impl Write, stats: &EnhancedFileStats) -> io::Result<()> {
        writeln!(writer, "# Project Code Analysis")?;
        writeln!(
            writer,
            "Generated: {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;

        // Project statistics
        writeln!(writer, "## Project Overview")?;
        writeln!(writer, "- Total Files: {}", stats.file_statistics.len())?;
        writeln!(writer, "- Total Lines: {}", stats.total_lines)?;
        writeln!(writer, "- Total Size: {} bytes\n", stats.total_size)?;

        // Language statistics
        writeln!(writer, "## Language Distribution")?;
        for (lang, stats) in &stats.language_stats {
            writeln!(writer, "### {}", lang)?;
            writeln!(writer, "- Files: {}", stats.files)?;
            writeln!(writer, "- Total Lines: {}", stats.lines)?;
            writeln!(writer, "- Code Lines: {}", stats.code_lines)?;
            writeln!(writer, "- Comment Lines: {}", stats.comment_lines)?;
            writeln!(
                writer,
                "- Comment Ratio: {:.2}%\n",
                (stats.comment_lines as f64 / stats.lines as f64) * 100.0
            )?;
        }

        // Code complexity overview
        writeln!(writer, "## Complexity Analysis")?;
        let mut complex_files: Vec<_> = stats
            .file_statistics
            .iter()
            .filter(|(_, s)| s.complexity.cyclomatic_complexity > 10.0)
            .collect();
        complex_files.sort_by(|a, b| {
            b.1.complexity
                .cyclomatic_complexity
                .partial_cmp(&a.1.complexity.cyclomatic_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        writeln!(writer, "Most Complex Files:")?;
        for (path, stats) in complex_files.iter().take(5) {
            writeln!(
                writer,
                "- {} (Complexity: {:.2})",
                path.display(),
                stats.complexity.cyclomatic_complexity
            )?;
        }

        writeln!(writer, "\n## Content Structure")?;
        writeln!(
            writer,
            "The code analysis is split into the following chunks:"
        )?;
        writeln!(writer, "1. Core Files (most complex/important files)")?;
        writeln!(writer, "2. Supporting Files (utility and helper files)")?;
        writeln!(writer, "3. Documentation and Configuration Files\n")?;

        Ok(())
    }

    fn generate_chunks(&self, _base_path: &Path, stats: &EnhancedFileStats) -> Vec<LLMChunk> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_size = 0;

        // Sort files by complexity
        let mut files: Vec<_> = stats.file_statistics.iter().collect();
        files.sort_by(|a, b| {
            b.1.complexity
                .cyclomatic_complexity
                .partial_cmp(&a.1.complexity.cyclomatic_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Process core files (high complexity)
        for (path, stats) in files
            .iter()
            .filter(|(_, s)| s.complexity.cyclomatic_complexity > 10.0)
        {
            if let Ok(content) = fs::read_to_string(path) {
                let file_content = self.format_file_content(path, stats, &content);

                if current_size + file_content.len() > self.chunk_size && !current_chunk.is_empty()
                {
                    chunks.push(self.create_chunk("core", current_chunk));
                    current_chunk = String::new();
                    current_size = 0;
                }

                current_chunk.push_str(&file_content);
                current_size += file_content.len();
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(self.create_chunk("core", current_chunk));
        }

        // Process supporting files
        current_chunk = String::new();
        current_size = 0;

        for (path, stats) in files
            .iter()
            .filter(|(_, s)| s.complexity.cyclomatic_complexity <= 10.0)
        {
            let summary = self.format_file_summary(path, stats);

            if current_size + summary.len() > self.chunk_size && !current_chunk.is_empty() {
                chunks.push(self.create_chunk("supporting", current_chunk));
                current_chunk = String::new();
                current_size = 0;
            }

            current_chunk.push_str(&summary);
            current_size += summary.len();
        }

        if !current_chunk.is_empty() {
            chunks.push(self.create_chunk("supporting", current_chunk));
        }

        // Update sequence numbers
        let total = chunks.len();
        for (i, chunk) in chunks.iter_mut().enumerate() {
            chunk.sequence = i + 1;
            chunk.total_chunks = total;
        }

        chunks
    }

    fn format_file_content(&self, path: &Path, stats: &FileStatistics, content: &str) -> String {
        format!(
            "\n### File: {}\n#### Metrics\n- Lines: {}\n- Complexity: {:.2}\n- Comments: {}\n\n```\n{}\n```\n",
            path.display(),
            stats.lines,
            stats.complexity.cyclomatic_complexity,
            stats.comments,
            content
        )
    }

    fn format_file_summary(&self, path: &Path, stats: &FileStatistics) -> String {
        format!(
            "\n### {}\n- Lines: {}\n- Complexity: {:.2}\n- Comments: {}\n",
            path.display(),
            stats.lines,
            stats.complexity.cyclomatic_complexity,
            stats.comments
        )
    }

    fn create_chunk(&self, chunk_type: &str, content: String) -> LLMChunk {
        LLMChunk {
            sequence: 0,
            total_chunks: 0,
            content_type: chunk_type.to_string(),
            content,
        }
    }

    fn write_chunk(&self, writer: &mut impl Write, chunk: &LLMChunk) -> io::Result<()> {
        writeln!(
            writer,
            "# Code Analysis Chunk {}/{}",
            chunk.sequence, chunk.total_chunks
        )?;
        writeln!(writer, "Type: {}\n", chunk.content_type)?;
        writer.write_all(chunk.content.as_bytes())?;
        Ok(())
    }
}

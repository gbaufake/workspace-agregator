use chrono::Local;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::config::VerbosityLevel;
use crate::filters::gitignore::GitignoreFilter;
use crate::filters::patterns::should_ignore;

pub struct TreeOutput {
    base_path: PathBuf,
    verbose_level: VerbosityLevel,
    gitignore_filter: Option<GitignoreFilter>,
    respect_gitignore: bool,
}

impl TreeOutput {
    pub fn new(base_path: PathBuf, verbose_level: VerbosityLevel, respect_gitignore: bool) -> Self {
        let gitignore_filter = if respect_gitignore {
            Some(GitignoreFilter::new(
                &base_path,
                true,
                matches!(verbose_level, VerbosityLevel::Debug | VerbosityLevel::Trace),
            ))
        } else {
            None
        };

        Self {
            base_path,
            verbose_level,
            gitignore_filter,
            respect_gitignore,
        }
    }

    pub fn generate(&self, output_path: &Path) -> io::Result<()> {
        self.log(
            VerbosityLevel::Info,
            &format!("Generating tree view: {}", output_path.display()),
        );

        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        // Write header
        self.write_header(&mut writer)?;

        // Generate tree
        self.log(VerbosityLevel::Debug, "Generating directory tree");
        self.print_tree(&self.base_path, "", true, &mut writer)?;

        writer.flush()?;
        self.log(VerbosityLevel::Info, "Tree view generated successfully");
        Ok(())
    }

    fn should_skip(&self, path: &Path) -> bool {
        // Check standard ignore patterns
        if should_ignore(path) {
            self.log(
                VerbosityLevel::Debug,
                &format!("Skipping ignored path: {}", path.display()),
            );
            return true;
        }

        // Check gitignore if enabled
        if self.respect_gitignore {
            if let Some(ref gitignore) = self.gitignore_filter {
                if gitignore.is_ignored(path) {
                    self.log(
                        VerbosityLevel::Debug,
                        &format!("Skipping gitignored path: {}", path.display()),
                    );
                    return true;
                }
            }
        }

        false
    }

    fn write_header(&self, writer: &mut impl Write) -> io::Result<()> {
        self.log(VerbosityLevel::Debug, "Writing header");
        writeln!(writer, "Directory Tree for: {}", self.base_path.display())?;
        writeln!(
            writer,
            "Generated: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;
        if self.respect_gitignore {
            writeln!(writer, "Note: Respecting .gitignore rules")?;
        }
        writeln!(writer)?;
        Ok(())
    }

    fn print_tree(
        &self,
        dir: &Path,
        prefix: &str,
        is_last: bool,
        writer: &mut impl Write,
    ) -> io::Result<()> {
        self.log(
            VerbosityLevel::Trace,
            &format!("Processing directory: {}", dir.display()),
        );

        // Get all entries and filter out ignored ones
        let mut entries: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(Result::ok)
            .filter(|entry| !self.should_skip(&entry.path()))
            .collect();

        // Sort entries (directories first, then files)
        entries.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            if a_is_dir == b_is_dir {
                a.path().file_name().cmp(&b.path().file_name())
            } else {
                b_is_dir.cmp(&a_is_dir)
            }
        });

        let total = entries.len();

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_current_last = i == total - 1;

            let branch = if is_last { "└── " } else { "├── " };
            let next_prefix = if is_last { "    " } else { "│   " };

            if path.is_dir() {
                self.log(
                    VerbosityLevel::Trace,
                    &format!("Writing directory: {}", path.display()),
                );
                writeln!(
                    writer,
                    "{}{}{}/",
                    prefix,
                    branch,
                    path.file_name().unwrap().to_string_lossy()
                )?;
                self.print_tree(
                    &path,
                    &format!("{}{}", prefix, next_prefix),
                    is_current_last,
                    writer,
                )?;
            } else {
                self.log(
                    VerbosityLevel::Trace,
                    &format!("Writing file: {}", path.display()),
                );
                writeln!(
                    writer,
                    "{}{}{}",
                    prefix,
                    branch,
                    path.file_name().unwrap().to_string_lossy()
                )?;
            }
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

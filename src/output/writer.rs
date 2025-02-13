use std::io::{self, Write, BufWriter};
use std::fs::File;
use std::path::Path;
use chrono::Local;

pub fn write_file_content(file_path: &Path, writer: &mut BufWriter<File>) -> io::Result<()> {
    let content = std::fs::read_to_string(file_path)?;
    let relative_path = file_path.to_string_lossy();
    let metadata = std::fs::metadata(file_path)?;
    let modified: chrono::DateTime<Local> = metadata.modified()?.into();
    
    writeln!(writer, "\n{}", "=".repeat(100))?;
    writeln!(writer, "File: {}", relative_path)?;
    writeln!(writer, "Size: {} bytes", metadata.len())?;
    writeln!(writer, "Modified: {}", modified.format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(writer, "{}", "=".repeat(100))?;
    writeln!(writer, "{}\n", content)?;
    Ok(())
}

pub fn write_file_list(
    output_file: &str,
    files: &[impl AsRef<Path>],
    base_path: &Path,
    total_size: u64,
) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(output_file)?);

    writeln!(writer, "# Processed Files List")?;
    writeln!(writer, "# Generated: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(writer, "# Base Path: {}", base_path.display())?;
    writeln!(writer, "# Total Files: {}", files.len())?;
    writeln!(writer, "# Total Size: {} bytes", total_size)?;
    writeln!(writer)?;

    for path in files {
        if let Ok(relative) = path.as_ref().strip_prefix(base_path) {
            writeln!(writer, "{}", relative.display())?;
        } else {
            writeln!(writer, "{}", path.as_ref().display())?;
        }
    }

    writer.flush()?;
    Ok(())
}
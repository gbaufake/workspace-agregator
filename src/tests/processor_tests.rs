use std::path::PathBuf;
use workspace_aggregator::{Config, FileProcessor};
use std::fs;

#[test]
fn test_processor_initialization() {
    // Create test directory and file
    let test_dir = PathBuf::from("test_data");
    fs::create_dir_all(&test_dir).unwrap();
    fs::write(test_dir.join("test.rs"), "fn main() {}").unwrap();

    // Set test arguments
    std::env::set_var("CARGO_TEST_ARGS", format!("workspace-aggregator {}", test_dir.display()));

    let config = Config::new().expect("Failed to create config");
    let processor = FileProcessor::new(config);

    assert_eq!(processor.processed_files(), 0);
    assert_eq!(processor.total_size(), 0);

    // Cleanup
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_file_exclusion() {
    // Create test directory and files
    let test_dir = PathBuf::from("test_data");
    fs::create_dir_all(&test_dir).unwrap();
    fs::write(test_dir.join("test.rs"), "fn main() {}").unwrap();
    fs::write(test_dir.join("README.md"), "# Test").unwrap();

    // Set test arguments with md exclusion
    std::env::set_var(
        "CARGO_TEST_ARGS",
        format!("workspace-aggregator {} --exclude md", test_dir.display())
    );

    let config = Config::new().expect("Failed to create config");
    let mut processor = FileProcessor::new(config);

    processor.process().expect("Processing failed");

    // Verify only .rs file was processed
    assert_eq!(processor.processed_files(), 1);

    // Cleanup
    fs::remove_dir_all(test_dir).unwrap();
}

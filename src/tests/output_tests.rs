use std::path::PathBuf;
use workspace_aggregator::{Config, FileProcessor};
use std::fs;

#[test]
fn test_output_generation() {
    // Create test directory and file
    let test_dir = PathBuf::from("test_data");
    let output_dir = PathBuf::from("test_output");
    fs::create_dir_all(&test_dir).unwrap();
    fs::write(test_dir.join("test.rs"), "fn main() {}").unwrap();

    // Set test arguments
    std::env::set_var(
        "CARGO_TEST_ARGS",
        format!(
            "workspace-aggregator {} --output-dir {}",
            test_dir.display(),
            output_dir.display()
        )
    );

    let config = Config::new().expect("Failed to create config");
    let mut processor = FileProcessor::new(config);

    processor.process().expect("Processing failed");

    // Verify output files exist
    assert!(output_dir.exists());
    assert!(output_dir.join("workspace.txt").exists());

    // Cleanup
    fs::remove_dir_all(test_dir).unwrap();
    fs::remove_dir_all(output_dir).unwrap();
}

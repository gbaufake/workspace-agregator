use std::path::PathBuf;
use workspace_aggregator::Config;
use std::env;

#[test]
fn test_basic_config() {
    // Set test command line arguments
    env::set_var("CARGO_TEST_ARGS", "workspace-aggregator .");
    
    let config = Config::new().expect("Failed to create config");
    
    assert_eq!(config.dir_path, PathBuf::from("."));
    assert!(config.exclude_extensions.is_empty());
    assert!(!config.verbose);
    assert!(!config.quiet);
}

#[test]
fn test_config_with_options() {
    // Set test command line arguments with options
    env::set_var("CARGO_TEST_ARGS", "workspace-aggregator . --exclude md --output-dir ./docs");
    
    let config = Config::new().expect("Failed to create config");
    
    assert_eq!(config.dir_path, PathBuf::from("."));
    assert!(config.exclude_extensions.contains("md"));
    assert_eq!(config.output_config.output_dir, Some(PathBuf::from("./docs")));
}

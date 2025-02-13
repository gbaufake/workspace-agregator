use crate::cli::help::{print_help, print_short_help, print_version};
use chrono::Local;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum OutputType {
    Workspace,
    Files,
    Tree,
    Summary,
    Meta,
    LLMFormat,
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Default)]
pub enum VerbosityLevel {
    #[default]
    Info,
    Error,
    Warn,
    Debug,
    Trace,
}

#[derive(Clone, Default)]
pub struct Config {
    pub dir_path: PathBuf,
    pub exclude_extensions: HashSet<String>,
    pub exclude_directories: HashSet<String>,
    pub exclude_patterns: HashSet<String>,
    pub verbose: bool,
    pub quiet: bool,
    pub progress_style: String,
    pub respect_gitignore: bool,
    pub generated_types: HashSet<OutputType>,
    pub output_config: OutputConfig,
    pub verbosity: VerbosityLevel,
}

#[derive(Clone, Default)]
pub struct OutputConfig {
    pub output_dir: Option<PathBuf>,
    pub outputs: HashMap<OutputType, PathBuf>,
    pub use_timestamp: bool,
}

impl Config {
    pub fn new() -> io::Result<Self> {
        let _args: Vec<String> = std::env::args().collect();

        let args: Vec<String> = if cfg!(test) {
            env::var("CARGO_TEST_ARGS")
                .map(|args| args.split_whitespace().map(String::from).collect())
                .unwrap_or_else(|_| vec!["workspace-aggregator".to_string(), ".".to_string()])
        } else {
            env::args().collect()
        };

        if args.len() <= 1
            || args.contains(&"--help".to_string())
            || args.contains(&"-h".to_string())
        {
            print_help();
            return Err(Error::new(ErrorKind::Other, "Help displayed"));
        }

        if args.contains(&"--version".to_string()) || args.contains(&"-v".to_string()) {
            print_version();
            return Err(Error::new(ErrorKind::Other, "Version displayed"));
        }

        let mut config = Config {
            dir_path: PathBuf::new(),
            exclude_extensions: HashSet::new(),
            exclude_directories: HashSet::new(),
            exclude_patterns: HashSet::new(),
            verbose: false,
            quiet: false,
            progress_style: String::from("detailed"),
            respect_gitignore: false,
            generated_types: HashSet::new(),
            output_config: OutputConfig::default(),
            verbosity: VerbosityLevel::Info,
        };

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--generate" => {
                    i += 1;
                    if i < args.len() {
                        for typ in args[i].split(',') {
                            match typ.trim() {
                                "workspace" => {
                                    config.generated_types.insert(OutputType::Workspace);
                                }
                                "files" => {
                                    config.generated_types.insert(OutputType::Files);
                                }
                                "tree" => {
                                    config.generated_types.insert(OutputType::Tree);
                                }
                                "summary" => {
                                    config.generated_types.insert(OutputType::Summary);
                                }
                                "meta" => {
                                    config.generated_types.insert(OutputType::Meta);
                                }
                                "llm" => {
                                    config.generated_types.insert(OutputType::LLMFormat);
                                }
                                _ => {
                                    println!("Warning: Unknown output type: {}", typ);
                                }
                            }
                        }
                        println!("Debug: Generated types: {:?}", config.generated_types);
                    }
                }
                "--output-dir" => {
                    i += 1;
                    if i < args.len() {
                        config.output_config.output_dir = Some(PathBuf::from(&args[i]));
                        println!(
                            "Debug: Output directory set to: {:?}",
                            config.output_config.output_dir
                        );
                    }
                }
                "--exclude" => {
                    i += 1;
                    if i < args.len() {
                        config.exclude_extensions = args[i]
                            .split(',')
                            .map(|s| s.trim().to_lowercase())
                            .collect();
                        println!(
                            "Debug: Excluding extensions: {:?}",
                            config.exclude_extensions
                        );
                    }
                }
                "--exclude-dir" => {
                    i += 1;
                    if i < args.len() {
                        config.exclude_directories =
                            args[i].split(',').map(|s| s.trim().to_string()).collect();
                        println!(
                            "Debug: Excluding directories: {:?}",
                            config.exclude_directories
                        );
                    }
                }
                "--exclude-pattern" => {
                    i += 1;
                    if i < args.len() {
                        config.exclude_patterns =
                            args[i].split(',').map(|s| s.trim().to_string()).collect();
                        println!("Debug: Excluding patterns: {:?}", config.exclude_patterns);
                    }
                }
                "--verbose" => {
                    config.verbose = true;
                    config.verbosity = VerbosityLevel::Debug;
                }
                "--quiet" => {
                    config.quiet = true;
                }
                "--respect-gitignore" => {
                    config.respect_gitignore = true;
                }
                "--verbosity" => {
                    i += 1;
                    if i < args.len() {
                        config.verbosity = match args[i].to_lowercase().as_str() {
                            "error" => VerbosityLevel::Error,
                            "warn" => VerbosityLevel::Warn,
                            "info" => VerbosityLevel::Info,
                            "debug" => VerbosityLevel::Debug,
                            "trace" => VerbosityLevel::Trace,
                            _ => VerbosityLevel::Info,
                        };
                    }
                }
                "--progress-style" => {
                    i += 1;
                    if i < args.len() {
                        config.progress_style = args[i].to_string();
                    }
                }
                arg if arg.starts_with("--output=") => {
                    let parts: Vec<&str> = arg.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let output_type = match parts[1] {
                            "workspace" => Some(OutputType::Workspace),
                            "files" => Some(OutputType::Files),
                            "tree" => Some(OutputType::Tree),
                            "summary" => Some(OutputType::Summary),
                            "meta" => Some(OutputType::Meta),
                            _ => None,
                        };
                        if let Some(output_type) = output_type {
                            i += 1;
                            if i < args.len() {
                                config
                                    .output_config
                                    .outputs
                                    .insert(output_type, PathBuf::from(&args[i]));
                            }
                        }
                    }
                }
                _ => {
                    if config.dir_path.as_os_str().is_empty() {
                        config.dir_path = PathBuf::from(&args[i]);
                    }
                }
            }
            i += 1;
        }

        // Validate directory
        if config.dir_path.as_os_str().is_empty() {
            print_short_help();
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No directory specified",
            ));
        }

        if !config.dir_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Directory not found: {}", config.dir_path.display()),
            ));
        }

        if !config.dir_path.is_dir() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Not a directory: {}", config.dir_path.display()),
            ));
        }

        // Create output directory if specified
        if let Some(output_dir) = &config.output_config.output_dir {
            if !output_dir.exists() {
                std::fs::create_dir_all(output_dir)?;
            }
        }

        // Set default generated types if none specified
        if config.generated_types.is_empty() {
            config.generated_types.insert(OutputType::Workspace);
            config.generated_types.insert(OutputType::Files);
            config.generated_types.insert(OutputType::Tree);
            config.generated_types.insert(OutputType::Summary);
            config.generated_types.insert(OutputType::Meta);
        }

        Ok(config)
    }

    pub fn get_output_path(&self, output_type: &OutputType) -> PathBuf {
        // Check if there's a specific path for this output type
        if let Some(path) = self.output_config.outputs.get(output_type) {
            return path.clone();
        }

        // Otherwise construct default path
        let timestamp = if self.output_config.use_timestamp {
            format!("_{}", Local::now().format("%Y%m%d_%H%M%S"))
        } else {
            String::new()
        };

        let filename = match output_type {
            OutputType::Workspace => format!("workspace{}.txt", timestamp),
            OutputType::Files => format!("files{}.txt", timestamp),
            OutputType::Tree => format!("tree{}.txt", timestamp),
            OutputType::Summary => format!("summary{}.txt", timestamp),
            OutputType::Meta => format!("meta{}.json", timestamp),
            OutputType::LLMFormat => format!("llm{}.md", timestamp), // Add this line
        };

        if let Some(dir) = &self.output_config.output_dir {
            dir.join(filename)
        } else {
            PathBuf::from(filename)
        }
    }
}

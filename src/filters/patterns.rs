use std::collections::HashSet;
use std::path::Path;

pub fn should_ignore(path: &Path) -> bool {
    let ignore_patterns = vec![
        // Virtual Environments
        ".venv",
        "venv",
        "env",
        "virtualenv",
        // Build and Cache
        "target",
        "dist",
        "build",
        "__pycache__",
        ".cache",
        ".next",
        "tmp",
        // Dependencies
        "node_modules",
        "site-packages",
        "vendor",
        "deps",
        // IDE and Config
        ".git",
        ".idea",
        ".vscode",
        ".env",
        ".DS_Store",
        // Coverage and Tests
        "coverage",
        ".coverage",
        ".pytest_cache",
        "__tests__",
        "test-results",
        // Other
        ".terraform",
        ".serverless",
        ".aws-sam",
    ];

    let path_str = path.to_string_lossy();

    if path_str
        .split('/')
        .any(|part| part.starts_with('.') && part != "." && part != "..")
    {
        return true;
    }

    for pattern in &ignore_patterns {
        if path_str.contains(&format!("/{}/", pattern))
            || path_str.starts_with(&format!("{}/", pattern))
            || path_str.ends_with(&format!("/{}", pattern))
        {
            return true;
        }
    }

    false
}

pub fn should_process_file(path: &Path, exclude_extensions: &HashSet<String>) -> (bool, String) {
    let extensions = vec![
        // Programming Languages
        "txt",
        "md",
        "rs",
        "py",
        "js",
        "jsx",
        "ts",
        "tsx",
        "java",
        "c",
        "cpp",
        "h",
        "hpp",
        "cs",
        "go",
        "php",
        "rb",
        "swift",
        "kt",
        "scala",
        // Web
        "html",
        "htm",
        "css",
        "scss",
        "sass",
        "less",
        "svg",
        // Config & Data
        "json",
        "yaml",
        "yml",
        "xml",
        "toml",
        "ini",
        "conf",
        "config",
        "properties",
        "props",
        "env",
        // Documentation
        "markdown",
        "rst",
        "asciidoc",
        "adoc",
        // Scripts
        "sh",
        "bash",
        "zsh",
        "fish",
        "ps1",
        "bat",
        "cmd",
        // Other
        "sql",
        "graphql",
        "proto",
    ];

    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            let ext_lower = ext_str.to_lowercase();
            let is_valid_ext = extensions.contains(&ext_lower.as_str());
            let is_not_excluded = !exclude_extensions.contains(&ext_lower);

            let reason = if !is_valid_ext {
                format!("Unsupported extension: {}", ext_lower)
            } else if !is_not_excluded {
                format!("Excluded extension: {}", ext_lower)
            } else {
                String::new()
            };

            return (is_valid_ext && is_not_excluded, reason);
        }
    }
    (false, "No extension".to_string())
}

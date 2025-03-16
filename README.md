# Workspace Aggregator

[![Version](https://img.shields.io/badge/version-0.3.0-blue.svg)](https://crates.io/crates/workspace-aggregator)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)

> A powerful tool for analyzing, documenting, and understanding your codebase.

## Features

### Analysis Tools 📊
- Language detection and statistics
- Code complexity metrics
- File size and line count analysis
- Directory structure visualization

### Documentation 📝
- Aggregated workspace content
- Processed files listing
- Directory tree generation
- Project statistics and summaries
- JSON metadata export

### Smart Filtering 🔍
- Extension-based filtering
- Directory exclusions
- Pattern matching
- .gitignore integration

## Quick Start

```bash
# Install
cargo install workspace-aggregator

# Basic usage
workspace-aggregator /path/to/project

# Generate all outputs
workspace-aggregator . --generate workspace,files,tree,stats,summary,meta
```

## Documentation

- [Installation Guide](docs/INSTALLATION.md)
- [Usage Guide](docs/USAGE.md)
- [Examples](docs/EXAMPLES.md)
- [Contributing](docs/CONTRIBUTING.md)

## Supported Output Types

| Type | Description | Default Filename |
|------|-------------|-----------------|
| `workspace` | Aggregated file contents | `workspace_YYYYMMDD_HHMMSS.txt` |
| `files` | List of processed files | `files_YYYYMMDD_HHMMSS.txt` |
| `tree` | Directory structure | `tree_YYYYMMDD_HHMMSS.txt` |
| `stats` | Detailed statistics | `stats_YYYYMMDD_HHMMSS.txt` |
| `summary` | Project overview | `summary_YYYYMMDD_HHMMSS.txt` |
| `meta` | JSON metadata | `meta_YYYYMMDD_HHMMSS.json` |

## Command Options

### Output Control
```bash
--output-dir <path>              # Set output directory for all files
--output <type>=<path>          # Set specific output file path
--generate <type1,type2,...>    # Specify which outputs to generate
--no-timestamp                  # Disable timestamps in filenames
```

### Filtering Options
```bash
--exclude <ext1,ext2,...>       # Exclude file extensions
--exclude-dir <dir1,dir2,...>   # Exclude directories
--exclude-pattern <pattern>     # Exclude by pattern
--respect-gitignore            # Use .gitignore rules
```

### Display Options
```bash
--verbosity <level>            # Set verbosity (error|warn|info|debug|trace)
--progress-style <style>       # Set progress style (simple|detailed|none)
--quiet                       # Minimal output
--verbose                     # Detailed output
```

## Example Usage

```bash
# Generate complete documentation
workspace-aggregator .
  --generate workspace,files,tree,stats,summary,meta
  --output-dir ./docs
  --verbose

# Analyze specific directories
workspace-aggregator ./src
  --exclude test,spec
  --exclude-dir __tests__,fixtures
  --generate stats,summary
  --verbosity debug

# Quick overview
workspace-aggregator .
  --generate summary,tree
  --quiet
```

## Output Examples

### Directory Tree
```
project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── integration_tests.rs
└── Cargo.toml
```

### Statistics Summary
```
Project Statistics
-----------------
Total Files: 42
Total Size: 1.2 MB
Languages:
  - Rust: 65%
  - Python: 20%
  - JavaScript: 15%
```

## Supported File Types

### Programming Languages
- Rust (`.rs`)
- Python (`.py`)
- JavaScript (`.js`, `.jsx`)
- TypeScript (`.ts`, `.tsx`)
- Java (`.java`)
- C/C++ (`.c`, `.cpp`, `.h`, `.hpp`)

### Web Technologies
- HTML (`.html`, `.htm`)
- CSS (`.css`, `.scss`, `.sass`)
- SVG (`.svg`)

### Configuration
- JSON (`.json`)
- YAML (`.yml`, `.yaml`)
- TOML (`.toml`)
- INI (`.ini`)

### Documentation
- Markdown (`.md`)
- Text (`.txt`)
- RST (`.rst`)
- AsciiDoc (`.adoc`)

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -am 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Guilherme Baufaker Rêgo**
- 📧 Email: baufaker@protonmail.com
- 🌟 GitHub: [@yourusername](https://github.com/yourusername)

---

Made with ❤️ in Rust

## 🗺️ Roadmap

### Phase 1: Core Enhancements (Q1 2025)
🎯 Fundamental improvements to enhance basic functionality

- [ ] Search and Filter
  - [ ] `--search` Text pattern search
  - [ ] `--after/--before` Date filtering
  - [ ] `--size` File size filtering
- [ ] Performance
  - [ ] `--parallel` Parallel processing
  - [ ] `--cache` Result caching
  - [ ] `--incremental` Incremental updates
- [ ] Export Formats
  - [ ] `--format markdown` Markdown output
  - [ ] `--format html` HTML with syntax highlighting
  - [ ] `--format json` JSON metadata export

### Phase 2: Analysis Tools (Q2 2025)
📊 Code analysis and quality assessment features

- [ ] Content Analysis
  - [ ] `--stats` Language and file type statistics
  - [ ] `--count-lines` Line counting by file type
  - [ ] `--duplicates` Code duplication detection
- [ ] Code Quality
  - [ ] `--lint` Basic linting integration
  - [ ] `--todo-extract` TODO/FIXME extraction
  - [ ] `--dead-code` Dead code detection
- [ ] Documentation
  - [ ] `--auto-doc` Documentation generation
  - [ ] `--readme-gen` README generation
  - [ ] `--api-doc` API documentation extraction

### Phase 3: Integration & Collaboration (Q3 2025)
🤝 Version control and team collaboration features

- [ ] Git Integration
  - [ ] `--git-blame` Git blame information
  - [ ] `--github-issues` GitHub issue linking
  - [ ] `--changelog` Changelog generation
- [ ] Team Features
  - [ ] `--team-annotations` Team annotations
  - [ ] `--review-ready` Code review preparation
  - [ ] `--share-cloud` Cloud sharing capabilities
- [ ] Workflow Integration
  - [ ] `--ci-mode` CI/CD pipeline mode
  - [ ] `--pre-commit` Pre-commit hooks
  - [ ] `--notify-slack` Slack notifications

### Phase 4: Advanced Features (Q4 2025)
🚀 Advanced analysis and AI-powered features

- [ ] Security & Compliance
  - [ ] `--secrets-scan` Secret detection
  - [ ] `--pii-detect` PII data detection
  - [ ] `--license-check` License scanning
- [ ] Visualization
  - [ ] `--generate-graph` Dependency graphing
  - [ ] `--heatmap` Code modification heatmap
  - [ ] `--timeline` File timeline visualization
- [ ] AI Features
  - [ ] `--ai-summary` AI-powered code summarization
  - [ ] `--suggest-docs` Documentation suggestions
  - [ ] `--detect-patterns` Design pattern detection

### Phase 5: Enterprise Features (2026)
💼 Enterprise-grade features and integrations

- [ ] Project Management
  - [ ] `--estimate` Project effort estimation
  - [ ] `--metrics` Project health metrics
  - [ ] `--roadmap` Roadmap extraction
- [ ] Enterprise Integration
  - [ ] `--jira-links` JIRA integration
  - [ ] `--confluence-export` Confluence export
  - [ ] `--style corporate` Corporate theming
- [ ] Advanced Export
  - [ ] `--format pdf` PDF export
  - [ ] `--wiki-export` Wiki export
  - [ ] Custom templating system


## 📦 Output Files

The tool generates up to three types of output files:

1. 📄 **Workspace Content** (`workspace_YYYYMMDD_HHMMSS.txt`)
   - Contains the content of all processed files
   - Includes file metadata and separators

2. 📑 **Processed Files List** (`processed_YYYYMMDD_HHMMSS.txt`)
   - Lists all files that were processed
   - Includes timestamp and statistics

3. 🌳 **Tree View** (`tree_YYYYMMDD_HHMMSS.txt`)
   - Shows directory structure
   - Includes only processed files

## 🔧 Supported File Types

### Programming Languages
- 💻 Python (`.py`)
- 🌐 JavaScript/TypeScript (`.js`, `.ts`, `.jsx`, `.tsx`)
- ☕ Java (`.java`)
- 🦀 Rust (`.rs`)
- 🔨 C/C++ (`.c`, `.cpp`, `.h`, `.hpp`)

### Web Technologies
- 🌐 HTML (`.html`, `.htm`)
- 🎨 CSS/SCSS (`.css`, `.scss`, `.sass`)
- 📐 SVG (`.svg`)

### Configuration
- ⚙️ JSON (`.json`)
- 🔧 YAML (`.yml`, `.yaml`)
- 📝 TOML (`.toml`)
- 🔨 INI (`.ini`)

### Documentation
- 📘 Markdown (`.md`)
- 📄 Text (`.txt`)
- 📚 RST (`.rst`)
- 📖 AsciiDoc (`.adoc`)

## 🚫 Automatically Ignored

- 📁 Hidden directories (`.git`, `.venv`, etc.)
- 📦 Package directories (`node_modules`, `site-packages`)
- 🏗️ Build directories (`target`, `dist`, `build`)
- 🔒 IDE directories (`.idea`, `.vscode`)
- 📝 Patterns from .gitignore (when --respect-gitignore is used)

## 🛠️ Development

### Requirements

- 🦀 Rust 2021 Edition
- 📦 Cargo

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

## 👤 Author

**Guilherme Baufaker Rêgo**
- 📧 Email: baufaker@protonmail.com
- 🌟 Feel free to report issues or suggest improvements

## 📄 License

MIT License - see the [LICENSE](LICENSE) file for details

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

Made with ❤️ by Guilherme Baufaker Rêgo

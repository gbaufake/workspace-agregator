# Usage Guide

## Basic Command Structure

\```bash
workspace-aggregator <directory> [options]
\```

## Command Line Options

### Output Control

| Option | Description | Example |
|--------|-------------|---------|
| \`--output-dir\` | Set output directory | \`--output-dir ./docs\` |
| \`--output\` | Set specific output file | \`--output workspace=./content.txt\` |
| \`--generate\` | Specify outputs to create | \`--generate workspace,tree\` |
| \`--no-timestamp\` | Disable timestamps | \`--no-timestamp\` |

### Filtering Options

| Option | Description | Example |
|--------|-------------|---------|
| \`--exclude\` | Exclude extensions | \`--exclude js,css,html\` |
| \`--exclude-dir\` | Exclude directories | \`--exclude-dir test,temp\` |
| \`--exclude-pattern\` | Exclude by pattern | \`--exclude-pattern "*.min.*"\` |
| \`--respect-gitignore\` | Use .gitignore rules | \`--respect-gitignore\` |

### Display Options

| Option | Description | Example |
|--------|-------------|---------|
| \`--verbosity\` | Set detail level | \`--verbosity debug\` |
| \`--progress-style\` | Set progress display | \`--progress-style detailed\` |
| \`--quiet\` | Minimal output | \`--quiet\` |
| \`--verbose\` | Detailed output | \`--verbose\` |

## Output Types

### 1. Workspace Content (\`workspace\`)
- Aggregates all file contents
- Includes metadata headers
- Default: \`workspace_YYYYMMDD_HHMMSS.txt\`

### 2. Files List (\`files\`)
- Lists all processed files
- Groups by extension
- Default: \`files_YYYYMMDD_HHMMSS.txt\`

### 3. Directory Tree (\`tree\`)
- Visual directory structure
- Shows file hierarchy
- Default: \`tree_YYYYMMDD_HHMMSS.txt\`

### 4. Statistics (\`stats\`)
- Detailed metrics
- Language breakdown
- Default: \`stats_YYYYMMDD_HHMMSS.txt\`

### 5. Summary (\`summary\`)
- Project overview
- Key metrics
- Default: \`summary_YYYYMMDD_HHMMSS.txt\`

### 6. Metadata (\`meta\`)
- JSON format
- Complete project data
- Default: \`meta_YYYYMMDD_HHMMSS.json\`

## Configuration

### Default Ignored Patterns
- \`.git\`, \`.svn\`
- \`node_modules\`, \`target\`
- \`dist\`, \`build\`
- \`.idea\`, \`.vscode\`

### Environment Variables
- \`WORKSPACE_OUTPUT_DIR\`: Default output directory
- \`WORKSPACE_VERBOSITY\`: Default verbosity level
- \`WORKSPACE_IGNORE_FILE\`: Custom ignore file

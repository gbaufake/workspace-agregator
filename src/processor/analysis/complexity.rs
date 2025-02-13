use crate::processor::types::CodeComplexity;
use std::path::Path;

pub struct ComplexityAnalyzer {
    pub total_complexity: f64,
    pub file_count: usize,
}

impl Default for ComplexityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplexityAnalyzer {
    pub fn new() -> Self {
        Self {
            total_complexity: 0.0,
            file_count: 0,
        }
    }

    pub fn analyze_file(&mut self, content: &str, _path: &Path) -> CodeComplexity {
        let mut complexity = CodeComplexity::default();

        let lines: Vec<&str> = content.lines().collect();
        complexity.lines_of_code = lines.len();

        let mut branch_points = 0;
        let mut comment_lines = 0;

        for line in &lines {
            let trimmed = line.trim();

            // Count comments
            if trimmed.starts_with("//")
                || trimmed.starts_with("#")
                || trimmed.starts_with("/*")
                || trimmed.contains("*/")
            {
                comment_lines += 1;
            }

            // Count branch points
            if trimmed.contains("if ")
                || trimmed.contains("else ")
                || trimmed.contains("match ")
                || trimmed.contains("while ")
                || trimmed.contains("for ")
                || trimmed.contains("&&")
                || trimmed.contains("||")
            {
                branch_points += 1;
            }
        }

        complexity.cyclomatic_complexity = 1.0 + branch_points as f64;
        complexity.comment_ratio = if !lines.is_empty() {
            comment_lines as f64 / lines.len() as f64
        } else {
            0.0
        };

        complexity
    }
}

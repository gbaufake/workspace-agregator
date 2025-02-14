use crate::processor::types::*;
use rayon::prelude::*;

pub struct MetricsAnalyzer {
    metrics: CodeQualityMetrics,
}

impl MetricsAnalyzer {
    pub fn new() -> Self {
        Self {
            metrics: CodeQualityMetrics::default(),
        }
    }

    pub fn analyze_file(&mut self, path: &Path, content: &str) -> io::Result<FileMetrics> {
        let mut metrics = FileMetrics::default();

        // Calculate complexity metrics
        metrics.complexity = self.calculate_complexity(content);

        // Calculate maintainability metrics
        metrics.maintainability = self.calculate_maintainability(content);

        // Calculate documentation metrics
        metrics.documentation = self.analyze_documentation(content);

        // Security scan
        metrics.security = self.security_scan(content);

        Ok(metrics)
    }

    fn calculate_complexity(&self, content: &str) -> ComplexityMetrics {
        ComplexityMetrics {
            cyclomatic: calculate_cyclomatic_complexity(content),
            cognitive: calculate_cognitive_complexity(content),
            halstead: calculate_halstead_metrics(content),
        }
    }

    fn analyze_documentation(&self, content: &str) -> DocumentationMetrics {
        let mut metrics = DocumentationMetrics::default();

        for line in content.lines() {
            if line.trim().starts_with("///") || line.trim().starts_with("//!") {
                metrics.doc_lines += 1;
            }
        }

        metrics.coverage = metrics.doc_lines as f64 / content.lines().count() as f64;
        metrics
    }

    fn security_scan(&self, content: &str) -> SecurityMetrics {
        let mut metrics = SecurityMetrics::default();

        // Scan for common security issues
        metrics.findings = find_security_issues(content);

        // Calculate severity counts
        for finding in &metrics.findings {
            match finding.severity {
                Severity::Critical => metrics.critical += 1,
                Severity::High => metrics.high += 1,
                Severity::Medium => metrics.medium += 1,
                Severity::Low => metrics.low += 1,
            }
        }

        metrics
    }
}

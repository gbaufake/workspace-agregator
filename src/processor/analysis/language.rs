use std::collections::HashMap;
use std::path::Path;

pub struct LanguageDetector {
    language_patterns: HashMap<String, Vec<String>>,
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageDetector {
    pub fn new() -> Self {
        let mut language_patterns = HashMap::new();

        // Rust
        language_patterns.insert(
            "Rust".to_string(),
            vec![
                ".rs".to_string(),
                "fn ".to_string(),
                "impl ".to_string(),
                "pub ".to_string(),
            ],
        );

        // Python
        language_patterns.insert(
            "Python".to_string(),
            vec![
                ".py".to_string(),
                "def ".to_string(),
                "import ".to_string(),
                "class ".to_string(),
            ],
        );

        // JavaScript
        language_patterns.insert(
            "JavaScript".to_string(),
            vec![
                ".js".to_string(),
                "function ".to_string(),
                "const ".to_string(),
                "let ".to_string(),
            ],
        );

        // Add more languages as needed...

        Self { language_patterns }
    }

    pub fn detect_language(&self, path: &Path, content: &str) -> String {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        // First try by extension
        match ext.as_str() {
            "rs" => return "Rust".to_string(),
            "py" => return "Python".to_string(),
            "js" => return "JavaScript".to_string(),
            "java" => return "Java".to_string(),
            "cpp" | "hpp" => return "C++".to_string(),
            "c" | "h" => return "C".to_string(),
            // Add more direct mappings...
            _ => {}
        }

        // If extension is ambiguous, analyze content
        let mut scores = HashMap::new();
        for (language, patterns) in &self.language_patterns {
            let score = patterns
                .iter()
                .filter(|pattern| content.contains(pattern.as_str()))
                .count();
            scores.insert(language, score);
        }

        scores
            .into_iter()
            .max_by_key(|&(_, score)| score)
            .map(|(lang, _)| lang.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

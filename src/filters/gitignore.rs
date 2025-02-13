use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

pub struct GitignoreFilter {
    gitignore: Option<Gitignore>,
    verbose: bool,
}

impl GitignoreFilter {
    pub fn new(dir: &Path, respect_gitignore: bool, verbose: bool) -> Self {
        let gitignore = if respect_gitignore {
            let gitignore_path = dir.join(".gitignore");
            if gitignore_path.exists() {
                let mut builder = GitignoreBuilder::new(dir);

                // The add method returns Option<Error>
                if let Some(err) = builder.add(&gitignore_path) {
                    if verbose {
                        println!("⚠️  Failed to add .gitignore: {}", err);
                    }
                    None
                } else {
                    // Now try to build
                    match builder.build() {
                        Ok(gitignore) => {
                            if verbose {
                                println!("📝 Using .gitignore patterns from: {}", dir.display());
                            }
                            Some(gitignore)
                        }
                        Err(e) => {
                            if verbose {
                                println!("⚠️  Failed to build .gitignore: {}", e);
                            }
                            None
                        }
                    }
                }
            } else {
                if verbose {
                    println!("ℹ️  No .gitignore found in: {}", dir.display());
                }
                None
            }
        } else {
            None
        };

        Self { gitignore, verbose }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        if let Some(ref gitignore) = self.gitignore {
            match gitignore.matched(path, false) {
                ignore::Match::Ignore(_) => {
                    if self.verbose {
                        println!("🚫 Ignored by .gitignore: {}", path.display());
                    }
                    true
                }
                ignore::Match::None => false,
                ignore::Match::Whitelist(_) => false,
            }
        } else {
            false
        }
    }
}

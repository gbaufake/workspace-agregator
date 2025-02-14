pub struct CrossReferenceGenerator {
    references: HashMap<String, Vec<Reference>>,
    type_references: HashMap<String, Vec<TypeReference>>,
}

impl CrossReferenceGenerator {
    pub fn new() -> Self {
        Self {
            references: HashMap::new(),
            type_references: HashMap::new(),
        }
    }

    pub fn generate(&mut self, stats: &EnhancedFileStats) -> io::Result<CrossReferences> {
        let mut cross_refs = CrossReferences::default();

        // Build function references
        for (path, file_stats) in &stats.file_statistics {
            self.analyze_function_references(path, file_stats, &mut cross_refs)?;
        }

        // Build type references
        self.analyze_type_references(&stats.symbols, &mut cross_refs)?;

        Ok(cross_refs)
    }

    fn analyze_function_references(
        &self,
        path: &Path,
        stats: &FileStatistics,
        cross_refs: &mut CrossReferences,
    ) -> io::Result<()> {
        for symbol in &stats.symbols {
            if let Some(refs) = self.references.get(&symbol.name) {
                cross_refs.function_references.insert(
                    symbol.name.clone(),
                    refs.clone(),
                );
            }
        }
        Ok(())
    }

    fn analyze_type_references(
        &self,
        symbols: &[CodeSymbols],
        cross_refs: &mut CrossReferences,
    ) -> io::Result<()> {
        for symbol in symbols {
            for struct_sym in &symbol.structs {
                if let Some(refs) = self.type_references.get(&struct_sym.name) {
                    cross_refs.type_references.insert(
                        struct_sym.name.clone(),
                        refs.clone(),
                    );
                }
            }
        }
        Ok(())
    }
}

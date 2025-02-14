use std::collections::{HashMap, HashSet};
use syn::{self, visit::{self, Visit}};
use petgraph::Graph;
use petgraph::dot::{Dot, Config};

pub struct DependencyAnalyzer {
    graph: Graph<String, ()>,
    function_calls: HashMap<String, Vec<String>>,
    imports: HashMap<String, HashSet<String>>,
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            function_calls: HashMap::new(),
            imports: HashMap::new(),
        }
    }

    pub fn analyze_file(&mut self, path: &Path, content: &str) -> io::Result<CodeSymbols> {
        let syntax = syn::parse_file(content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut visitor = RustVisitor::new(path);
        visitor.visit_file(&syntax);

        Ok(CodeSymbols {
            file: path.to_string_lossy().to_string(),
            structs: visitor.get_structs(),
            imports: visitor.get_imports(),
        })
    }

    pub fn build_call_graph(&mut self) -> FunctionCallGraphs {
        let mut graphs = FunctionCallGraphs {
            function_calls: HashMap::new(),
        };

        for (caller, callees) in &self.function_calls {
            graphs.function_calls.insert(caller.clone(), callees.clone());
        }

        graphs
    }

    pub fn export_graph(&self, output_path: &Path) -> io::Result<()> {
        let dot = Dot::with_config(&self.graph, &[Config::EdgeNoLabel]);
        fs::write(output_path, format!("{:?}", dot))?;
        Ok(())
    }
}

struct RustVisitor {
    path: PathBuf,
    current_struct: Option<String>,
    structs: Vec<StructSymbol>,
    imports: Vec<Import>,
}

impl<'ast> Visit<'ast> for RustVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.current_struct = Some(i.ident.to_string());
        // Continue visiting struct contents
        visit::visit_item_struct(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if let Some(ref struct_name) = self.current_struct {
            let method = MethodSymbol {
                name: i.sig.ident.to_string(),
                line: i.span().start().line,
                complexity: calculate_complexity(&i.block),
                params: extract_params(&i.sig),
                return_type: extract_return_type(&i.sig),
                called_by: Vec::new(),
                calls: extract_function_calls(&i.block),
            };

            if let Some(struct_symbol) = self.structs.iter_mut()
                .find(|s| s.name == *struct_name) {
                struct_symbol.methods.push(method);
            }
        }
    }

    fn visit_use_tree(&mut self, i: &'ast syn::UseTree) {
        let import = parse_use_tree(i);
        self.imports.push(import);
    }
}

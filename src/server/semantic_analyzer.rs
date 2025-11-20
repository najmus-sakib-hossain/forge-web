//! Semantic Analysis Module
//!
//! Uses tree-sitter to analyze code structure, detect symbols, and provide semantic information

use anyhow::Result;
use std::collections::HashMap;
use tree_sitter::{Parser, Node};
use std::path::Path;



/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range,
    pub children: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Impl,
    Mod,
    Const,
    Static,
    Trait,
    Type,
    Variable,
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

/// Semantic analyzer for Rust code
pub struct SemanticAnalyzer {
    parser: Parser,
    symbol_table: HashMap<String, Vec<Symbol>>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        let language = tree_sitter_rust::LANGUAGE.into();
        parser.set_language(&language)?;

        Ok(Self {
            parser,
            symbol_table: HashMap::new(),
        })
    }

    /// Parse and analyze a file
    pub fn analyze_file(&mut self, file_path: &Path, source: &str) -> Result<Vec<Symbol>> {
        let tree = self.parser.parse(source, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse file"))?;

        let root = tree.root_node();
        let symbols = self.extract_symbols(root, source)?;

        // Store in symbol table
        self.symbol_table.insert(
            file_path.to_string_lossy().to_string(),
            symbols.clone()
        );

        Ok(symbols)
    }

    /// Extract symbols from AST node
    fn extract_symbols(&self, node: Node, source: &str) -> Result<Vec<Symbol>> {
        let mut symbols = Vec::new();
        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            if let Some(symbol) = self.node_to_symbol(child, source)? {
                symbols.push(symbol);
            }
        }

        Ok(symbols)
    }

    /// Convert tree-sitter node to symbol
    fn node_to_symbol(&self, node: Node, source: &str) -> Result<Option<Symbol>> {
        let kind_str = node.kind();
        
        let kind = match kind_str {
            "function_item" => SymbolKind::Function,
            "struct_item" => SymbolKind::Struct,
            "enum_item" => SymbolKind::Enum,
            "impl_item" => SymbolKind::Impl,
            "mod_item" => SymbolKind::Mod,
            "const_item" => SymbolKind::Const,
            "static_item" => SymbolKind::Static,
            "trait_item" => SymbolKind::Trait,
            "type_item" => SymbolKind::Type,
            _ => return Ok(None),
        };

        // Extract name
        let name = self.extract_name(node, source)?;
        
        // Create range
        let range = Range {
            start_line: node.start_position().row + 1,
            start_col: node.start_position().column,
            end_line: node.end_position().row + 1,
            end_col: node.end_position().column,
        };

        // Extract children symbols
        let children = self.extract_symbols(node, source)?;

        Ok(Some(Symbol {
            name,
            kind,
            range,
            children,
        }))
    }

    /// Extract name from node
    fn extract_name(&self, node: Node, source: &str) -> Result<String> {
        // Find identifier child node
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                let start = child.start_byte();
                let end = child.end_byte();
                return Ok(source[start..end].to_string());
            }
        }
        Ok("(anonymous)".to_string())
    }

    /// Find symbol at position
    pub fn find_symbol_at_position(
        &self,
        file_path: &Path,
        line: usize,
        column: usize
    ) -> Option<&Symbol> {
        let symbols = self.symbol_table.get(&file_path.to_string_lossy().to_string())?;
        self.find_symbol_in_tree(symbols, line, column)
    }

    /// Recursively search symbol tree
    fn find_symbol_in_tree<'a>(
        &self,
        symbols: &'a [Symbol],
        line: usize,
        column: usize
    ) -> Option<&'a Symbol> {
        for symbol in symbols {
            if self.contains_position(&symbol.range, line, column) {
                // Check children first (more specific)
                if let Some(child) = self.find_symbol_in_tree(&symbol.children, line, column) {
                    return Some(child);
                }
                return Some(symbol);
            }
        }
        None
    }

    /// Check if range contains position
    fn contains_position(&self, range: &Range, line: usize, column: usize) -> bool {
        if line < range.start_line || line > range.end_line {
            return false;
        }
        if line == range.start_line && column < range.start_col {
            return false;
        }
        if line == range.end_line && column > range.end_col {
            return false;
        }
        true
    }

    /// Get all symbols in file
    pub fn get_symbols(&self, file_path: &Path) -> Option<&Vec<Symbol>> {
        self.symbol_table.get(&file_path.to_string_lossy().to_string())
    }

    /// Detect DX component patterns using tree-sitter
    pub fn detect_dx_patterns(&mut self, source: &str) -> Result<Vec<DxPattern>> {
        let tree = self.parser.parse(source, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse source"))?;

        let mut patterns = Vec::new();
        let root = tree.root_node();

        // Query for JSX/TSX elements that match dx* pattern
        self.find_dx_elements(root, source, &mut patterns)?;

        Ok(patterns)
    }

    /// Recursively find DX element patterns
    fn find_dx_elements(
        &self,
        node: Node,
        source: &str,
        patterns: &mut Vec<DxPattern>
    ) -> Result<()> {
        // Check if this node is a JSX element starting with "dx"
        if node.kind() == "jsx_element" || node.kind() == "jsx_self_closing_element" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "jsx_opening_element" || child.kind() == "identifier" {
                    let start = child.start_byte();
                    let end = child.end_byte();
                    let text = &source[start..end];
                    
                    if text.starts_with("dx") || text.contains("<dx") {
                        let component_name = text
                            .trim_start_matches('<')
                            .split(|c: char| c.is_whitespace() || c == '>')
                            .next()
                            .unwrap_or("")
                            .to_string();

                        if component_name.starts_with("dx") {
                            patterns.push(DxPattern {
                                component_name,
                                line: node.start_position().row + 1,
                                col: node.start_position().column,
                            });
                        }
                    }
                }
            }
        }

        // Recurse through children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.find_dx_elements(child, source, patterns)?;
        }

        Ok(())
    }
}

/// DX component pattern match
#[derive(Debug, Clone)]
pub struct DxPattern {
    pub component_name: String,
    pub line: usize,
    pub col: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = SemanticAnalyzer::new();
        assert!(analyzer.is_ok());
    }

    #[test]
    fn test_rust_parsing() {
        let mut analyzer = SemanticAnalyzer::new().unwrap();
        let source = r#"
            fn main() {
                println!("Hello");
            }
            
            struct MyStruct {
                field: i32,
            }
        "#;

        let path = Path::new("test.rs");
        let symbols = analyzer.analyze_file(path, source).unwrap();
        
        assert!(!symbols.is_empty());
        assert!(symbols.iter().any(|s| s.kind == SymbolKind::Function));
        assert!(symbols.iter().any(|s| s.kind == SymbolKind::Struct));
    }

    #[test]
    fn test_nested_symbols() {
        let mut analyzer = SemanticAnalyzer::new().unwrap();
        let source = r#"
            mod my_mod {
                struct Inner {
                    x: i32
                }
                
                impl Inner {
                    fn new() -> Self { Self { x: 0 } }
                }
            }
        "#;

        let path = Path::new("nested.rs");
        let symbols = analyzer.analyze_file(path, source).unwrap();
        
        let mod_symbol = symbols.iter().find(|s| s.kind == SymbolKind::Mod).unwrap();
        assert_eq!(mod_symbol.name, "my_mod");
        assert!(!mod_symbol.children.is_empty());
        
        let struct_symbol = mod_symbol.children.iter().find(|s| s.kind == SymbolKind::Struct).unwrap();
        assert_eq!(struct_symbol.name, "Inner");
    }

    #[test]
    fn test_find_symbol_at_position() {
        let mut analyzer = SemanticAnalyzer::new().unwrap();
        let source = r#"
            fn target_function() {
                // code
            }
        "#;
        
        let path = Path::new("lookup.rs");
        analyzer.analyze_file(path, source).unwrap();
        
        // Line 2 (1-indexed), column 15 should be inside the function
        let symbol = analyzer.find_symbol_at_position(path, 2, 15);
        assert!(symbol.is_some());
        assert_eq!(symbol.unwrap().name, "target_function");
        
        // Line 10 should be None
        let symbol = analyzer.find_symbol_at_position(path, 10, 0);
        assert!(symbol.is_none());
    }
}

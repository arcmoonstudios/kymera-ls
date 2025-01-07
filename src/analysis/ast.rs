//! src/analysis/ast.rs
//! LSP-specific AST representation for IDE features.

use std::sync::Arc;
use tower_lsp::lsp_types::{Position, Range};

/// LSP-aware Abstract Syntax Tree
#[derive(Debug, Clone)]
pub struct LspAst {
    root: Arc<LspNode>,
    uri: String,
}

impl LspAst {
    /// Creates a new LSP-aware AST
    pub fn new(root: Arc<LspNode>, uri: String) -> Self {
        Self { root, uri }
    }

    /// Returns the root node
    pub fn root(&self) -> &Arc<LspNode> {
        &self.root
    }

    /// Returns the document URI
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

/// LSP-specific node with location information
#[derive(Debug, Clone)]
pub struct LspNode {
    /// The kind of node
    pub kind: LspNodeKind,
    /// Location in the document
    pub range: Range,
    /// Selection range for the node
    pub selection_range: Range,
    /// Child nodes
    pub children: Vec<Arc<LspNode>>,
}

/// LSP-specific node types optimized for IDE features
#[derive(Debug, Clone)]
pub enum LspNodeKind {
    // Document Structure
    Document {
        imports: Vec<Arc<LspNode>>,
        declarations: Vec<Arc<LspNode>>,
    },
    Import {
        path: String,
        alias: Option<String>,
        is_used: bool,
    },

    // Declarations with IDE metadata
    Function {
        name: String,
        params: Vec<Arc<LspNode>>,
        return_type: Option<String>,
        is_public: bool,
        is_async: bool,
        doc_comment: Option<String>,
        references: Vec<Range>,
    },
    Variable {
        name: String,
        type_info: Option<String>,
        is_mutable: bool,
        is_used: bool,
        references: Vec<Range>,
    },
    Struct {
        name: String,
        fields: Vec<Arc<LspNode>>,
        is_public: bool,
        implementations: Vec<Range>,
    },
    Enum {
        name: String,
        variants: Vec<String>,
        is_public: bool,
        usages: Vec<Range>,
    },

    // IDE-specific nodes
    Reference {
        name: String,
        definition_range: Option<Range>,
        is_definition: bool,
    },
    Completion {
        text: String,
        kind: CompletionKind,
        detail: Option<String>,
    },
    Diagnostic {
        severity: DiagnosticSeverity,
        message: String,
        related_ranges: Vec<Range>,
    },
    Hover {
        content: String,
        type_info: Option<String>,
    },

    // Code Actions
    CodeAction {
        title: String,
        kind: CodeActionKind,
        edits: Vec<TextEdit>,
    },
}

/// Types of code completions
#[derive(Debug, Clone)]
pub enum CompletionKind {
    Function,
    Variable,
    Type,
    Keyword,
    Module,
    Field,
    Value,
}

/// Diagnostic severity levels
#[derive(Debug, Clone)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// Types of code actions
#[derive(Debug, Clone)]
pub enum CodeActionKind {
    QuickFix,
    Refactor,
    RefactorExtract,
    RefactorInline,
    RefactorRewrite,
}

/// Text edit operation
#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

/// LSP-specific visitor for IDE features
pub trait LspAstVisitor {
    fn visit_node(&mut self, node: &Arc<LspNode>, uri: &str);
    
    fn visit_children(&mut self, node: &Arc<LspNode>, uri: &str) {
        for child in &node.children {
            self.visit_node(child, uri);
        }
    }
}

/// Utility functions for LSP operations
impl LspNode {
    /// Creates a new LSP node
    pub fn new(kind: LspNodeKind, range: Range) -> Self {
        Self {
            kind,
            range,
            selection_range: range,
            children: Vec::new(),
        }
    }

    /// Checks if a position is within this node's range
    pub fn contains(&self, position: Position) -> bool {
        position >= self.range.start && position <= self.range.end
    }

    /// Gets the deepest node containing the position
    pub fn node_at_position(&self, position: Position) -> Option<&LspNode> {
        if !self.contains(position) {
            return None;
        }

        for child in &self.children {
            if let Some(node) = child.node_at_position(position) {
                return Some(node);
            }
        }

        Some(self)
    }

    /// Gets completion items at a position
    pub fn completions_at_position(&self, position: Position) -> Vec<&LspNode> {
        let mut completions = Vec::new();
        if !self.contains(position) {
            return completions;
        }

        if let LspNodeKind::Completion { .. } = self.kind {
            completions.push(self);
        }

        for child in &self.children {
            completions.extend(child.completions_at_position(position));
        }

        completions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_ast_creation() {
        let node = Arc::new(LspNode::new(
            LspNodeKind::Function {
                name: "test".to_string(),
                params: vec![],
                return_type: None,
                is_public: true,
                is_async: false,
                doc_comment: None,
                references: vec![],
            },
            Range::default(),
        ));

        let ast = LspAst::new(node, "file:///test.ky".to_string());
        assert!(matches!(
            ast.root().kind,
            LspNodeKind::Function { .. }
        ));
    }

    #[test]
    fn test_position_queries() {
        let mut node = LspNode::new(
            LspNodeKind::Document {
                imports: vec![],
                declarations: vec![],
            },
            Range {
                start: Position::new(0, 0),
                end: Position::new(10, 0),
            },
        );

        let child = Arc::new(LspNode::new(
            LspNodeKind::Function {
                name: "test".to_string(),
                params: vec![],
                return_type: None,
                is_public: true,
                is_async: false,
                doc_comment: None,
                references: vec![],
            },
            Range {
                start: Position::new(1, 0),
                end: Position::new(2, 0),
            },
        ));

        node.children.push(child);

        assert!(node.contains(Position::new(1, 0)));
        assert!(node.node_at_position(Position::new(1, 0)).is_some());
    }
}
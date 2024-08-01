use crate::{agentic::symbol::identifier::SymbolIdentifier, chunking::text_document::Range};

use super::initial_request::SymbolRequestHistoryItem;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SymbolToEdit {
    outline: bool,
    range: Range,
    fs_file_path: String,
    symbol_name: String,
    instructions: Vec<String>,
    is_new: bool,
    // If this is a full symbol edit instead of being sub-symbol level
    is_full_edit: bool,
    original_user_query: String,
}

impl SymbolToEdit {
    pub fn new(
        symbol_name: String,
        range: Range,
        fs_file_path: String,
        instructions: Vec<String>,
        outline: bool,
        is_new: bool,
        is_full_edit: bool,
        original_user_query: String,
    ) -> Self {
        Self {
            symbol_name,
            range,
            outline,
            fs_file_path,
            instructions,
            is_new,
            is_full_edit,
            original_user_query,
        }
    }

    pub fn original_user_query(&self) -> &str {
        &self.original_user_query
    }

    pub fn is_full_edit(&self) -> bool {
        self.is_full_edit
    }

    pub fn set_fs_file_path(&mut self, fs_file_path: String) {
        self.fs_file_path = fs_file_path;
    }

    pub fn set_range(&mut self, range: Range) {
        self.range = range;
    }

    pub fn is_new(&self) -> bool {
        self.is_new.clone()
    }

    pub fn range(&self) -> &Range {
        &self.range
    }

    pub fn is_outline(&self) -> bool {
        self.outline
    }

    pub fn symbol_name(&self) -> &str {
        &self.symbol_name
    }

    pub fn instructions(&self) -> &[String] {
        self.instructions.as_slice()
    }

    pub fn fs_file_path(&self) -> &str {
        &self.fs_file_path
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SymbolToEditRequest {
    symbols: Vec<SymbolToEdit>,
    symbol_identifier: SymbolIdentifier,
    history: Vec<SymbolRequestHistoryItem>,
}

impl SymbolToEditRequest {
    pub fn new(
        symbols: Vec<SymbolToEdit>,
        identifier: SymbolIdentifier,
        history: Vec<SymbolRequestHistoryItem>,
    ) -> Self {
        Self {
            symbol_identifier: identifier,
            symbols,
            history,
        }
    }

    pub fn symbols(self) -> Vec<SymbolToEdit> {
        self.symbols
    }

    pub fn symbol_identifier(&self) -> &SymbolIdentifier {
        &self.symbol_identifier
    }

    pub fn history(&self) -> &[SymbolRequestHistoryItem] {
        self.history.as_slice()
    }
}

//! Contains the different datastructures which are necessary for the initial
//! request

#[derive(Debug, Clone, serde::Serialize)]
pub struct SymbolEditedItem {
    symbol: String,
    fs_file_path: String,
    is_new: bool,
    thinking: String,
}

impl SymbolEditedItem {
    pub fn new(symbol: String, fs_file_path: String, is_new: bool, thinking: String) -> Self {
        Self {
            symbol,
            fs_file_path,
            is_new,
            thinking,
        }
    }

    pub fn name(&self) -> &str {
        &self.symbol
    }

    pub fn fs_file_path(&self) -> &str {
        &self.fs_file_path
    }

    pub fn is_new(&self) -> bool {
        self.is_new
    }

    pub fn thinking(&self) -> &str {
        &self.thinking
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SymbolRequestHistoryItem {
    symbol: String,
    fs_file_path: String,
    request: String,
}

impl SymbolRequestHistoryItem {
    pub fn new(symbol: String, fs_file_path: String, request: String) -> Self {
        Self {
            symbol,
            fs_file_path,
            request,
        }
    }

    pub fn symbol_name(&self) -> &str {
        &self.symbol
    }

    pub fn fs_file_path(&self) -> &str {
        &self.fs_file_path
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct InitialRequestData {
    original_question: String,
    plan_if_available: Option<String>,
    history: Vec<SymbolRequestHistoryItem>,
    /// We operate on the full symbol instead of the
    full_symbol_request: bool,
    // This is an option for now since we for code-correctness we also send
    // this request, but this is more tied to the original plan
    // in the future this will be a reference to some plan object which will
    // dynamically update the symbol edited items inside
    symbols_edited_list: Option<Vec<SymbolEditedItem>>,
}

impl InitialRequestData {
    pub fn new(
        original_question: String,
        plan_if_available: Option<String>,
        history: Vec<SymbolRequestHistoryItem>,
        full_symbol_request: bool,
        symbols_edited_list: Option<Vec<SymbolEditedItem>>,
    ) -> Self {
        Self {
            original_question,
            plan_if_available,
            history,
            full_symbol_request,
            symbols_edited_list,
        }
    }

    pub fn full_symbol_request(&self) -> bool {
        self.full_symbol_request
    }

    pub fn get_original_question(&self) -> &str {
        &self.original_question
    }

    pub fn get_plan(&self) -> Option<String> {
        self.plan_if_available.clone()
    }

    pub fn history(&self) -> &[SymbolRequestHistoryItem] {
        self.history.as_slice()
    }

    pub fn symbols_edited_list(&self) -> Option<&[SymbolEditedItem]> {
        self.symbols_edited_list
            .as_ref()
            .map(|symbol_list| symbol_list.as_slice())
    }
}

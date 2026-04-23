use oxc_ast::ast::{Expression, Statement};

use super::memo::MemoAttr;
use super::template::Template;

#[derive(Default)]
pub(crate) struct EmitState<'a> {
    pub template: Template,
    pub init: Vec<Statement<'a>>,
    pub update: Vec<Statement<'a>>,
    pub after_update: Vec<Statement<'a>>,
    pub bound_contenteditable: bool,
    pub root_var: Option<String>,
    pub special_elements: Vec<Statement<'a>>,
    pub memo_attrs: Vec<MemoAttr<'a>>,
    pub script_blockers: Vec<u32>,
    pub extra_blockers: Vec<Expression<'a>>,
    pub local_snippet_decls: Option<Vec<Statement<'a>>>,
    pub skip_snippets: bool,
    pub last_fragment_needs_reset: bool,
    pub pending_anchor_idents: Option<(String, String)>,
    pub suppress_root_finalize: bool,
}

impl<'a> EmitState<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

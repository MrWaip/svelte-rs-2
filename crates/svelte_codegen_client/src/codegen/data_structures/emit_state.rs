use oxc_ast::ast::{Expression, Statement};

use super::memo::TemplateMemoState;
use super::template::Template;

#[derive(Default)]
pub(crate) struct EmitState<'a> {
    pub template: Template,
    pub init: Vec<Statement<'a>>,
    pub update: Vec<Statement<'a>>,
    pub after_update: Vec<Statement<'a>>,
    pub element_after_update: Vec<Statement<'a>>,
    pub bound_contenteditable: bool,
    pub root_var: Option<String>,
    pub special_elements: Vec<Statement<'a>>,
    pub shared_memo: TemplateMemoState<'a>,
    pub script_blockers: Vec<u32>,
    pub extra_blockers: Vec<Expression<'a>>,
    pub local_snippet_decls: Option<Vec<Statement<'a>>>,
    pub skip_snippets: bool,
    pub last_fragment_needs_reset: bool,
    pub pending_element_init: Vec<Statement<'a>>,
    pub pending_pre_update: Vec<Statement<'a>>,
    pub pending_anchor_idents: Option<(String, String)>,
    pub anchor_comment_pre_emitted: bool,
    pub suppress_root_finalize: bool,
    pub legacy_slot_record_const_tag_end: bool,
    pub legacy_slot_const_tag_end: Option<usize>,
    pub legacy_slot_const_tag_start: Option<usize>,
}

impl<'a> EmitState<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

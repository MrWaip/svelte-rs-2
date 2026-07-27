use oxc_ast::ast::{BindingPattern, Statement};
use svelte_ast::SnippetBlock;
use svelte_component_semantics::SymbolId;

use crate::types::data::JsAst;

pub(crate) fn snippet_name_symbol(parsed: &JsAst<'_>, block: &SnippetBlock) -> Option<SymbolId> {
    let Statement::VariableDeclaration(decl) = parsed.stmt(block.decl.id())? else {
        return None;
    };
    let BindingPattern::BindingIdentifier(ident) = &decl.declarations.first()?.id else {
        return None;
    };
    ident.symbol_id.get()
}

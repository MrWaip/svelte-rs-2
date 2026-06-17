use oxc_ast::ast::Expression;
use svelte_analyze::AnalysisData;
use svelte_ast_builder::Builder;
use svelte_component_semantics::SymbolId;

use crate::runes::rune_get;
use crate::runtime::thunk_call;

pub fn each_item_collection_read_legacy<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'_>,
    source_sym: SymbolId,
    hoisted_collection_name: Option<&str>,
) -> Expression<'a> {
    if let Some(name) = hoisted_collection_name {
        return thunk_call(b, name);
    }
    let name = analysis.scoping.symbol_name(source_sym);
    let semantics = analysis.binding_semantics(source_sym);
    if semantics.reads_via_each_item_accessor() || semantics.reads_via_thunk() {
        return thunk_call(b, name);
    }
    if semantics.is_non_reactive() {
        return b.rid_expr(name);
    }
    rune_get(b, name)
}

pub fn each_item_indexed_member_legacy<'a>(
    b: &Builder<'a>,
    collection: Expression<'a>,
    index_name: &str,
) -> Expression<'a> {
    b.computed_member_expr(collection, b.rid_expr(index_name))
}

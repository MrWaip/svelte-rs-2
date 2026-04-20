//! Pre-semantic pattern-name collector.
//!
//! Used only by passes that run BEFORE `ComponentSemantics` is built
//! (`script_info::extract_script_info`), where `BindingIdentifier.symbol_id`
//! is not yet populated and the shared `walk_bindings` walker would panic.
//!
//! Post-semantic code (analyze passes that run after `ComponentSemantics`,
//! transform, codegen) MUST use `svelte_component_semantics::walk_bindings`
//! — it yields `SymbolId` directly and carries path / default / rest info.

use oxc_ast::ast::BindingPattern;

/// Collect all binding identifier names from a BindingPattern.
pub(crate) fn collect_binding_names(pat: &BindingPattern<'_>, out: &mut Vec<String>) {
    match pat {
        BindingPattern::BindingIdentifier(id) => out.push(id.name.to_string()),
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_binding_names(&prop.value, out);
            }
            if let Some(rest) = &obj.rest {
                collect_binding_names(&rest.argument, out);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() {
                collect_binding_names(el, out);
            }
            if let Some(rest) = &arr.rest {
                collect_binding_names(&rest.argument, out);
            }
        }
        BindingPattern::AssignmentPattern(assign) => collect_binding_names(&assign.left, out),
    }
}

use svelte_ast::{Attribute, BindDirective, Component, Element, ExpressionTag};
use oxc_semantic::ScopeId;

use crate::data::AnalysisData;
use crate::walker::{self, TemplateVisitor};

/// Detect which rune symbols are mutated: assigned in script (already tracked by OXC semantic),
/// template expression assignments, or bound via bind directives.
/// After detection, caches the rune name sets on AnalysisData.
pub fn detect_mutations(component: &Component, data: &mut AnalysisData) {
    // Script mutations are already tracked by OXC's SemanticBuilder
    // (symbol_is_mutated() returns true for symbols with write references).

    let root_scope = data.scoping.root_scope_id();

    // Single walk: template expression assignments + bind directive mutations
    let mut visitor = (
        TemplateMutationVisitor,
        BindMutationVisitor { component },
    );
    walker::walk_template(&component.fragment, data, root_scope, &mut visitor);

    // Cache the computed rune name sets
    data.cache_rune_sets();
}

/// Detects write/read-write references to runes in template expressions.
struct TemplateMutationVisitor;

impl TemplateVisitor for TemplateMutationVisitor {
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {
        if let Some(info) = data.expressions.get(&tag.id) {
            for r in &info.references {
                if r.flags == svelte_js::ReferenceFlags::Write
                    || r.flags == svelte_js::ReferenceFlags::ReadWrite
                {
                    if let Some(sym_id) = data.scoping.find_binding(scope, &r.name) {
                        if data.scoping.is_rune(sym_id) {
                            data.scoping.mark_template_mutated(sym_id);
                        }
                    }
                }
            }
        }
    }
}

/// Detects rune mutations via bind directives.
struct BindMutationVisitor<'a> {
    component: &'a Component,
}

impl TemplateVisitor for BindMutationVisitor<'_> {
    fn visit_bind_directive(&mut self, dir: &BindDirective, _el: &Element, scope: ScopeId, data: &mut AnalysisData) {
        let name = if dir.shorthand {
            dir.name.clone()
        } else if let Some(span) = dir.expression_span {
            self.component.source_text(span).trim().to_string()
        } else {
            return;
        };
        if let Some(sym_id) = data.scoping.find_binding(scope, &name) {
            if data.scoping.is_rune(sym_id) {
                data.scoping.mark_bind_mutated(sym_id);
                data.scoping.mark_template_mutated(sym_id);
            }
        }
    }
}

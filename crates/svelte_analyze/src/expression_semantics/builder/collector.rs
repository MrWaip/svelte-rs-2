use crate::reactivity_semantics::data::{ReactivitySemantics, ReferenceSemantics};
use crate::types::data::PickledAwaitOffsets;
use oxc_ast::ast::{AwaitExpression, CallExpression, ChainElement, Expression, IdentifierReference};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{walk_await_expression, walk_call_expression};
use smallvec::SmallVec;
use svelte_component_semantics::{ComponentSemantics, SymbolId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TopLevelShape {
    Member,
    Assignment,
    Update,
    Other,
}

pub(super) struct ExprFacts {
    pub references: SmallVec<[SymbolId; 2]>,
    pub has_await: bool,
    pub has_call: bool,
    pub uses_legacy_sanitized_props: bool,
    pub any_pickled_await: bool,
    pub top_level_shape: TopLevelShape,
}

pub(super) fn collect<'a>(
    expr: &Expression<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    pickled: &PickledAwaitOffsets,
) -> ExprFacts {
    let top_level_shape = top_level_shape_of(expr);
    let mut visitor = Collector {
        semantics,
        reactivity,
        pickled,
        references: SmallVec::new(),
        has_await: false,
        has_call: false,
        uses_legacy_sanitized_props: false,
        any_pickled_await: false,
    };
    visitor.visit_expression(expr);
    ExprFacts {
        references: visitor.references,
        has_await: visitor.has_await,
        has_call: visitor.has_call,
        uses_legacy_sanitized_props: visitor.uses_legacy_sanitized_props,
        any_pickled_await: visitor.any_pickled_await,
        top_level_shape,
    }
}

fn top_level_shape_of(expr: &Expression<'_>) -> TopLevelShape {
    match expr {
        Expression::StaticMemberExpression(_)
        | Expression::ComputedMemberExpression(_)
        | Expression::PrivateFieldExpression(_) => TopLevelShape::Member,
        Expression::AssignmentExpression(_) => TopLevelShape::Assignment,
        Expression::UpdateExpression(_) => TopLevelShape::Update,
        Expression::ChainExpression(chain) => match &chain.expression {
            ChainElement::CallExpression(_) => TopLevelShape::Other,
            _ => TopLevelShape::Member,
        },
        _ => TopLevelShape::Other,
    }
}

struct Collector<'c, 'a> {
    semantics: &'c ComponentSemantics<'a>,
    reactivity: &'c ReactivitySemantics,
    pickled: &'c PickledAwaitOffsets,
    references: SmallVec<[SymbolId; 2]>,
    has_await: bool,
    has_call: bool,
    uses_legacy_sanitized_props: bool,
    any_pickled_await: bool,
}

impl<'a> Visit<'a> for Collector<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let name = ident.name.as_str();
        if name == "$$props" || name == "$$restProps" {
            self.uses_legacy_sanitized_props = true;
        }
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        let sym = match self.reactivity.reference_semantics(ref_id) {
            ReferenceSemantics::StoreRead { symbol }
            | ReferenceSemantics::StoreWrite { symbol }
            | ReferenceSemantics::StoreUpdate { symbol } => Some(symbol),
            ReferenceSemantics::LegacyStateSubscribedRead { store_symbol, .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { store_symbol }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { store_symbol, .. }
            | ReferenceSemantics::ImportSubscribedRead { store_symbol } => Some(store_symbol),
            _ => self.semantics.get_reference(ref_id).symbol_id(),
        };
        let Some(sym) = sym else { return };
        if !self.references.contains(&sym) {
            self.references.push(sym);
        }
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        self.has_await = true;
        if self.pickled.contains_offset(expr.span.start) {
            self.any_pickled_await = true;
        }
        walk_await_expression(self, expr);
    }

    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        self.has_call = true;
        walk_call_expression(self, expr);
    }
}


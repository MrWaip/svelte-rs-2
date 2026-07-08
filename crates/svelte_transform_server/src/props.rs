use std::collections::HashMap;
use std::mem;

use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, BindingPattern, BindingProperty, Expression, PropertyKey, VariableDeclaration,
    VariableDeclarationKind, VariableDeclarator,
};
use oxc_span::SPAN;
use svelte_analyze::DeclaratorGroup;
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::{Access, Step, SymbolId, walk_bindings};
use svelte_emit_builders::binding_pattern as bp;
use svelte_emit_builders::props::unwrap_paren_and_ts;
use svelte_emit_builders::runtime::is_simple_expression;

use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_rune_props(&mut self, declarator: &mut VariableDeclarator<'a>) {
        self.erase_bindable(&mut declarator.id);

        match &mut declarator.id {
            BindingPattern::ObjectPattern(obj) => {
                if obj.rest.is_some() {
                    let slots = self.shorthand_binding_property("$$slots");
                    let events = self.shorthand_binding_property("$$events");
                    obj.properties.push(slots);
                    obj.properties.push(events);
                }
            }
            BindingPattern::BindingIdentifier(id) => {
                let rest_pattern = self.b.ast.binding_pattern_binding_identifier(SPAN, id.name);
                let rest = self.b.ast.binding_rest_element(SPAN, rest_pattern);
                let mut properties = self.b.ast.vec();
                properties.push(self.shorthand_binding_property("$$slots"));
                properties.push(self.shorthand_binding_property("$$events"));
                let object =
                    self.b
                        .ast
                        .object_pattern(SPAN, properties, Some(self.b.ast.alloc(rest)));
                declarator.id = BindingPattern::ObjectPattern(self.b.ast.alloc(object));
            }
            _ => {}
        }

        declarator.init = Some(self.b.rid_expr("$$props"));
    }

    fn shorthand_binding_property(&self, name: &str) -> BindingProperty<'a> {
        let atom = self.b.ast.atom(name);
        let key =
            PropertyKey::StaticIdentifier(self.b.alloc(self.b.ast.identifier_name(SPAN, atom)));
        let value = self.b.ast.binding_pattern_binding_identifier(SPAN, atom);
        self.b.ast.binding_property(SPAN, key, value, true, false)
    }

    fn erase_bindable(&self, pattern: &mut BindingPattern<'a>) {
        let BindingPattern::ObjectPattern(obj) = pattern else {
            return;
        };
        for prop in obj.properties.iter_mut() {
            let BindingPattern::AssignmentPattern(assign) = &mut prop.value else {
                continue;
            };
            let BindingPattern::BindingIdentifier(id) = &assign.left else {
                continue;
            };
            let Some(symbol) = id.symbol_id.get() else {
                continue;
            };
            if !self.analysis.binding_semantics(symbol).is_bindable() {
                continue;
            }
            self.unwrap_bindable_default(&mut assign.right);
        }
    }

    fn unwrap_bindable_default(&self, right: &mut Expression<'a>) {
        let replaced = mem::replace(right, self.b.void_zero_expr());
        let Expression::CallExpression(call) = replaced.into_inner_expression() else {
            return;
        };
        let mut call = call.unbox();
        let Some(arg) = call.arguments.first_mut() else {
            return;
        };
        if matches!(arg, Argument::SpreadElement(_)) {
            return;
        }
        let mut taken = Argument::from(self.b.cheap_expr());
        mem::swap(arg, &mut taken);
        *right = taken.into_expression();
    }
}

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_legacy_prop(&mut self, declarator: &mut VariableDeclarator<'a>) {
        let BindingPattern::BindingIdentifier(id) = &declarator.id else {
            return;
        };
        let Some(symbol) = id.symbol_id.get() else {
            return;
        };
        let prop = self.legacy_prop_member(symbol);
        let init = match declarator.init.as_mut() {
            Some(default) => {
                let default = unwrap_paren_and_ts(self.b.move_expr(default));
                build_fallback_legacy(self.b, prop, default)
            }
            None => prop,
        };
        declarator.init = Some(init);
    }

    pub(crate) fn expand_legacy_destructure(&mut self, decl: &mut VariableDeclaration<'a>) {
        if !decl
            .declarations
            .iter()
            .any(|d| self.is_legacy_destructure(d))
        {
            return;
        }
        let kind = decl.kind;
        let mut rebuilt = self.b.ast.vec_with_capacity(decl.declarations.len());
        for mut declarator in decl.declarations.drain(..) {
            if !self.is_legacy_destructure(&declarator) {
                rebuilt.push(declarator);
                continue;
            }
            match self.expand_destructure_declarator(kind, &mut declarator) {
                Some(expanded) => {
                    for leaf in expanded {
                        rebuilt.push(leaf);
                    }
                }
                None => rebuilt.push(declarator),
            }
        }
        decl.declarations = rebuilt;
    }

    fn is_legacy_destructure(&self, declarator: &VariableDeclarator<'a>) -> bool {
        if matches!(&declarator.id, BindingPattern::BindingIdentifier(_)) {
            return false;
        }
        self.analysis
            .declarator_semantics(declarator.node_id())
            .group()
            == DeclaratorGroup::Legacy
    }

    fn expand_destructure_declarator(
        &mut self,
        kind: VariableDeclarationKind,
        declarator: &mut VariableDeclarator<'a>,
    ) -> Option<Vec<VariableDeclarator<'a>>> {
        let init = declarator.init.take()?;
        let tmp_name: &'a str = self.b.alloc_str(&self.ident_gen.generate("tmp"));

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_declarators: Vec<VariableDeclarator<'a>> = Vec::new();

        let placeholder = self
            .b
            .ast
            .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(tmp_name));
        let pattern = mem::replace(&mut declarator.id, placeholder);

        walk_bindings(&pattern, |v| {
            let root = self.b.rid_expr(tmp_name);
            let access = self.server_unfold_carrier_access(
                root,
                v.path,
                v.is_rest,
                v.excluded,
                &mut carriers,
                &mut carrier_declarators,
                kind,
            );
            let name: &'a str = self
                .b
                .alloc_str(self.analysis.scoping.symbol_name(v.symbol));
            let value = if self.analysis.binding_semantics(v.symbol).is_legacy_prop() {
                let prop = self.legacy_prop_member(v.symbol);
                build_fallback_legacy(self.b, prop, access)
            } else {
                access
            };
            leaf_declarators.push(self.build_leaf_declarator(kind, name, value));
        });

        let mut out = Vec::with_capacity(1 + carrier_declarators.len() + leaf_declarators.len());
        out.push(self.build_leaf_declarator(kind, tmp_name, init));
        out.extend(carrier_declarators);
        out.extend(leaf_declarators);
        Some(out)
    }

    #[allow(clippy::too_many_arguments)]
    fn server_unfold_carrier_access(
        &mut self,
        mut expr: Expression<'a>,
        path: &[Step<'_>],
        is_rest: bool,
        excluded: &[&PropertyKey<'_>],
        carriers: &mut HashMap<String, &'a str>,
        carrier_declarators: &mut Vec<VariableDeclarator<'a>>,
        kind: VariableDeclarationKind,
    ) -> Expression<'a> {
        for (i, step) in path.iter().enumerate() {
            match step.access {
                Access::Key { key, computed } => {
                    expr = bp::member_access(self.b, expr, key, computed);
                }
                Access::Index {
                    index,
                    len,
                    has_rest,
                } => {
                    let prefix = bp::serialize_prefix(&path[..i]);
                    let count = if has_rest { None } else { Some(len) };
                    let name = self.ensure_carrier(
                        carriers,
                        carrier_declarators,
                        &prefix,
                        expr,
                        count,
                        kind,
                    );
                    expr = self
                        .b
                        .computed_member_expr(self.b.rid_expr(name), self.b.num_expr(index as f64));
                }
                Access::Slice { from } => {
                    let prefix = bp::serialize_prefix(&path[..i]);
                    let name = self.ensure_carrier(
                        carriers,
                        carrier_declarators,
                        &prefix,
                        expr,
                        None,
                        kind,
                    );
                    let slice = self.b.static_member_expr(self.b.rid_expr(name), "slice");
                    expr = self.b.call_expr_callee(slice, [Arg::Num(from as f64)]);
                }
            }
            if let Some(default) = step.default {
                expr = bp::fallback(self.b, expr, default, None);
            }
        }
        if is_rest {
            expr = bp::exclude_from_object(self.b, expr, excluded);
        }
        expr
    }

    fn ensure_carrier(
        &mut self,
        carriers: &mut HashMap<String, &'a str>,
        carrier_declarators: &mut Vec<VariableDeclarator<'a>>,
        prefix: &str,
        source: Expression<'a>,
        count: Option<u32>,
        kind: VariableDeclarationKind,
    ) -> &'a str {
        if let Some(name) = carriers.get(prefix) {
            return name;
        }
        let name: &'a str = self.b.alloc_str(&self.ident_gen.generate("$$array"));
        let carrier = bp::to_array(self.b, source, count);
        carrier_declarators.push(self.build_leaf_declarator(kind, name, carrier));
        carriers.insert(prefix.to_string(), name);
        name
    }

    fn legacy_prop_member(&self, symbol: SymbolId) -> Expression<'a> {
        let key = self
            .analysis
            .reactivity
            .legacy_bindable_prop_alias(symbol)
            .map(str::to_string)
            .unwrap_or_else(|| self.analysis.scoping.symbol_name(symbol).to_string());
        self.b
            .computed_member_expr(self.b.rid_expr("$$props"), self.b.str_expr(&key))
    }

    fn build_leaf_declarator(
        &self,
        kind: VariableDeclarationKind,
        name: &'a str,
        value: Expression<'a>,
    ) -> VariableDeclarator<'a> {
        self.b.ast.variable_declarator(
            SPAN,
            kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
            NONE,
            Some(value),
            false,
        )
    }
}

fn build_fallback_legacy<'a>(
    b: &Builder<'a>,
    prop: Expression<'a>,
    fallback: Expression<'a>,
) -> Expression<'a> {
    if is_simple_expression(&fallback) {
        return b.call_expr("$.fallback", [Arg::Expr(prop), Arg::Expr(fallback)]);
    }
    let thunk = b.thunk(fallback);
    b.call_expr(
        "$.fallback",
        [Arg::Expr(prop), Arg::Expr(thunk), Arg::Bool(true)],
    )
}

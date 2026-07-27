use std::collections::HashMap;
use std::mem;

use oxc_allocator::CloneIn;
use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, BindingPattern, Expression, PropertyKey, VariableDeclarationKind, VariableDeclarator,
};
use oxc_span::SPAN;
use svelte_analyze::{DeclaratorSemantics, DerivedAsyncKind, DerivedKind};
use svelte_ast_builder::Arg;
use svelte_component_semantics::{Access, Step, SymbolId, walk_bindings};
use svelte_emit_builders::binding_pattern as bp;

use crate::derived::build_derived_init;
use crate::model::ServerTransform;

#[derive(Clone, Copy)]
enum Mode {
    State,
    Derived,
}

enum Root<'a> {
    Ident(&'a str),
    Reference(Expression<'a>),
    DerivedCall(&'a str),
}

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn expand_rune_destructure(
        &mut self,
        declarator: &mut VariableDeclarator<'a>,
    ) -> Option<Vec<VariableDeclarator<'a>>> {
        if matches!(&declarator.id, BindingPattern::BindingIdentifier(_)) {
            return None;
        }
        match self.analysis.declarator_semantics(declarator.node_id()) {
            DeclaratorSemantics::RuneState { .. } => self.expand_state_destructure(declarator),
            DeclaratorSemantics::RuneDerived {
                kind, async_kind, ..
            } => {
                if matches!(async_kind, DerivedAsyncKind::Async { .. }) && self.fn_depth == 0 {
                    return None;
                }
                self.expand_derived_destructure(declarator, kind, async_kind)
            }
            _ => None,
        }
    }

    fn expand_state_destructure(
        &mut self,
        declarator: &mut VariableDeclarator<'a>,
    ) -> Option<Vec<VariableDeclarator<'a>>> {
        let init = declarator.init.take()?;
        let value = self.take_call_first_argument(init)?;

        let tmp: &str = self.b.alloc_str(&self.ident_gen.generate("tmp"));

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_decls: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_decls: Vec<VariableDeclarator<'a>> = Vec::new();

        walk_bindings(&declarator.id, |v| {
            let access = self.unfold(
                Mode::State,
                &Root::Ident(tmp),
                v.path,
                v.is_rest,
                v.excluded,
                &mut carriers,
                &mut carrier_decls,
            );
            let name: &str = self
                .b
                .alloc_str(self.analysis.scoping.symbol_name(v.symbol));
            leaf_decls.push(self.make_declarator_for(name, access, Some(v.symbol)));
        });

        let mut out = vec![self.make_declarator(tmp, value)];
        out.extend(carrier_decls);
        out.extend(leaf_decls);
        Some(out)
    }

    fn expand_derived_destructure(
        &mut self,
        declarator: &mut VariableDeclarator<'a>,
        kind: DerivedKind,
        async_kind: DerivedAsyncKind,
    ) -> Option<Vec<VariableDeclarator<'a>>> {
        let init = declarator.init.take()?;
        let Expression::CallExpression(mut call) = init else {
            return None;
        };

        let inline = matches!(async_kind, DerivedAsyncKind::Sync)
            && matches!(kind, DerivedKind::Derived)
            && call
                .arguments
                .first()
                .and_then(|arg| arg.as_expression())
                .is_some_and(source_is_reference);

        let mut prefix_decls: Vec<VariableDeclarator<'a>> = Vec::new();
        let root = if inline {
            Root::Reference(call.arguments.remove(0).into_expression())
        } else {
            let arg = call.arguments.remove(0).into_expression();
            let derived = build_derived_init(self.b, kind, async_kind, arg);
            let name: &str = self.b.alloc_str(&self.ident_gen.generate("$$d"));
            prefix_decls.push(self.make_declarator(name, derived));
            Root::DerivedCall(name)
        };

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_decls: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_decls: Vec<VariableDeclarator<'a>> = Vec::new();

        walk_bindings(&declarator.id, |v| {
            let access = self.unfold(
                Mode::Derived,
                &root,
                v.path,
                v.is_rest,
                v.excluded,
                &mut carriers,
                &mut carrier_decls,
            );
            let value = self
                .b
                .call_expr("$.derived", [Arg::Expr(self.b.thunk(access))]);
            let name: &str = self
                .b
                .alloc_str(self.analysis.scoping.symbol_name(v.symbol));
            leaf_decls.push(self.make_declarator_for(name, value, Some(v.symbol)));
        });

        let mut out = prefix_decls;
        out.extend(carrier_decls);
        out.extend(leaf_decls);
        Some(out)
    }

    #[allow(clippy::too_many_arguments)]
    fn unfold<'p>(
        &mut self,
        mode: Mode,
        root: &Root<'a>,
        path: &[Step<'p>],
        is_rest: bool,
        excluded: &[&PropertyKey<'p>],
        carriers: &mut HashMap<String, &'a str>,
        carrier_decls: &mut Vec<VariableDeclarator<'a>>,
    ) -> Expression<'a> {
        let mut access = self.root_expr(root);
        for (i, step) in path.iter().enumerate() {
            match step.access {
                Access::Key { key, computed } => {
                    access = bp::member_access(self.b, access, key, computed);
                }
                Access::Index {
                    index,
                    len,
                    has_rest,
                } => {
                    let prefix = bp::serialize_prefix(&path[..i]);
                    let count = if has_rest { None } else { Some(len) };
                    let name = self.ensure_array_carrier(
                        mode,
                        carriers,
                        carrier_decls,
                        &prefix,
                        access,
                        count,
                    );
                    let base = self.carrier_read(mode, name);
                    access = self
                        .b
                        .computed_member_expr(base, self.b.num_expr(index as f64));
                }
                Access::Slice { from } => {
                    let prefix = bp::serialize_prefix(&path[..i]);
                    let name = self.ensure_array_carrier(
                        mode,
                        carriers,
                        carrier_decls,
                        &prefix,
                        access,
                        None,
                    );
                    let base = self.carrier_read(mode, name);
                    let slice = self.b.static_member_expr(base, "slice");
                    access = self.b.call_expr_callee(slice, [Arg::Num(from as f64)]);
                }
            }
            if let Some(default) = step.default {
                access = bp::fallback(self.b, access, default, None);
            }
        }
        if is_rest {
            access = bp::exclude_from_object(self.b, access, excluded);
        }
        access
    }

    fn ensure_array_carrier(
        &mut self,
        mode: Mode,
        carriers: &mut HashMap<String, &'a str>,
        carrier_decls: &mut Vec<VariableDeclarator<'a>>,
        prefix: &str,
        source: Expression<'a>,
        count: Option<u32>,
    ) -> &'a str {
        if let Some(name) = carriers.get(prefix) {
            return name;
        }
        let seed = match mode {
            Mode::State => "$$array",
            Mode::Derived => "$$derived_array",
        };
        let name: &'a str = self.b.alloc_str(&self.ident_gen.generate(seed));
        let to_array = bp::to_array(self.b, source, count);
        let init = match mode {
            Mode::State => to_array,
            Mode::Derived => self
                .b
                .call_expr("$.derived", [Arg::Expr(self.b.thunk(to_array))]),
        };
        carrier_decls.push(self.make_declarator(name, init));
        carriers.insert(prefix.to_string(), name);
        name
    }

    fn carrier_read(&self, mode: Mode, name: &'a str) -> Expression<'a> {
        match mode {
            Mode::State => self.b.rid_expr(name),
            Mode::Derived => self.b.call_expr_callee(self.b.rid_expr(name), []),
        }
    }

    fn root_expr(&self, root: &Root<'a>) -> Expression<'a> {
        match root {
            Root::Ident(name) => self.b.rid_expr(name),
            Root::Reference(expr) => expr.clone_in_with_semantic_ids(self.b.ast.allocator),
            Root::DerivedCall(name) => self.b.call_expr_callee(self.b.rid_expr(name), []),
        }
    }

    fn take_call_first_argument(&self, init: Expression<'a>) -> Option<Expression<'a>> {
        let Expression::CallExpression(mut call) = init else {
            return None;
        };
        if call.arguments.is_empty() {
            return Some(self.b.ast.expression_object(SPAN, self.b.ast.vec()));
        }
        let mut dummy = Argument::from(self.b.cheap_expr());
        mem::swap(&mut call.arguments[0], &mut dummy);
        Some(dummy.into_expression())
    }

    fn make_declarator(&self, name: &str, init: Expression<'a>) -> VariableDeclarator<'a> {
        self.make_declarator_for(name, init, None)
    }

    fn make_declarator_for(
        &self,
        name: &str,
        init: Expression<'a>,
        symbol: Option<SymbolId>,
    ) -> VariableDeclarator<'a> {
        let pattern = self
            .b
            .ast
            .binding_pattern_binding_identifier(SPAN, self.b.alloc_str(name));
        if let Some(symbol) = symbol
            && let BindingPattern::BindingIdentifier(id) = &pattern
        {
            id.symbol_id.set(Some(symbol));
        }
        self.b.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Let,
            pattern,
            NONE,
            Some(init),
            false,
        )
    }
}

fn source_is_reference(source: &Expression<'_>) -> bool {
    matches!(source.get_inner_expression(), Expression::Identifier(_))
}

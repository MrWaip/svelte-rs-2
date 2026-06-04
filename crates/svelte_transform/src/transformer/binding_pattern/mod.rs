mod derived;
mod state;

use std::collections::HashMap;
use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{
    Expression, PropertyKey, Statement, VariableDeclaration, VariableDeclarationKind,
    VariableDeclarator,
};
use oxc_span::{SPAN, Span};

use svelte_analyze::{DeclaratorSemantics, DerivedEmit};
use svelte_ast_builder::Arg;
use svelte_component_semantics::{Access, Step};
use svelte_emit_builders::binding_pattern as bp;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_binding_declarations(&mut self, stmts: &mut OxcVec<'a, Statement<'a>>) {
        let mut i = 0;
        while i < stmts.len() {
            if !matches!(&stmts[i], Statement::VariableDeclaration(_)) {
                i += 1;
                continue;
            }
            let Statement::VariableDeclaration(decl) = stmts.remove(i) else {
                unreachable!()
            };
            let produced = self.rewrite_declaration(decl.unbox());
            let n = produced.len();
            for (k, stmt) in produced.into_iter().enumerate() {
                stmts.insert(i + k, stmt);
            }
            i += n;
        }
    }

    fn rewrite_declaration(&mut self, decl: VariableDeclaration<'a>) -> Vec<Statement<'a>> {
        let decl_kind = decl.kind;
        let span = decl.span;
        let declare = decl.declare;
        let mut pending: OxcVec<'a, VariableDeclarator<'a>> = self.b.ast.vec();
        let mut out: Vec<Statement<'a>> = Vec::new();

        for declarator in decl.declarations {
            let semantics = self
                .analysis
                .map(|a| a.declarator_semantics(declarator.node_id()))
                .unwrap_or(DeclaratorSemantics::None);
            match semantics {
                DeclaratorSemantics::RuneState { kind } => {
                    self.rewrite_state(decl_kind, declarator, kind, &mut pending)
                }
                DeclaratorSemantics::RuneDerived {
                    kind,
                    emit: DerivedEmit::Sync,
                } => self.rewrite_derived(decl_kind, declarator, kind, &mut pending),

                DeclaratorSemantics::RuneDerived {
                    emit: DerivedEmit::Async,
                    ..
                } => {
                    self.flush_pending(&mut pending, decl_kind, span, declare, &mut out);
                    out.push(self.rewrite_async_derived(decl_kind, span.start, declarator));
                }

                DeclaratorSemantics::None
                | DeclaratorSemantics::PropsIdentifier { .. }
                | DeclaratorSemantics::PropsObject { .. }
                | DeclaratorSemantics::LegacyState { .. }
                | DeclaratorSemantics::ClassFieldState(_)
                | DeclaratorSemantics::ClassFieldDerived(_) => pending.push(declarator),

                DeclaratorSemantics::EachItem { .. }
                | DeclaratorSemantics::AwaitValue
                | DeclaratorSemantics::LetCarrier { .. } => {
                    unreachable!("template-stage declarator kind in a script declaration")
                }
            }
        }

        self.flush_pending(&mut pending, decl_kind, span, declare, &mut out);
        out
    }

    fn flush_pending(
        &mut self,
        pending: &mut OxcVec<'a, VariableDeclarator<'a>>,
        kind: VariableDeclarationKind,
        span: Span,
        declare: bool,
        out: &mut Vec<Statement<'a>>,
    ) {
        if pending.is_empty() {
            return;
        }
        let decls = mem::replace(pending, self.b.ast.vec());
        let decl = self.b.ast.variable_declaration(span, kind, decls, declare);
        out.push(Statement::VariableDeclaration(self.b.alloc(decl)));
    }

    #[allow(clippy::too_many_arguments)]
    fn unfold_carrier_access<'w>(
        &mut self,
        mut expr: Expression<'a>,
        path: &[Step<'w>],
        is_rest: bool,
        excluded: &[&PropertyKey<'w>],
        carriers: &mut HashMap<String, &'a str>,
        carrier_declarators: &mut Vec<VariableDeclarator<'a>>,
        dev_label: Option<&'static str>,
        decl_kind: VariableDeclarationKind,
    ) -> Expression<'a> {
        for (i, step) in path.iter().enumerate() {
            match step.access {
                Access::Key { key, computed } => {
                    expr = bp::member_access(self.b, expr, key, computed);
                }
                Access::Index { index, len, has_rest } => {
                    let prefix = bp::serialize_prefix(&path[..i]);
                    let name = self.ensure_carrier_declarator(
                        carriers,
                        carrier_declarators,
                        &prefix,
                        expr,
                        carrier_count(len, has_rest),
                        dev_label,
                        decl_kind,
                    );
                    let get = self.b.call_expr("$.get", [Arg::Ident(name)]);
                    expr = self.b.computed_member_expr(get, self.b.num_expr(index as f64));
                }
                Access::Slice { from } => {
                    let prefix = bp::serialize_prefix(&path[..i]);
                    let name = self.ensure_carrier_declarator(
                        carriers,
                        carrier_declarators,
                        &prefix,
                        expr,
                        None,
                        dev_label,
                        decl_kind,
                    );
                    let get = self.b.call_expr("$.get", [Arg::Ident(name)]);
                    let slice = self.b.static_member_expr(get, "slice");
                    expr = self.b.call_expr_callee(slice, [Arg::Num(from as f64)]);
                }
            }
            if let Some(default) = step.default {
                expr = bp::fallback(self.b, expr, default);
            }
        }
        if is_rest {
            expr = bp::exclude_from_object(self.b, expr, excluded);
        }
        expr
    }

    #[allow(clippy::too_many_arguments)]
    fn ensure_carrier_declarator(
        &mut self,
        carriers: &mut HashMap<String, &'a str>,
        carrier_declarators: &mut Vec<VariableDeclarator<'a>>,
        prefix: &str,
        source: Expression<'a>,
        count: Option<u32>,
        dev_label: Option<&'static str>,
        decl_kind: VariableDeclarationKind,
    ) -> &'a str {
        if let Some(name) = carriers.get(prefix) {
            return name;
        }
        let name_owned = self.ident_gen.generate("$$array");
        let name: &'a str = self.b.alloc_str(&name_owned);
        let derived = bp::to_array_derived(self.b, source, count);
        let derived = match dev_label.filter(|_| self.dev) {
            Some(label) => self
                .b
                .call_expr("$.tag", [Arg::Expr(derived), Arg::Str(label.to_string())]),
            None => derived,
        };
        let declarator = self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
            NONE,
            Some(derived),
            false,
        );
        carrier_declarators.push(declarator);
        carriers.insert(prefix.to_string(), name);
        name
    }
}

fn carrier_count(len: u32, has_rest: bool) -> Option<u32> {
    if has_rest { None } else { Some(len) }
}

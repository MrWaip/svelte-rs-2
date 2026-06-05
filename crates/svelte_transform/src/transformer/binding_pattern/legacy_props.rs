use std::collections::HashMap;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{BindingPattern, Expression, VariableDeclarationKind, VariableDeclarator};
use oxc_span::SPAN;
use oxc_syntax::scope::ScopeId;

use svelte_analyze::{BindingSemantics, LegacyBindablePropSemantics, PropDefaultKind, PropsFlags};
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::walk_bindings;

use super::super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(super) fn rewrite_legacy_props(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        if let BindingPattern::BindingIdentifier(id) = &declarator.id {
            let Some(sym) = id.symbol_id.get() else {
                out.push(declarator);
                return;
            };
            let Some(BindingSemantics::LegacyBindableProp(legacy)) =
                self.analysis.map(|a| a.binding_semantics(sym))
            else {
                out.push(declarator);
                return;
            };
            let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(sym));
            let init = declarator.init.take();
            let call = build_legacy_prop_call(self.b, self.gen_arrow_scope, name, None, legacy, init);
            out.push(self.build_leaf_declarator(decl_kind, name, call));
            return;
        }

        let init = declarator
            .init
            .take()
            .expect("legacy export-let destructure declarator carries an initializer");

        let tmp_name = self.ident_gen.generate("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_declarators: Vec<VariableDeclarator<'a>> = Vec::new();

        walk_bindings(&declarator.id, |v| {
            let root = self.b.rid_expr(tmp_name_str);
            let access = self.unfold_carrier_access(
                root,
                v.path,
                v.is_rest,
                v.excluded,
                &mut carriers,
                &mut carrier_declarators,
                None,
                decl_kind,
            );

            let Some(BindingSemantics::LegacyBindableProp(legacy)) =
                self.analysis.map(|a| a.binding_semantics(v.symbol))
            else {
                return;
            };

            let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(v.symbol));
            let call =
                build_legacy_prop_call(self.b, self.gen_arrow_scope, name, None, legacy, Some(access));
            leaf_declarators.push(self.build_leaf_declarator(decl_kind, name, call));
        });

        out.push(self.build_leaf_declarator(decl_kind, tmp_name_str, init));
        out.extend(carrier_declarators);
        out.extend(leaf_declarators);
    }

    fn build_leaf_declarator(
        &self,
        decl_kind: VariableDeclarationKind,
        name: &'a str,
        value: Expression<'a>,
    ) -> VariableDeclarator<'a> {
        self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
            NONE,
            Some(value),
            false,
        )
    }
}

fn build_legacy_prop_call<'a>(
    b: &Builder<'a>,
    gen_arrow_scope: Option<ScopeId>,
    local: &'a str,
    alias: Option<&str>,
    legacy: LegacyBindablePropSemantics,
    default_init: Option<Expression<'a>>,
) -> Expression<'a> {
    let prop_key = alias.unwrap_or(local).to_string();
    let mut runtime_flags = legacy.flags;
    if matches!(
        legacy.default_kind,
        PropDefaultKind::Lazy | PropDefaultKind::LazyAccessor
    ) {
        runtime_flags |= PropsFlags::LAZY_INITIAL;
    }
    let flags_bits = runtime_flags.bits();
    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$props"), Arg::Str(prop_key)];
    let default_init = default_init.map(unwrap_paren_and_ts);
    match legacy.default_kind {
        PropDefaultKind::None => {
            if !runtime_flags.is_empty() {
                args.push(Arg::Num(flags_bits as f64));
            }
        }
        PropDefaultKind::Eager => {
            args.push(Arg::Num(flags_bits as f64));
            let default_expr = default_init
                .unwrap_or_else(|| panic!("eager default missing for legacy prop {local}"));
            args.push(Arg::Expr(default_expr));
        }
        PropDefaultKind::Lazy => {
            args.push(Arg::Num(flags_bits as f64));
            let default_expr = default_init
                .unwrap_or_else(|| panic!("lazy default missing for legacy prop {local}"));
            let lazy = super::super::derived::wrap_lazy(b, default_expr);
            b.seed_arrow_scope(&lazy, gen_arrow_scope);
            args.push(Arg::Expr(lazy));
        }
        PropDefaultKind::LazyAccessor => {
            args.push(Arg::Num(flags_bits as f64));
            let default_expr = default_init.unwrap_or_else(|| {
                panic!("lazy accessor default missing for legacy prop {local}")
            });
            let accessor_name = match &default_expr {
                Expression::Identifier(id) => id.name.as_str().to_string(),
                Expression::CallExpression(call) if call.arguments.is_empty() => match &call.callee {
                    Expression::Identifier(callee) => callee.name.as_str().to_string(),
                    _ => panic!(
                        "lazy accessor default must be a bare-identifier call for legacy prop {local}"
                    ),
                },
                _ => panic!(
                    "lazy accessor default must be an identifier or bare-call for legacy prop {local}"
                ),
            };
            args.push(Arg::Expr(b.rid_expr(&accessor_name)));
        }
    }
    b.call_expr("$.prop", args)
}

fn unwrap_paren_and_ts<'a>(expr: Expression<'a>) -> Expression<'a> {
    let mut inner = expr.into_inner_expression();
    match &mut inner {
        Expression::ArrowFunctionExpression(arrow) => arrow.pife = false,
        Expression::FunctionExpression(func) => func.pife = false,
        _ => {}
    }
    inner
}

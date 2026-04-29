use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{Expression, Program, Statement};
use rustc_hash::{FxHashMap, FxHashSet};
use svelte_analyze::AnalysisData;
use svelte_analyze::reactivity_semantics::legacy_reactive::{
    LegacyReactiveKind, LegacyReactiveStatement, legacy_reactive_import_wrapper_name,
};
use svelte_analyze::types::data::BindingSemantics;
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::{OxcNodeId, SymbolId};

pub(crate) fn rewrite_legacy_reactive<'a>(
    b: &Builder<'a>,
    program: &mut Program<'a>,
    analysis: &AnalysisData<'a>,
) {
    let lr = analysis.reactivity.legacy_reactive();
    let topo: Vec<&LegacyReactiveStatement> = lr.iter_statements_topo().collect();
    if topo.is_empty() {
        return;
    }

    let stmt_node_set: FxHashSet<OxcNodeId> = topo.iter().map(|s| s.stmt_node).collect();
    let stmt_meta_by_node: FxHashMap<OxcNodeId, &LegacyReactiveStatement> =
        topo.iter().map(|s| (s.stmt_node, *s)).collect();

    let allocator = b.ast.allocator;
    let old_body = std::mem::replace(&mut program.body, OxcVec::new_in(allocator));

    let mut bodies_by_node: FxHashMap<OxcNodeId, Statement<'a>> = FxHashMap::default();
    enum Slot<'a> {
        Keep(Statement<'a>),
        DollarPlaceholder,
    }
    let mut slots: Vec<Slot<'a>> = Vec::with_capacity(old_body.len());
    let mut source_order: Vec<OxcNodeId> = Vec::with_capacity(topo.len());
    for stmt in old_body {
        if let Statement::LabeledStatement(labeled) = stmt {
            let node_id = labeled.node_id();
            if stmt_node_set.contains(&node_id) {
                bodies_by_node.insert(node_id, labeled.unbox().body);
                slots.push(Slot::DollarPlaceholder);
                source_order.push(node_id);
                continue;
            }
            slots.push(Slot::Keep(Statement::LabeledStatement(labeled)));
            continue;
        }
        slots.push(Slot::Keep(stmt));
    }

    let mut implicit_syms: Vec<SymbolId> = Vec::new();
    let mut seen_implicit: FxHashSet<SymbolId> = FxHashSet::default();
    for node_id in &source_order {
        let Some(stmt) = stmt_meta_by_node.get(node_id) else {
            continue;
        };
        match &stmt.kind {
            LegacyReactiveKind::SimpleAssignment {
                target_sym,
                implicit_decl: true,
            } => {
                if seen_implicit.insert(*target_sym) {
                    implicit_syms.push(*target_sym);
                }
            }
            LegacyReactiveKind::DestructureAssignment {
                implicit_decl_syms, ..
            } => {
                for sym in implicit_decl_syms {
                    if seen_implicit.insert(*sym) {
                        implicit_syms.push(*sym);
                    }
                }
            }
            _ => {}
        }
    }

    let mut pre_effect_stmts: Vec<Statement<'a>> = Vec::with_capacity(topo.len() + 1);
    for stmt_meta in &topo {
        let Some(body) = bodies_by_node.remove(&stmt_meta.stmt_node) else {
            continue;
        };
        let body_thunk = wrap_body_as_thunk(b, body);
        let deps_thunk = build_deps_thunk(b, stmt_meta, analysis);
        let call = b.call_expr(
            "$.legacy_pre_effect",
            [Arg::Expr(deps_thunk), Arg::Expr(body_thunk)],
        );
        pre_effect_stmts.push(b.expr_stmt(call));
    }
    pre_effect_stmts.push(b.call_stmt("$.legacy_pre_effect_reset", []));

    let mut new_body: OxcVec<'a, Statement<'a>> = OxcVec::with_capacity_in(
        slots.len() + implicit_syms.len() + topo.len() + 1,
        allocator,
    );

    for sym in &implicit_syms {
        let name = analysis.scoping.symbol_name(*sym);
        let init = b.call_expr("$.mutable_source", []);
        new_body.push(b.const_stmt(name, init));
    }

    let mut inserted = false;
    for slot in slots {
        match slot {
            Slot::Keep(stmt) => new_body.push(stmt),
            Slot::DollarPlaceholder => {
                if !inserted {
                    for stmt in pre_effect_stmts.drain(..) {
                        new_body.push(stmt);
                    }
                    inserted = true;
                }
            }
        }
    }

    program.body = new_body;
}

fn wrap_body_as_thunk<'a>(b: &Builder<'a>, body: Statement<'a>) -> Expression<'a> {
    match body {
        Statement::BlockStatement(block) => {
            let block = block.unbox();
            let stmts: Vec<Statement<'a>> = block.body.into_iter().collect();
            b.thunk_block(stmts)
        }
        other => b.thunk_block(vec![other]),
    }
}

fn build_deps_thunk<'a>(
    b: &Builder<'a>,
    stmt: &LegacyReactiveStatement,
    analysis: &AnalysisData<'a>,
) -> Expression<'a> {
    let mut dep_exprs: Vec<Expression<'a>> = stmt
        .dependencies
        .iter()
        .map(|&sym| build_dep_read(b, sym, analysis))
        .collect();
    if stmt.uses_props {
        dep_exprs.push(b.call_expr(
            "$.deep_read_state",
            [Arg::Expr(b.rid_expr("$$sanitized_props"))],
        ));
    }
    if stmt.uses_rest_props {
        dep_exprs.push(b.call_expr("$.deep_read_state", [Arg::Expr(b.rid_expr("$$restProps"))]));
    }

    if dep_exprs.is_empty() {
        return b.thunk_block(Vec::new());
    }

    if dep_exprs.len() == 1 {
        let single = dep_exprs.into_iter().next().expect("len==1");
        return b.arrow_expr(b.no_params(), [b.expr_stmt(single)]);
    }

    let allocator = b.ast.allocator;
    let mut seq_vec: OxcVec<'a, Expression<'a>> =
        OxcVec::with_capacity_in(dep_exprs.len(), allocator);
    for expr in dep_exprs {
        seq_vec.push(expr);
    }
    let seq = b.ast.expression_sequence(oxc_span::SPAN, seq_vec);
    b.arrow_expr(b.no_params(), [b.expr_stmt(seq)])
}

fn build_dep_read<'a>(
    b: &Builder<'a>,
    sym: SymbolId,
    analysis: &AnalysisData<'a>,
) -> Expression<'a> {
    let name = analysis.scoping.symbol_name(sym);
    if analysis.reactivity.legacy_reactive().is_mutated_import(sym) {
        let import_name: &str = b.alloc_str(&legacy_reactive_import_wrapper_name(name));
        return b.call_expr_callee(b.rid_expr(import_name), []);
    }
    match analysis.reactivity.binding_semantics(sym) {
        BindingSemantics::LegacyState(state) => {
            let helper = if state.var_declared {
                "$.safe_get"
            } else {
                "$.get"
            };
            b.call_expr(helper, [Arg::Expr(b.rid_expr(name))])
        }
        BindingSemantics::LegacyBindableProp(_) => {
            let accessor_call = b.call_expr_callee(b.rid_expr(name), []);
            b.call_expr("$.deep_read_state", [Arg::Expr(accessor_call)])
        }
        _ => b.rid_expr(name),
    }
}

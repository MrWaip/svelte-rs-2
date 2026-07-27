use rustc_hash::FxHashMap;
use std::mem;

use oxc_ast::ast::{
    BindingPattern, Declaration, Expression, Function, Program, Statement, VariableDeclaration,
    VariableDeclarator,
};
use svelte_component_semantics::SymbolId;

use crate::expression_semantics::Suspension;
use crate::types::data::{
    AnalysisData, AsyncEntry, AsyncEntryLocation, AsyncEntryMember, AsyncEntryMemberKind,
    BlockerSlot, JsAst,
};
use crate::utils::{expression_has_await, statement_has_await};

pub(crate) fn calculate_instance_blockers(parsed: &JsAst<'_>, data: &mut AnalysisData) {
    let Some(program) = parsed.program.as_ref() else {
        return;
    };

    let mut awaited = false;
    let root = data.scoping.root_scope_id();
    let mut non_import_idx: usize = 0;
    let mut sync_group: Vec<AsyncEntryMember> = Vec::new();
    let mut member_counter: u32 = 0;
    let assigned = super::async_trace::AssignedValues::collect(&data.scoping, program);

    for stmt in &program.body {
        if matches!(stmt, Statement::ImportDeclaration(_)) {
            continue;
        }

        let stmt_ref = match stmt {
            Statement::ExportNamedDeclaration(export) if export.declaration.is_none() => {
                non_import_idx += 1;
                continue;
            }
            other => other,
        };

        let suspension = statement_suspension(stmt_ref);
        let has_await = suspension.suspends();
        awaited |= has_await;

        if awaited && data.script.blocker_data.first_await_index.is_none() {
            data.script.blocker_data.first_await_index = Some(non_import_idx);
        }

        if is_function_declaration(stmt_ref) || !awaited {
            non_import_idx += 1;
            continue;
        }

        match declaration_of(stmt_ref) {
            Some(var_decl) => {
                for declarator in var_decl.declarations.iter() {
                    if is_props_id_declarator(data, declarator) {
                        continue;
                    }
                    if matches!(
                        declarator.init.as_ref().map(|e| e.get_inner_expression()),
                        Some(
                            Expression::ArrowFunctionExpression(_)
                                | Expression::FunctionExpression(_)
                        )
                    ) {
                        continue;
                    }

                    if has_await {
                        flush_sync_group(data, &mut sync_group);
                    }
                    let index = data.script.blocker_data.entries.len() as u32;
                    let slot = BlockerSlot {
                        entry: index,
                        member: member_counter,
                    };
                    member_counter += 1;

                    for sym in declarator_writes(data, &assigned, declarator) {
                        data.script.blocker_data.symbol_blockers.insert(sym, slot);
                    }

                    let mut syms: Vec<SymbolId> = Vec::new();
                    svelte_component_semantics::walk_bindings(&declarator.id, |v| {
                        syms.push(v.symbol);
                    });
                    for sym in &syms {
                        data.script.blocker_data.symbol_blockers.insert(*sym, slot);
                        let name = data.scoping.symbol_name(*sym).to_string();
                        data.script.blocker_data.hoisted_names.push(name);
                    }

                    let member = AsyncEntryMember {
                        node: declarator.node_id(),
                        stmt_index: non_import_idx,
                        kind: AsyncEntryMemberKind::Declarator,
                        symbols: syms,
                    };
                    push_member(data, &mut sync_group, member, suspension, has_await);
                }
            }
            None => {
                if has_await {
                    flush_sync_group(data, &mut sync_group);
                }
                let index = data.script.blocker_data.entries.len() as u32;
                let slot = BlockerSlot {
                    entry: index,
                    member: member_counter,
                };
                member_counter += 1;

                for sym in statement_writes(data, &assigned, stmt_ref) {
                    data.script.blocker_data.symbol_blockers.insert(sym, slot);
                }

                if let Statement::ClassDeclaration(class) = stmt_ref
                    && let Some(ref id) = class.id
                {
                    let name = id.name.to_string();
                    if let Some(sym) = data.scoping.find_binding(root, &name) {
                        data.script.blocker_data.symbol_blockers.insert(sym, slot);
                    }
                    data.script.blocker_data.hoisted_names.push(name);
                }

                let member = AsyncEntryMember {
                    node: stmt_ref.node_id(),
                    stmt_index: non_import_idx,
                    kind: AsyncEntryMemberKind::Statement,
                    symbols: Vec::new(),
                };
                push_member(data, &mut sync_group, member, suspension, has_await);
            }
        }

        non_import_idx += 1;
    }

    flush_sync_group(data, &mut sync_group);
    assign_deferred_function_blockers(program, data, &assigned, &mut member_counter);
    data.script.blocker_data.has_async = !data.script.blocker_data.entries.is_empty();
    build_member_lookup(data);
}

fn build_member_lookup(data: &mut AnalysisData) {
    let mut lookup = FxHashMap::default();
    let mut by_symbol = FxHashMap::default();
    for (entry_index, entry) in data.script.blocker_data.entries.iter().enumerate() {
        for member in &entry.members {
            let location = AsyncEntryLocation {
                entry: entry_index,
                kind: member.kind,
            };
            lookup.insert(member.node, location);
            for sym in &member.symbols {
                by_symbol.insert(*sym, location);
            }
        }
    }
    data.script.blocker_data.member_lookup = lookup;
    data.script.blocker_data.symbol_member_lookup = by_symbol;
}

fn push_member(
    data: &mut AnalysisData,
    sync_group: &mut Vec<AsyncEntryMember>,
    member: AsyncEntryMember,
    suspension: Suspension,
    has_await: bool,
) {
    if !has_await {
        sync_group.push(member);
        return;
    }
    data.script.blocker_data.entries.push(AsyncEntry {
        members: vec![member],
        suspension,
    });
}

fn flush_sync_group(data: &mut AnalysisData, sync_group: &mut Vec<AsyncEntryMember>) {
    if sync_group.is_empty() {
        return;
    }
    data.script.blocker_data.entries.push(AsyncEntry {
        members: mem::take(sync_group),
        suspension: Suspension::None,
    });
}

fn is_function_declaration(stmt: &Statement<'_>) -> bool {
    match stmt {
        Statement::FunctionDeclaration(_) => true,
        Statement::ExportNamedDeclaration(export) => matches!(
            &export.declaration,
            Some(Declaration::FunctionDeclaration(_))
        ),
        _ => false,
    }
}

fn declaration_of<'b, 'a>(stmt: &'b Statement<'a>) -> Option<&'b VariableDeclaration<'a>> {
    match stmt {
        Statement::VariableDeclaration(v) => Some(v),
        Statement::ExportNamedDeclaration(export) => match &export.declaration {
            Some(Declaration::VariableDeclaration(v)) => Some(v),
            _ => None,
        },
        _ => None,
    }
}

fn statement_suspension(stmt: &Statement<'_>) -> Suspension {
    if !statement_has_await(stmt) {
        return Suspension::None;
    }
    let Statement::ExpressionStatement(expr_stmt) = stmt else {
        return Suspension::Interleaved;
    };
    let Expression::AwaitExpression(await_expr) = expr_stmt.expression.get_inner_expression()
    else {
        return Suspension::Interleaved;
    };
    if expression_has_await(&await_expr.argument) {
        return Suspension::Interleaved;
    }
    Suspension::Outermost
}

fn is_props_id_declarator(data: &AnalysisData, declarator: &VariableDeclarator<'_>) -> bool {
    if data
        .declarator_semantics(declarator.node_id())
        .is_props_id_call()
    {
        return true;
    }
    let Some(init) = declarator.init.as_ref() else {
        return false;
    };
    data.declarator_semantics(init.get_inner_expression().node_id())
        .is_props_id_call()
}

fn declarator_writes<'a>(
    data: &AnalysisData,
    assigned: &super::async_trace::AssignedValues,
    declarator: &VariableDeclarator<'a>,
) -> Vec<SymbolId> {
    let Some(init) = declarator.init.as_ref() else {
        return Vec::new();
    };
    let mut tracer = super::async_trace::TouchedSymbols::new(&data.scoping, assigned);
    tracer.visit_init(init);
    tracer.writes
}

fn statement_writes<'a>(
    data: &AnalysisData,
    assigned: &super::async_trace::AssignedValues,
    stmt: &Statement<'a>,
) -> Vec<SymbolId> {
    super::async_trace::trace_statement(&data.scoping, assigned, stmt).writes
}

fn assign_deferred_function_blockers(
    program: &Program<'_>,
    data: &mut AnalysisData,
    assigned: &super::async_trace::AssignedValues,
    member_counter: &mut u32,
) {
    let root = data.scoping.root_scope_id();

    for stmt in &program.body {
        let stmt_ref = match stmt {
            Statement::ExportNamedDeclaration(export) if export.declaration.is_none() => continue,
            other => other,
        };

        if let Some(func) = function_declaration_of(stmt_ref) {
            let Some(id) = func.id.as_ref() else {
                continue;
            };
            let Some(sym) = data.scoping.find_binding(root, id.name.as_str()) else {
                continue;
            };
            let Some(body) = func.body.as_ref() else {
                continue;
            };
            let mut tracer = super::async_trace::TouchedSymbols::new(&data.scoping, assigned);
            tracer.visit_body(body);
            if let Some(entry) = max_blocker_entry(data, &tracer) {
                assign_function_blocker(data, sym, entry, member_counter);
            }
            continue;
        }

        let Some(var_decl) = declaration_of(stmt_ref) else {
            continue;
        };
        for declarator in &var_decl.declarations {
            let Some(init) = declarator.init.as_ref() else {
                continue;
            };
            if !matches!(
                init.get_inner_expression(),
                Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
            ) {
                continue;
            }
            let BindingPattern::BindingIdentifier(id) = &declarator.id else {
                continue;
            };
            let Some(sym) = data.scoping.find_binding(root, id.name.as_str()) else {
                continue;
            };
            let mut tracer = super::async_trace::TouchedSymbols::new(&data.scoping, assigned);
            tracer.visit_function_like(init.get_inner_expression());
            if let Some(entry) = max_blocker_entry(data, &tracer) {
                assign_function_blocker(data, sym, entry, member_counter);
            }
        }
    }
}

fn assign_function_blocker(
    data: &mut AnalysisData,
    sym: SymbolId,
    entry: u32,
    member_counter: &mut u32,
) {
    let slot = BlockerSlot {
        entry,
        member: *member_counter,
    };
    *member_counter += 1;
    data.script.blocker_data.symbol_blockers.insert(sym, slot);
}

fn max_blocker_entry(
    data: &AnalysisData,
    tracer: &super::async_trace::TouchedSymbols<'_, '_>,
) -> Option<u32> {
    let mut max: Option<u32> = None;
    for sym in tracer.reads.iter().chain(tracer.writes.iter()) {
        let Some(slot) = data.script.blocker_data.symbol_blocker(*sym) else {
            continue;
        };
        max = Some(match max {
            Some(current) => current.max(slot.entry),
            None => slot.entry,
        });
    }
    max
}

fn function_declaration_of<'b, 'a>(stmt: &'b Statement<'a>) -> Option<&'b Function<'a>> {
    match stmt {
        Statement::FunctionDeclaration(func) => Some(func),
        Statement::ExportNamedDeclaration(export) => match &export.declaration {
            Some(Declaration::FunctionDeclaration(func)) => Some(func),
            _ => None,
        },
        _ => None,
    }
}

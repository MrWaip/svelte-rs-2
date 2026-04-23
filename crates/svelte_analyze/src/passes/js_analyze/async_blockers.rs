use oxc_ast::ast::{Expression, Statement};
use oxc_ast_visit::walk::{walk_arrow_function_expression, walk_function};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeFlags;

use crate::types::data::{AnalysisData, AsyncStmtMeta, JsAst};

fn has_await_in_statement(stmt: &Statement<'_>) -> bool {
    struct AwaitCheck {
        found: bool,
        fn_depth: u32,
    }

    impl<'a> Visit<'a> for AwaitCheck {
        fn visit_await_expression(&mut self, _expr: &oxc_ast::ast::AwaitExpression<'a>) {
            if self.fn_depth == 0 {
                self.found = true;
            }
        }

        fn visit_arrow_function_expression(
            &mut self,
            arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
        ) {
            self.fn_depth += 1;
            walk_arrow_function_expression(self, arrow);
            self.fn_depth -= 1;
        }

        fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, flags: ScopeFlags) {
            self.fn_depth += 1;
            walk_function(self, func, flags);
            self.fn_depth -= 1;
        }
    }

    let mut check = AwaitCheck {
        found: false,
        fn_depth: 0,
    };
    check.visit_statement(stmt);
    check.found
}

pub(crate) fn calculate_instance_blockers(parsed: &JsAst<'_>, data: &mut AnalysisData) {
    let Some(program) = parsed.program.as_ref() else {
        return;
    };

    let mut awaited = false;
    let mut async_index: u32 = 0;
    let root = data.scoping.root_scope_id();
    let mut non_import_idx: usize = 0;

    for stmt in &program.body {
        if matches!(stmt, Statement::ImportDeclaration(_)) {
            continue;
        }

        let stmt_ref = if let Statement::ExportNamedDeclaration(export) = stmt {
            if export.declaration.is_some() {
                stmt
            } else {
                non_import_idx += 1;
                continue;
            }
        } else {
            stmt
        };

        let has_await = has_await_in_statement(stmt_ref);
        awaited |= has_await;

        if awaited && data.script.blocker_data.first_await_index.is_none() {
            data.script.blocker_data.first_await_index = Some(non_import_idx);
        }

        let is_function = matches!(stmt_ref, Statement::FunctionDeclaration(_))
            || matches!(stmt_ref, Statement::ExportNamedDeclaration(export)
                if matches!(&export.declaration, Some(oxc_ast::ast::Declaration::FunctionDeclaration(_))));

        if is_function {
            if awaited {
                data.script.blocker_data.stmt_metas.push(AsyncStmtMeta {
                    has_await: false,
                    hoist_names: Vec::new(),
                });
            }
            non_import_idx += 1;
            continue;
        }

        if !awaited {
            non_import_idx += 1;
            continue;
        }

        let mut hoist_names = Vec::new();
        let var_decl = match stmt_ref {
            Statement::VariableDeclaration(v) => Some(&**v),
            Statement::ExportNamedDeclaration(export) => {
                if let Some(oxc_ast::ast::Declaration::VariableDeclaration(v)) = &export.declaration
                {
                    Some(&**v)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(var_decl) = var_decl {
            for declarator in &var_decl.declarations {
                if matches!(
                    &declarator.init,
                    Some(
                        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
                    )
                ) {
                    continue;
                }

                let mut syms: Vec<svelte_component_semantics::SymbolId> = Vec::new();
                svelte_component_semantics::walk_bindings(&declarator.id, |v| {
                    syms.push(v.symbol);
                });
                for sym in syms {
                    data.script
                        .blocker_data
                        .symbol_blockers
                        .insert(sym, async_index);
                    hoist_names.push(data.scoping.symbol_name(sym).to_string());
                }
                let _ = root;
                async_index += 1;
            }
        } else {
            if let Statement::ClassDeclaration(class) = stmt_ref {
                if let Some(ref id) = class.id {
                    let name = id.name.to_string();
                    if let Some(sym) = data.scoping.find_binding(root, &name) {
                        data.script
                            .blocker_data
                            .symbol_blockers
                            .insert(sym, async_index);
                    }
                    hoist_names.push(name);
                }
            }
            async_index += 1;
        }

        data.script.blocker_data.stmt_metas.push(AsyncStmtMeta {
            has_await,
            hoist_names,
        });

        non_import_idx += 1;
    }

    data.script.blocker_data.async_thunk_count = async_index;
    data.script.blocker_data.has_async = async_index > 0;
}

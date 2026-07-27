use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{
    AsyncEntry, AsyncEntryLocation, AsyncEntryMemberKind, BlockerData, Suspension,
};
use svelte_ast_builder::{Builder, OutermostAwait};

pub enum EntryStatement<'a> {
    Expression(Expression<'a>),
    Value(Expression<'a>),
    Plain(Statement<'a>),
}

pub fn entry_thunk<'a>(
    b: &Builder<'a>,
    entry: &AsyncEntry,
    statements: Vec<EntryStatement<'a>>,
) -> Expression<'a> {
    let mut statements = statements;
    if statements.is_empty() {
        return b.thunk(b.void_zero_expr());
    }
    if statements.len() == 1 {
        match statements.remove(0) {
            EntryStatement::Expression(expr) => {
                return suspending_arrow_body(b, void_unless_await(b, expr), entry.suspension());
            }
            EntryStatement::Value(expr) => {
                return suspending_arrow_body(b, expr, entry.suspension());
            }
            EntryStatement::Plain(stmt) => statements.push(EntryStatement::Plain(stmt)),
        }
    }

    let mut body: Vec<Statement<'a>> = Vec::with_capacity(statements.len());
    for statement in statements {
        match statement {
            EntryStatement::Expression(expr) => body.push(b.expr_stmt(b.void_expr(expr))),
            EntryStatement::Value(expr) => body.push(b.expr_stmt(expr)),
            EntryStatement::Plain(stmt) => body.push(stmt),
        }
    }
    if entry.suspends() {
        return b.async_thunk_block(body);
    }
    b.thunk_block(body)
}

fn void_unless_await<'a>(b: &Builder<'a>, expr: Expression<'a>) -> Expression<'a> {
    if matches!(expr.get_inner_expression(), Expression::AwaitExpression(_)) {
        return expr;
    }
    b.void_expr(expr)
}

pub fn suspending_arrow_body<'a>(
    b: &Builder<'a>,
    expr: Expression<'a>,
    suspension: Suspension,
) -> Expression<'a> {
    let suspends = suspension.suspends();
    if !suspension.is_outermost() {
        if suspends {
            return b.async_arrow_expr_body(expr);
        }
        return b.thunk(expr);
    }
    match b.outermost_await(expr) {
        OutermostAwait::Operand(operand) => b.thunk(operand),
        OutermostAwait::Absent(expr) => b.async_arrow_expr_body(expr),
    }
}

pub fn statement_entry_location(
    blocker_data: &BlockerData,
    stmt: &Statement<'_>,
    stmt_index: usize,
) -> Option<AsyncEntryLocation> {
    if let Some(location) = entry_location_by_node(blocker_data, stmt) {
        return Some(location);
    }
    if stmt_index < blocker_data.first_await_index()? {
        return None;
    }
    blocker_data.entry_location_at(stmt_index)
}

fn entry_location_by_node(
    blocker_data: &BlockerData,
    stmt: &Statement<'_>,
) -> Option<AsyncEntryLocation> {
    if let Some(location) = blocker_data.entry_location(stmt.node_id()) {
        return Some(location);
    }
    match stmt {
        Statement::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                if let Some(location) = blocker_data.entry_location(declarator.node_id()) {
                    return Some(location);
                }
            }
            None
        }
        Statement::BlockStatement(block) => {
            for inner in &block.body {
                if let Some(location) = entry_location_by_node(blocker_data, inner) {
                    return Some(location);
                }
            }
            None
        }
        _ => None,
    }
}

pub fn push_entry_statement<'a>(
    b: &Builder<'a>,
    bucket: &mut Vec<EntryStatement<'a>>,
    stmt: Statement<'a>,
    kind: AsyncEntryMemberKind,
) {
    match kind {
        AsyncEntryMemberKind::SideEffect => match stmt {
            Statement::EmptyStatement(_) => {}
            Statement::ExpressionStatement(expr_stmt) => {
                bucket.push(EntryStatement::Expression(expr_stmt.unbox().expression));
            }
            other => bucket.push(EntryStatement::Plain(other)),
        },
        AsyncEntryMemberKind::Binding => push_binding_statement(b, bucket, stmt),
    }
}

fn push_binding_statement<'a>(
    b: &Builder<'a>,
    bucket: &mut Vec<EntryStatement<'a>>,
    stmt: Statement<'a>,
) {
    match stmt {
        Statement::EmptyStatement(_) => {}
        Statement::ExpressionStatement(expr_stmt) => {
            bucket.push(EntryStatement::Value(expr_stmt.unbox().expression));
        }
        Statement::BlockStatement(block) => {
            for inner in block.unbox().body {
                push_binding_statement(b, bucket, inner);
            }
        }
        Statement::ClassDeclaration(class) if class.id.is_some() => {
            bucket.push(EntryStatement::Value(
                b.class_declaration_to_assignment(class),
            ));
        }
        other => bucket.push(EntryStatement::Plain(other)),
    }
}

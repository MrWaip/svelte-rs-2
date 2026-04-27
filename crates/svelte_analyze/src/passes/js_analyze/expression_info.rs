use crate::types::data::{ExpressionInfo, ExpressionKind};
use crate::types::script::RuneKind;
use compact_str::CompactString;
use oxc_ast::ast::{
    AssignmentTargetPropertyIdentifier, CallExpression, Expression, MemberExpression,
    SimpleAssignmentTarget,
};
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_expression, walk_call_expression,
    walk_expression, walk_function, walk_member_expression, walk_simple_assignment_target,
    walk_update_expression,
};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeFlags;

struct ExpressionAnalyzer {
    kind: ExpressionKind,
    uses_legacy_slots: bool,
    /// LEGACY(svelte4): true when the expression contains a read of the
    /// unresolved `$$props` / `$$restProps` identifier. Drives the legacy
    /// coarse-wrap (`$.deep_read_state` / `$.untrack`) at codegen time.
    /// Deprecated in Svelte 5, remove in Svelte 6.
    uses_legacy_sanitized_props: bool,
    has_call: bool,
    has_await: bool,
    has_state_rune: bool,
    has_store_member_mutation: bool,
    has_store_ref: bool,
    has_side_effects: bool,
    depth: u32,
    fn_depth: u32,
    in_write_position: bool,
}

impl<'a> Visit<'a> for ExpressionAnalyzer {
    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if self.depth == 0 {
            self.kind = match expr {
                Expression::Identifier(ident) => {
                    ExpressionKind::Identifier(CompactString::from(ident.name.as_str()))
                }
                Expression::NumericLiteral(_)
                | Expression::StringLiteral(_)
                | Expression::BooleanLiteral(_)
                | Expression::NullLiteral(_) => ExpressionKind::Literal,
                Expression::CallExpression(call) => {
                    let callee = match &call.callee {
                        Expression::Identifier(ident) => CompactString::from(ident.name.as_str()),
                        _ => CompactString::default(),
                    };
                    ExpressionKind::CallExpression { callee }
                }
                Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_) => {
                    ExpressionKind::MemberExpression
                }
                Expression::ArrowFunctionExpression(_) => ExpressionKind::ArrowFunction,
                Expression::AssignmentExpression(_) => ExpressionKind::Assignment,
                _ => ExpressionKind::Other,
            };
            self.has_side_effects = matches!(
                expr,
                Expression::CallExpression(_)
                    | Expression::AssignmentExpression(_)
                    | Expression::UpdateExpression(_)
            );
        }
        self.depth += 1;
        walk_expression(self, expr);
        self.depth -= 1;
    }

    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        let name = ident.name.as_str();
        if name == "$$slots" {
            self.uses_legacy_slots = true;
        }
        if name == "$$props" || name == "$$restProps" {
            self.uses_legacy_sanitized_props = true;
        }
        if name.starts_with('$') && name.len() > 1 {
            self.has_store_ref = true;
        }
        self.in_write_position = false;
    }

    fn visit_assignment_expression(&mut self, assign: &oxc_ast::ast::AssignmentExpression<'a>) {
        walk_assignment_expression(self, assign);
    }

    fn visit_simple_assignment_target(&mut self, it: &SimpleAssignmentTarget<'a>) {
        self.in_write_position = true;
        walk_simple_assignment_target(self, it);
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &AssignmentTargetPropertyIdentifier<'a>,
    ) {
        self.in_write_position = true;
        self.visit_identifier_reference(&it.binding);
        if let Some(init) = &it.init {
            self.visit_expression(init);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        if self.in_write_position {
            let root_expr = match expr {
                MemberExpression::StaticMemberExpression(m) => Some(&m.object),
                MemberExpression::ComputedMemberExpression(m) => Some(&m.object),
                _ => None,
            };
            if root_expr.is_some_and(member_root_is_store) {
                self.has_store_member_mutation = true;
            }
        }
        self.in_write_position = false;
        walk_member_expression(self, expr);
    }

    fn visit_update_expression(&mut self, upd: &oxc_ast::ast::UpdateExpression<'a>) {
        self.in_write_position = true;
        walk_update_expression(self, upd);
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_call = true;
            if let Some(rune) = crate::utils::script_info::detect_rune_from_call(call) {
                if matches!(rune, RuneKind::EffectPending | RuneKind::StateEager) {
                    self.has_state_rune = true;
                }
            }
        }

        walk_call_expression(self, call);
    }

    fn visit_await_expression(&mut self, expr: &oxc_ast::ast::AwaitExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_await = true;
        }
        oxc_ast_visit::walk::walk_await_expression(self, expr);
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

pub(crate) fn analyze_expression(expr: &Expression<'_>) -> ExpressionInfo {
    let mut analyzer = ExpressionAnalyzer {
        kind: ExpressionKind::Other,
        uses_legacy_slots: false,
        uses_legacy_sanitized_props: false,
        has_call: false,
        has_await: false,
        has_state_rune: false,
        has_store_member_mutation: false,
        has_store_ref: false,
        has_side_effects: false,
        depth: 0,
        fn_depth: 0,
        in_write_position: false,
    };
    analyzer.visit_expression(expr);
    let mut info = ExpressionInfo::new(analyzer.kind);
    info.set_initial_flags(
        analyzer.uses_legacy_slots,
        analyzer.has_store_ref,
        analyzer.has_side_effects,
        analyzer.has_call,
        analyzer.has_await,
        analyzer.has_state_rune,
        analyzer.has_store_member_mutation,
    );
    info.set_uses_legacy_sanitized_props(analyzer.uses_legacy_sanitized_props);
    info
}

fn member_root_is_store(expr: &Expression<'_>) -> bool {
    let mut node = expr;
    loop {
        match node {
            Expression::StaticMemberExpression(m) => node = &m.object,
            Expression::ComputedMemberExpression(m) => node = &m.object,
            _ => break,
        }
    }
    if let Expression::Identifier(id) = node {
        id.name.starts_with('$') && id.name.len() > 1
    } else {
        false
    }
}

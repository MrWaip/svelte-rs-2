use oxc_ast::ast::{
    AssignmentExpression, AssignmentTarget, CallExpression, Class, Expression, MethodDefinition,
    MethodDefinitionKind, Program, PropertyDefinition, SimpleAssignmentTarget, Statement,
    UpdateExpression,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_assignment_expression, walk_call_expression, walk_method_definition,
    walk_update_expression,
};

use super::needs_context::NeedsContextVisitor;
use crate::reactivity_semantics::data::{
    DeclaratorSemantics, ReactivitySemantics, RuntimeRuneKind,
};
use crate::scope::ComponentScoping;
use crate::types::data::AnalysisData;

pub(crate) fn analyze_script(data: &mut AnalysisData, program: &Program<'_>) {
    let body = analyze_script_body(program, &data.reactivity);
    data.script.has_store_member_mutations = body.has_store_member_mutations;
    data.script.has_class_state_fields = body.has_class_state_fields;
    data.output.needs_context = body.has_effects
        || body.has_store_member_mutations
        || NeedsContextVisitor::check(program, &data.scoping, &data.reactivity);
}

pub(crate) fn needs_context_for_program(
    program: &Program<'_>,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
) -> bool {
    let body = analyze_script_body(program, reactivity);
    body.has_effects || NeedsContextVisitor::check(program, scoping, reactivity)
}

pub(crate) fn analyze_script_body<'r>(
    program: &Program<'_>,
    reactivity: &'r ReactivitySemantics,
) -> ScriptBodyAnalyzer<'r> {
    let mut analyzer = ScriptBodyAnalyzer {
        reactivity,
        has_effects: false,
        has_class_state_fields: false,
        has_store_member_mutations: false,
    };
    analyzer.visit_program(program);
    analyzer
}

pub(crate) struct ScriptBodyAnalyzer<'r> {
    reactivity: &'r ReactivitySemantics,
    pub(crate) has_effects: bool,
    pub(crate) has_class_state_fields: bool,
    pub(crate) has_store_member_mutations: bool,
}

fn is_effect_call_fact(semantics: DeclaratorSemantics) -> bool {
    match semantics {
        DeclaratorSemantics::RuntimeRuneCall { kind } => match kind {
            RuntimeRuneKind::Effect | RuntimeRuneKind::EffectPre => true,
            RuntimeRuneKind::PropsId
            | RuntimeRuneKind::EffectTracking
            | RuntimeRuneKind::EffectPending
            | RuntimeRuneKind::Host
            | RuntimeRuneKind::InspectTrace
            | RuntimeRuneKind::EffectRoot
            | RuntimeRuneKind::Inspect
            | RuntimeRuneKind::InspectWith
            | RuntimeRuneKind::StateSnapshot
            | RuntimeRuneKind::StateEager
            | RuntimeRuneKind::Bindable => false,
        },
        DeclaratorSemantics::None
        | DeclaratorSemantics::RuneProps
        | DeclaratorSemantics::LegacyProps
        | DeclaratorSemantics::LegacyState
        | DeclaratorSemantics::RuneState { .. }
        | DeclaratorSemantics::RuneDerived { .. }
        | DeclaratorSemantics::ConstTag { .. }
        | DeclaratorSemantics::LetCarrier { .. }
        | DeclaratorSemantics::EachItem
        | DeclaratorSemantics::AwaitValue
        | DeclaratorSemantics::SnippetParam
        | DeclaratorSemantics::ClassFieldState(_)
        | DeclaratorSemantics::ClassFieldDerived(_) => false,
    }
}

fn is_class_field_state_fact(semantics: DeclaratorSemantics) -> bool {
    match semantics {
        DeclaratorSemantics::ClassFieldState(_) => true,
        DeclaratorSemantics::None
        | DeclaratorSemantics::RuntimeRuneCall { .. }
        | DeclaratorSemantics::RuneProps
        | DeclaratorSemantics::LegacyProps
        | DeclaratorSemantics::LegacyState
        | DeclaratorSemantics::RuneState { .. }
        | DeclaratorSemantics::RuneDerived { .. }
        | DeclaratorSemantics::ConstTag { .. }
        | DeclaratorSemantics::LetCarrier { .. }
        | DeclaratorSemantics::EachItem
        | DeclaratorSemantics::AwaitValue
        | DeclaratorSemantics::SnippetParam
        | DeclaratorSemantics::ClassFieldDerived(_) => false,
    }
}

impl<'a> Visit<'a> for ScriptBodyAnalyzer<'_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        let semantics = self.reactivity.declarator_semantics(call.node_id());
        if is_effect_call_fact(semantics) {
            self.has_effects = true;
        }
        walk_call_expression(self, call);
    }

    fn visit_assignment_expression(&mut self, assign: &AssignmentExpression<'a>) {
        if assignment_target_root_is_store(&assign.left) {
            self.has_store_member_mutations = true;
        }
        walk_assignment_expression(self, assign);
    }

    fn visit_update_expression(&mut self, upd: &UpdateExpression<'a>) {
        if simple_target_root_is_store(&upd.argument) {
            self.has_store_member_mutations = true;
        }
        walk_update_expression(self, upd);
    }

    fn visit_class(&mut self, class: &Class<'a>) {
        for element in &class.body.body {
            self.visit_class_element(element);
        }
    }

    fn visit_property_definition(&mut self, prop: &PropertyDefinition<'a>) {
        if is_class_field_state_fact(self.reactivity.declarator_semantics(prop.node_id())) {
            self.has_class_state_fields = true;
        }
    }

    fn visit_method_definition(&mut self, method: &MethodDefinition<'a>) {
        if method.kind != MethodDefinitionKind::Constructor {
            walk_method_definition(self, method);
            return;
        }
        let Some(body) = &method.value.body else {
            walk_method_definition(self, method);
            return;
        };
        for stmt in &body.statements {
            let Statement::ExpressionStatement(es) = stmt else {
                continue;
            };
            let Expression::AssignmentExpression(assign) = &es.expression else {
                continue;
            };
            if is_class_field_state_fact(self.reactivity.declarator_semantics(assign.node_id())) {
                self.has_class_state_fields = true;
            }
        }
        walk_method_definition(self, method);
    }
}

fn assignment_target_root_is_store(target: &AssignmentTarget<'_>) -> bool {
    let expr = match target {
        AssignmentTarget::StaticMemberExpression(m) => &m.object,
        AssignmentTarget::ComputedMemberExpression(m) => &m.object,
        _ => return false,
    };
    member_root_is_store(expr)
}

fn simple_target_root_is_store(target: &SimpleAssignmentTarget<'_>) -> bool {
    let expr = match target {
        SimpleAssignmentTarget::StaticMemberExpression(m) => &m.object,
        SimpleAssignmentTarget::ComputedMemberExpression(m) => &m.object,
        _ => return false,
    };
    member_root_is_store(expr)
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

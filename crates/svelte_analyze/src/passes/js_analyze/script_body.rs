use oxc_ast::ast::{
    AssignmentExpression, AssignmentTarget, CallExpression, Class, Expression, IdentifierReference,
    MemberExpression, MethodDefinition, MethodDefinitionKind, NewExpression, Program,
    PropertyDefinition, SimpleAssignmentTarget, Statement, UpdateExpression,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_assignment_expression, walk_call_expression, walk_member_expression,
    walk_method_definition, walk_new_expression, walk_update_expression,
};

use crate::reactivity_semantics::data::{
    BindingSemantics, DeclaratorSemantics, ReactivitySemantics, RuntimeRuneKind,
};
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{AnalysisData, JsAst};

pub(crate) fn analyze_script(data: &mut AnalysisData, parsed: &JsAst) {
    let mut observes_context = false;
    if let Some(program) = parsed.program.as_ref()
        && parsed.script_content_span.is_some()
    {
        let (class_fields, observed) = {
            let body = analyze_script_body(program, &data.scoping, &data.reactivity);
            (
                body.has_class_state_fields,
                body.has_effects || body.has_store_member_mutations || body.needs_context,
            )
        };
        data.script.has_class_state_fields = class_fields;
        observes_context |= observed;
    }
    if let Some(program) = parsed.module_program.as_ref()
        && parsed.module_script_content_span.is_some()
    {
        let observed = {
            let body = analyze_script_body(program, &data.scoping, &data.reactivity);
            body.has_effects || body.needs_context
        };
        observes_context |= observed;
    }
    data.script.observes_context = observes_context;
}

pub(crate) fn analyze_script_body<'r>(
    program: &Program<'_>,
    scoping: &'r ComponentScoping<'r>,
    reactivity: &'r ReactivitySemantics,
) -> ScriptBodyAnalyzer<'r> {
    let mut analyzer = ScriptBodyAnalyzer {
        scoping,
        reactivity,
        has_effects: false,
        has_class_state_fields: false,
        has_store_member_mutations: false,
        needs_context: false,
    };
    analyzer.visit_program(program);
    analyzer
}

pub(crate) struct ScriptBodyAnalyzer<'r> {
    scoping: &'r ComponentScoping<'r>,
    reactivity: &'r ReactivitySemantics,
    pub(crate) has_effects: bool,
    pub(crate) has_class_state_fields: bool,
    pub(crate) has_store_member_mutations: bool,
    pub(crate) needs_context: bool,
}

impl ScriptBodyAnalyzer<'_> {
    fn resolve_ref(&self, ident: &IdentifierReference<'_>) -> Option<SymbolId> {
        let ref_id = ident.reference_id.get()?;
        self.scoping.get_reference(ref_id).symbol_id()
    }

    fn is_safe_sym(&self, ident: &IdentifierReference<'_>) -> bool {
        if let Some(ref_id) = ident.reference_id.get() {
            let reference = self.reactivity.reference_semantics(ref_id);
            if reference.is_legacy_props_object_read() {
                return false;
            }
            if let Some(store) = reference.store_symbol() {
                return self.is_safe_binding(store);
            }
            if reference.is_store_subscription() {
                return false;
            }
        }
        let Some(sym_id) = self.resolve_ref(ident) else {
            return true;
        };
        self.is_safe_binding(sym_id)
    }

    fn is_safe_binding(&self, sym_id: SymbolId) -> bool {
        match self.reactivity.binding_semantics(sym_id) {
            BindingSemantics::MaybeReactive
            | BindingSemantics::Prop(_)
            | BindingSemantics::LegacyBindableProp(_) => false,
            BindingSemantics::Store(store) => self.is_safe_binding(store.base_symbol),
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::Unresolved => true,
        }
    }

    fn is_safe_expression_root(&self, expr: &Expression<'_>) -> bool {
        let mut node = expr.get_inner_expression();
        loop {
            match node {
                Expression::StaticMemberExpression(m) => node = m.object.get_inner_expression(),
                Expression::ComputedMemberExpression(m) => node = m.object.get_inner_expression(),
                _ => break,
            }
        }
        match node {
            Expression::Identifier(ident) => self.is_safe_sym(ident),
            _ => false,
        }
    }
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
    fn visit_new_expression(&mut self, it: &NewExpression<'a>) {
        self.needs_context = true;
        walk_new_expression(self, it);
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        let semantics = self.reactivity.declarator_semantics(call.node_id());
        if is_effect_call_fact(semantics) {
            self.has_effects = true;
        }
        if !self.is_safe_expression_root(&call.callee) {
            self.needs_context = true;
        }
        walk_call_expression(self, call);
    }

    fn visit_member_expression(&mut self, it: &MemberExpression<'a>) {
        let obj = match it {
            MemberExpression::StaticMemberExpression(m) => &m.object,
            MemberExpression::ComputedMemberExpression(m) => &m.object,
            _ => {
                walk_member_expression(self, it);
                return;
            }
        };
        if !self.is_safe_expression_root(obj) {
            self.needs_context = true;
        }
        walk_member_expression(self, it);
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

use std::mem::take;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{
    AssignmentExpression, AssignmentTarget, BindingPattern, CallExpression, Class, Declaration,
    Expression, MethodDefinition, MethodDefinitionKind, Program, PropertyDefinition,
    SimpleAssignmentTarget, Statement, UpdateExpression, VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_assignment_expression, walk_call_expression, walk_method_definition, walk_program,
    walk_update_expression,
};

use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::ComponentScoping;
use crate::types::data::{AnalysisData, ProxyStateInits};
use crate::types::script::{RuneKind, ScriptInfo};
use crate::utils::script_info::{detect_rune, detect_rune_from_call};

pub(crate) fn analyze_script(
    data: &mut AnalysisData,
    mut script_info: ScriptInfo,
    program: &Program<'_>,
) {
    let uses_runes = data.reactivity.uses_runes();
    let body = analyze_script_body(program, &script_info, uses_runes);
    let has_class_state_fields = body.has_class_state_fields;
    data.script.has_store_member_mutations = body.has_store_member_mutations;
    data.script.proxy_state_inits = body.proxy_state_inits;

    data.script.exports = take(&mut script_info.exports);
    data.script.has_class_state_fields = has_class_state_fields;
    data.script.info = Some(script_info);
}

pub(crate) fn needs_context_for_program(
    program: &Program<'_>,
    scoping: &ComponentScoping,
    reactivity: &ReactivitySemantics,
    script_info: &ScriptInfo,
    uses_runes: bool,
) -> bool {
    let body = analyze_script_body(program, script_info, uses_runes);
    body.has_effects
        || body.has_class_state_fields
        || super::needs_context::NeedsContextVisitor::check(program, scoping, reactivity)
}

pub(crate) fn analyze_script_body<'s>(
    program: &Program<'_>,
    script_info: &'s ScriptInfo,
    uses_runes: bool,
) -> ScriptBodyAnalyzer<'s> {
    let mut analyzer = ScriptBodyAnalyzer {
        uses_runes,
        has_effects: false,
        has_class_state_fields: false,
        has_store_member_mutations: false,
        proxy_state_inits: ProxyStateInits::new(),
        script_info,
    };
    analyzer.visit_program(program);
    analyzer
}

pub(crate) struct ScriptBodyAnalyzer<'s> {
    uses_runes: bool,
    pub(crate) has_effects: bool,
    pub(crate) has_class_state_fields: bool,
    pub(crate) has_store_member_mutations: bool,
    pub(crate) proxy_state_inits: ProxyStateInits,
    script_info: &'s ScriptInfo,
}

impl<'a> Visit<'a> for ScriptBodyAnalyzer<'_> {
    fn visit_program(&mut self, program: &Program<'a>) {
        for stmt in &program.body {
            match stmt {
                Statement::VariableDeclaration(decl) => {
                    self.check_proxy_state_inits(&decl.declarations);
                }
                Statement::ExportNamedDeclaration(export) => {
                    if let Some(Declaration::VariableDeclaration(d)) = &export.declaration {
                        self.check_proxy_state_inits(&d.declarations);
                    }
                }
                _ => {}
            }
        }
        walk_program(self, program);
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if self.uses_runes
            && matches!(
                detect_rune_from_call(call),
                Some(RuneKind::Effect | RuneKind::EffectPre)
            )
        {
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
        if let Some(value) = &prop.value
            && let Some(kind) = detect_rune(value)
            && matches!(kind, RuneKind::State | RuneKind::StateRaw)
        {
            self.has_class_state_fields = true;
        }
    }

    fn visit_method_definition(&mut self, method: &MethodDefinition<'a>) {
        if method.kind == MethodDefinitionKind::Constructor
            && let Some(body) = &method.value.body
        {
            for stmt in &body.statements {
                if let Statement::ExpressionStatement(es) = stmt
                    && let Expression::AssignmentExpression(assign) = &es.expression
                    && let Some(kind) = detect_rune(&assign.right)
                    && matches!(kind, RuneKind::State | RuneKind::StateRaw)
                {
                    self.has_class_state_fields = true;
                }
            }
        }
        walk_method_definition(self, method);
    }
}

impl ScriptBodyAnalyzer<'_> {
    fn check_proxy_state_inits(
        &mut self,
        declarations: &OxcVec<'_, VariableDeclarator<'_>>,
    ) {
        for declarator in declarations.iter() {
            let BindingPattern::BindingIdentifier(ident) = &declarator.id else {
                continue;
            };
            let Some(init) = &declarator.init else {
                continue;
            };
            let rune = detect_rune(init);
            if !matches!(rune, Some(RuneKind::State | RuneKind::StateRaw)) {
                continue;
            }
            let name = ident.name.as_str();
            if self.script_info.declarations.iter().any(|d| {
                d.name == name && matches!(d.is_rune, Some(RuneKind::State | RuneKind::StateRaw))
            }) && is_proxyable_state_init(init)
            {
                self.proxy_state_inits.set_proxied(name, true);
            }
        }
    }
}

fn is_proxyable_state_init(expr: &Expression<'_>) -> bool {
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
        return false;
    };
    let Some(arg) = call.arguments.first() else {
        return false;
    };
    let Some(e) = arg.as_expression().map(|e| e.get_inner_expression()) else {
        return false;
    };
    if e.is_literal() {
        return false;
    }
    if matches!(
        e,
        Expression::TemplateLiteral(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::BinaryExpression(_)
    ) {
        return false;
    }
    if let Expression::Identifier(id) = e
        && id.name == "undefined"
    {
        return false;
    }
    true
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

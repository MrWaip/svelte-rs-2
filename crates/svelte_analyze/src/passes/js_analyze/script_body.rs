use oxc_ast::ast::{Expression, MemberExpression, SimpleAssignmentTarget};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_member_expression, walk_simple_assignment_target, walk_update_expression,
};

use crate::scope::ComponentScoping;
use crate::types::data::{AnalysisData, ProxyStateInits};
use crate::types::script::{RuneKind, ScriptInfo};
use crate::utils::script_info::detect_rune_from_call;

pub(crate) fn analyze_script(
    data: &mut AnalysisData,
    mut script_info: ScriptInfo,
    program: &oxc_ast::ast::Program<'_>,
) {
    let uses_runes = data.reactivity.uses_runes();
    let body = analyze_script_body(program, &script_info, uses_runes);
    let has_class_state_fields = body.has_class_state_fields;
    data.script.has_store_member_mutations = body.has_store_member_mutations;
    data.script.proxy_state_inits = body.proxy_state_inits;

    data.script.exports = std::mem::take(&mut script_info.exports);
    data.script.has_class_state_fields = has_class_state_fields;
    data.script.info = Some(script_info);
}

pub(crate) fn needs_context_for_program(
    program: &oxc_ast::ast::Program<'_>,
    scoping: &ComponentScoping,
    script_info: &ScriptInfo,
    uses_runes: bool,
) -> bool {
    let body = analyze_script_body(program, script_info, uses_runes);
    body.has_effects
        || body.has_class_state_fields
        || super::needs_context::NeedsContextVisitor::check(program, scoping, script_info)
}

pub(crate) fn analyze_script_body<'s>(
    program: &oxc_ast::ast::Program<'_>,
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
    fn visit_program(&mut self, program: &oxc_ast::ast::Program<'a>) {
        for stmt in &program.body {
            self.visit_statement(stmt);
        }
    }

    fn visit_statement(&mut self, stmt: &oxc_ast::ast::Statement<'a>) {
        use oxc_ast::ast::Statement;

        match stmt {
            Statement::ExpressionStatement(es) => {
                if self.uses_runes
                    && let Expression::CallExpression(call) = &es.expression
                    && matches!(
                        detect_rune_from_call(call),
                        Some(RuneKind::Effect | RuneKind::EffectPre)
                    )
                {
                    self.has_effects = true;
                }
                if expression_has_store_member_mutation(&es.expression) {
                    self.has_store_member_mutations = true;
                }
            }
            Statement::ClassDeclaration(class) => {
                self.visit_class(class);
            }
            Statement::VariableDeclaration(decl) => {
                self.check_proxy_state_inits(&decl.declarations);
            }
            Statement::ExportNamedDeclaration(export) => {
                if let Some(oxc_ast::ast::Declaration::VariableDeclaration(d)) = &export.declaration
                {
                    self.check_proxy_state_inits(&d.declarations);
                }
            }
            _ => {}
        }
    }

    fn visit_class(&mut self, class: &oxc_ast::ast::Class<'a>) {
        for element in &class.body.body {
            self.visit_class_element(element);
        }
    }

    fn visit_property_definition(&mut self, prop: &oxc_ast::ast::PropertyDefinition<'a>) {
        if let Some(value) = &prop.value
            && let Some(kind) = crate::utils::script_info::detect_rune(value)
            && matches!(kind, RuneKind::State | RuneKind::StateRaw)
        {
            self.has_class_state_fields = true;
        }
    }

    fn visit_method_definition(&mut self, method: &oxc_ast::ast::MethodDefinition<'a>) {
        if method.kind != oxc_ast::ast::MethodDefinitionKind::Constructor {
            return;
        }
        let Some(body) = &method.value.body else {
            return;
        };
        for stmt in &body.statements {
            if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt
                && let Expression::AssignmentExpression(assign) = &es.expression
                && let Some(kind) = crate::utils::script_info::detect_rune(&assign.right)
                && matches!(kind, RuneKind::State | RuneKind::StateRaw)
            {
                self.has_class_state_fields = true;
            }
        }
    }
}

impl ScriptBodyAnalyzer<'_> {
    fn check_proxy_state_inits(
        &mut self,
        declarations: &oxc_allocator::Vec<'_, oxc_ast::ast::VariableDeclarator<'_>>,
    ) {
        for declarator in declarations.iter() {
            let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id else {
                continue;
            };
            let Some(init) = &declarator.init else {
                continue;
            };
            let rune = crate::utils::script_info::detect_rune(init);
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
    let Expression::CallExpression(call) = expr else {
        return false;
    };
    let Some(arg) = call.arguments.first() else {
        return false;
    };
    let Some(e) = arg.as_expression() else {
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

fn expression_has_store_member_mutation(expr: &Expression<'_>) -> bool {
    struct Probe {
        has: bool,
        in_write_position: bool,
    }
    impl<'a> Visit<'a> for Probe {
        fn visit_simple_assignment_target(&mut self, it: &SimpleAssignmentTarget<'a>) {
            self.in_write_position = true;
            walk_simple_assignment_target(self, it);
        }
        fn visit_update_expression(&mut self, upd: &oxc_ast::ast::UpdateExpression<'a>) {
            self.in_write_position = true;
            walk_update_expression(self, upd);
        }
        fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
            if self.in_write_position {
                let root_expr = match expr {
                    MemberExpression::StaticMemberExpression(m) => Some(&m.object),
                    MemberExpression::ComputedMemberExpression(m) => Some(&m.object),
                    _ => None,
                };
                if root_expr.is_some_and(member_root_is_store) {
                    self.has = true;
                }
            }
            self.in_write_position = false;
            walk_member_expression(self, expr);
        }
        fn visit_identifier_reference(
            &mut self,
            _ident: &oxc_ast::ast::IdentifierReference<'a>,
        ) {
            self.in_write_position = false;
        }
    }
    let mut p = Probe {
        has: false,
        in_write_position: false,
    };
    p.visit_expression(expr);
    p.has
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

use oxc_ast::ast::{
    AssignmentTarget, Class, ClassElement, Expression, MethodDefinitionKind, Program, PropertyKey,
    Statement,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_class;
use rustc_hash::FxHashSet;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::utils::script_info::detect_rune_from_call;

pub(super) fn validate(program: &Program<'_>, diags: &mut Vec<Diagnostic>) {
    let mut walker = ClassStateFieldsValidator { diags };
    walker.visit_program(program);
}

struct ClassStateFieldsValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
}

impl<'a> Visit<'a> for ClassStateFieldsValidator<'_> {
    fn visit_class(&mut self, class: &Class<'a>) {
        check_class(class, self.diags);
        walk_class(self, class);
    }
}

fn check_class<'a>(class: &Class<'a>, diags: &mut Vec<Diagnostic>) {
    let mut declared: FxHashSet<&'a str> = FxHashSet::default();

    for element in &class.body.body {
        let ClassElement::PropertyDefinition(prop) = element else {
            continue;
        };
        if prop.r#static || prop.computed {
            continue;
        }
        let PropertyKey::StaticIdentifier(name_id) = &prop.key else {
            continue;
        };
        let Some(value) = prop.value.as_ref() else {
            continue;
        };
        let Expression::CallExpression(call) = value else {
            continue;
        };
        if !is_state_creation_rune(detect_rune_from_call(call)) {
            continue;
        }
        declared.insert(name_id.name.as_str());
    }

    let Some(constructor) = class.body.body.iter().find_map(|el| match el {
        ClassElement::MethodDefinition(m) if m.kind == MethodDefinitionKind::Constructor => Some(m),
        _ => None,
    }) else {
        return;
    };
    let Some(body) = &constructor.value.body else {
        return;
    };
    for stmt in &body.statements {
        let Statement::ExpressionStatement(es) = stmt else {
            continue;
        };
        let Expression::AssignmentExpression(assign) = &es.expression else {
            continue;
        };
        let AssignmentTarget::StaticMemberExpression(member) = &assign.left else {
            continue;
        };
        if !matches!(&member.object, Expression::ThisExpression(_)) {
            continue;
        }
        let Expression::CallExpression(call) = &assign.right else {
            continue;
        };
        if !is_state_creation_rune(detect_rune_from_call(call)) {
            continue;
        }
        let name = member.property.name.as_str();
        if declared.contains(name) {
            diags.push(Diagnostic::error(
                DiagnosticKind::StateFieldDuplicate {
                    name: name.to_string(),
                },
                Span::new(assign.span.start, assign.span.end),
            ));
        }
    }
}

fn is_state_creation_rune(kind: Option<crate::types::script::RuneKind>) -> bool {
    use crate::types::script::RuneKind;
    matches!(
        kind,
        Some(RuneKind::State | RuneKind::StateRaw | RuneKind::Derived | RuneKind::DerivedBy)
    )
}

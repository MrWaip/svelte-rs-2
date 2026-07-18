use std::collections::hash_map::Entry;
use std::marker::PhantomData;

use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentExpression, AssignmentOperator, Class, ClassElement,
    Expression, Function, MemberExpression, MethodDefinition, MethodDefinitionKind, PropertyKey,
    Statement,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_assignment_expression;
use oxc_span::GetSpan;
use oxc_syntax::scope::ScopeFlags;
use rustc_hash::FxHashMap;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::reactivity_semantics::data::ReactivitySemantics;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FieldKind {
    Prop,
    AssignedProp,
    Method,
    Get,
    Set,
}

struct StateField {
    decl_start: u32,
    via_assignment: bool,
}

fn state_rune_name(value: &Expression<'_>) -> Option<&'static str> {
    let Expression::CallExpression(call) = value.get_inner_expression() else {
        return None;
    };
    match call.callee.get_inner_expression() {
        Expression::Identifier(id) => match id.name.as_str() {
            "$state" => Some("$state"),
            "$derived" => Some("$derived"),
            _ => None,
        },
        Expression::StaticMemberExpression(member) => {
            let Expression::Identifier(object) = member.object.get_inner_expression() else {
                return None;
            };
            match (object.name.as_str(), member.property.name.as_str()) {
                ("$state", "raw") => Some("$state.raw"),
                ("$derived", "by") => Some("$derived.by"),
                _ => None,
            }
        }
        _ => None,
    }
}

fn property_key_name(key: &PropertyKey<'_>) -> Option<String> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.to_string()),
        PropertyKey::PrivateIdentifier(id) => Some(format!("#{}", id.name)),
        PropertyKey::StringLiteral(lit) => Some(lit.value.to_string()),
        PropertyKey::NumericLiteral(lit) => Some(lit.value.to_string()),
        _ => None,
    }
}

fn this_member_name(member: &MemberExpression<'_>) -> Option<String> {
    match member {
        MemberExpression::StaticMemberExpression(m) => {
            if !matches!(
                m.object.get_inner_expression(),
                Expression::ThisExpression(_)
            ) {
                return None;
            }
            Some(m.property.name.to_string())
        }
        MemberExpression::PrivateFieldExpression(m) => {
            if !matches!(
                m.object.get_inner_expression(),
                Expression::ThisExpression(_)
            ) {
                return None;
            }
            Some(format!("#{}", m.field.name))
        }
        MemberExpression::ComputedMemberExpression(m) => {
            if !matches!(
                m.object.get_inner_expression(),
                Expression::ThisExpression(_)
            ) {
                return None;
            }
            match m.expression.get_inner_expression() {
                Expression::StringLiteral(lit) => Some(lit.value.to_string()),
                Expression::NumericLiteral(lit) => Some(lit.value.to_string()),
                _ => None,
            }
        }
    }
}

fn this_assignment<'a, 'b>(stmt: &'b Statement<'a>) -> Option<&'b AssignmentExpression<'a>> {
    let Statement::ExpressionStatement(es) = stmt else {
        return None;
    };
    let Expression::AssignmentExpression(assign) = es.expression.get_inner_expression() else {
        return None;
    };
    let member = assign.left.as_member_expression()?;
    matches!(
        member.object().get_inner_expression(),
        Expression::ThisExpression(_)
    )
    .then_some(assign)
}

pub(super) fn check_class<'a>(
    class: &Class<'a>,
    _reactivity: &ReactivitySemantics,
    diags: &mut Vec<Diagnostic>,
) {
    let mut state_fields: FxHashMap<String, StateField> = FxHashMap::default();
    let mut fields: FxHashMap<String, Vec<FieldKind>> = FxHashMap::default();
    let mut constructor: Option<&MethodDefinition<'a>> = None;

    for element in &class.body.body {
        match element {
            ClassElement::PropertyDefinition(prop) if !prop.computed && !prop.r#static => {
                let Some(name) = property_key_name(&prop.key) else {
                    continue;
                };
                if let Some(value) = &prop.value {
                    handle_state_field(
                        &mut state_fields,
                        &fields,
                        &name,
                        false,
                        false,
                        value,
                        Span::new(prop.span.start, prop.span.end),
                        diags,
                    );
                }
                let kind = if prop.value.is_some() {
                    FieldKind::AssignedProp
                } else {
                    FieldKind::Prop
                };
                match fields.entry(name) {
                    Entry::Occupied(entry) => {
                        diags.push(Diagnostic::error(
                            DiagnosticKind::DuplicateClassField {
                                name: entry.key().clone(),
                            },
                            Span::new(prop.span.start, prop.span.end),
                        ));
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![kind]);
                    }
                }
            }
            ClassElement::MethodDefinition(method) => {
                if method.kind == MethodDefinitionKind::Constructor {
                    constructor = Some(method);
                } else if !method.computed {
                    handle_method(&mut fields, method, diags);
                }
            }
            _ => {}
        }
    }

    if let Some(constructor) = constructor
        && let Some(body) = &constructor.value.body
    {
        for stmt in &body.statements {
            let Some(assign) = this_assignment(stmt) else {
                continue;
            };
            let member = assign
                .left
                .as_member_expression()
                .and_then(this_member_name);
            let Some(name) = member else {
                if let Some(rune) = state_rune_name(&assign.right) {
                    diags.push(Diagnostic::error(
                        DiagnosticKind::StateInvalidPlacement { rune: rune.into() },
                        state_call_span(&assign.right),
                    ));
                }
                continue;
            };
            handle_state_field(
                &mut state_fields,
                &fields,
                &name,
                false,
                true,
                &assign.right,
                Span::new(assign.span.start, assign.span.end),
                diags,
            );
        }

        let mut placement = ConstructorWalker {
            root_assign_starts: body
                .statements
                .iter()
                .filter_map(|stmt| this_assignment(stmt).map(|a| a.span.start))
                .collect(),
            state_fields: &state_fields,
            diags,
            _marker: PhantomData,
        };
        for stmt in &body.statements {
            placement.visit_statement(stmt);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_state_field(
    state_fields: &mut FxHashMap<String, StateField>,
    fields: &FxHashMap<String, Vec<FieldKind>>,
    name: &str,
    is_static: bool,
    via_assignment: bool,
    value: &Expression<'_>,
    node_span: Span,
    diags: &mut Vec<Diagnostic>,
) {
    if state_rune_name(value).is_none() {
        return;
    }
    if state_fields.contains_key(name) {
        diags.push(Diagnostic::error(
            DiagnosticKind::StateFieldDuplicate {
                name: name.to_string(),
            },
            node_span,
        ));
    }
    let key = if via_assignment || !is_static {
        name.to_string()
    } else {
        format!("@{name}")
    };
    if let Some(field) = fields.get(&key)
        && !(field.len() == 1 && field[0] == FieldKind::Prop)
    {
        diags.push(Diagnostic::error(
            DiagnosticKind::DuplicateClassField { name: key },
            node_span,
        ));
    }
    state_fields.entry(name.to_string()).or_insert(StateField {
        decl_start: node_span.start,
        via_assignment,
    });
}

fn handle_method(
    fields: &mut FxHashMap<String, Vec<FieldKind>>,
    method: &MethodDefinition<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(base) = property_key_name(&method.key) else {
        return;
    };
    let key = if method.r#static {
        format!("@{base}")
    } else {
        base
    };
    let kind = match method.kind {
        MethodDefinitionKind::Get => FieldKind::Get,
        MethodDefinitionKind::Set => FieldKind::Set,
        MethodDefinitionKind::Method => FieldKind::Method,
        MethodDefinitionKind::Constructor => return,
    };
    let span = Span::new(method.span.start, method.span.end);
    let Some(field) = fields.get_mut(&key) else {
        fields.insert(key, vec![kind]);
        return;
    };
    if field.contains(&kind)
        || field.contains(&FieldKind::Prop)
        || field.contains(&FieldKind::AssignedProp)
    {
        diags.push(Diagnostic::error(
            DiagnosticKind::DuplicateClassField { name: key },
            span,
        ));
        return;
    }
    if kind == FieldKind::Get && field.len() == 1 && field[0] == FieldKind::Set {
        field.push(FieldKind::Get);
        return;
    }
    if kind == FieldKind::Set && field.len() == 1 && field[0] == FieldKind::Get {
        field.push(FieldKind::Set);
        return;
    }
    if kind != FieldKind::Get && kind != FieldKind::Set {
        field.push(kind);
        return;
    }
    diags.push(Diagnostic::error(
        DiagnosticKind::DuplicateClassField { name: key },
        span,
    ));
}

fn state_call_span(value: &Expression<'_>) -> Span {
    if let Expression::CallExpression(call) = value.get_inner_expression() {
        return Span::new(call.span.start, call.span.end);
    }
    Span::new(value.span().start, value.span().end)
}

struct ConstructorWalker<'a, 'b> {
    root_assign_starts: rustc_hash::FxHashSet<u32>,
    state_fields: &'b FxHashMap<String, StateField>,
    diags: &'b mut Vec<Diagnostic>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Visit<'a> for ConstructorWalker<'a, '_> {
    fn visit_function(&mut self, _func: &Function<'a>, _flags: ScopeFlags) {}

    fn visit_arrow_function_expression(&mut self, _expr: &ArrowFunctionExpression<'a>) {}

    fn visit_class(&mut self, _class: &Class<'a>) {}

    fn visit_assignment_expression(&mut self, assign: &AssignmentExpression<'a>) {
        if let Some(member) = assign.left.as_member_expression() {
            let name = this_member_name(member);
            let is_root = self.root_assign_starts.contains(&assign.span.start);

            if let Some(rune) = state_rune_name(&assign.right)
                && !is_root
            {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::StateInvalidPlacement { rune: rune.into() },
                    state_call_span(&assign.right),
                ));
            }

            if let Some(name) = name
                && assign.operator == AssignmentOperator::Assign
                && let Some(field) = self.state_fields.get(&name)
                && field.via_assignment
                && field.decl_start != assign.span.start
                && assign.span.start < field.decl_start
            {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::StateFieldInvalidAssignment,
                    Span::new(assign.span.start, assign.span.end),
                ));
            }
        }
        walk_assignment_expression(self, assign);
    }
}

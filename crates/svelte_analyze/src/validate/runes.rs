//! Rune validation — placement, argument count, deprecated/removed runes.

use oxc_ast::ast::{
    AssignmentOperator, CallExpression, Expression,
    MethodDefinitionKind, PropertyDefinition, VariableDeclarator,
};
use oxc_semantic::NodeId as OxcNodeId;
use oxc_ast_visit::walk::{
    walk_assignment_expression, walk_call_expression, walk_method_definition,
    walk_property_definition,
};
use oxc_ast_visit::Visit;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::{AnalysisData, types::script::RuneKind};
use crate::utils::script_info::detect_rune_from_call;

/// Constructor assignments to `this` are valid rune placement targets,
/// same as variable declarations and class property initializers.
fn is_this_member_assign(target: &oxc_ast::ast::AssignmentTarget<'_>) -> bool {
    let object = match target {
        oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) => &m.object,
        oxc_ast::ast::AssignmentTarget::PrivateFieldExpression(m) => &m.object,
        oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) => &m.object,
        _ => return false,
    };
    matches!(object, Expression::ThisExpression(_))
}

pub(super) fn validate(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = RuneValidator {
        diags,
        offset,
        in_var_declarator_init: false,
        in_class_property_init: false,
        in_constructor_body: false,
        in_this_assign_rhs: false,
    };
    v.visit_program(program);
    validate_derived_invalid_export(data, program, offset, diags);
    validate_state_referenced_locally_derived(data, diags);
}

struct RuneValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    offset: u32,
    in_var_declarator_init: bool,
    in_class_property_init: bool,
    in_constructor_body: bool,
    /// RHS of `this.x = ...` inside a constructor — valid rune placement.
    in_this_assign_rhs: bool,
}

impl RuneValidator<'_> {
    fn span(&self, oxc: oxc_span::Span) -> Span {
        Span::new(oxc.start + self.offset, oxc.end + self.offset)
    }

    /// `detect_rune_from_call` only matches known rune names — deprecated forms like
    /// `$state.frozen(...)` are not recognized, so they must be intercepted here first.
    fn check_deprecated_rune(&mut self, call: &CallExpression<'_>) -> bool {
        let Expression::StaticMemberExpression(member) = &call.callee else { return false };
        let Expression::Identifier(obj) = &member.object else { return false };
        if obj.name != "$state" {
            return false;
        }
        let prop = member.property.name.as_str();
        match prop {
            "frozen" => {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::RuneRenamed {
                        name: "$state.frozen".into(),
                        replacement: "$state.raw".into(),
                    },
                    self.span(call.span),
                ));
                true
            }
            "is" => {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::RuneRemoved { name: "$state.is".into() },
                    self.span(call.span),
                ));
                true
            }
            _ => false,
        }
    }

}

fn validate_derived_invalid_export(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let root = data.scoping.root_scope_id();
    for stmt in &program.body {
        let oxc_ast::ast::Statement::ExportNamedDeclaration(export) = stmt else {
            continue;
        };
        let mut has_derived = false;
        for spec in &export.specifiers {
            let local = spec.local.name().as_str();
            let is_derived = data
                .scoping
                .find_binding(root, local)
                .and_then(|sym_id| data.scoping.rune_kind(sym_id))
                .is_some_and(|kind| kind.is_derived());
            if is_derived {
                has_derived = true;
                break;
            }
        }
        if !has_derived {
            if let Some(decl) = &export.declaration {
                if let oxc_ast::ast::Declaration::VariableDeclaration(var_decl) = decl {
                    has_derived = var_decl.declarations.iter().any(|declarator| {
                        let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id else {
                            return false;
                        };
                        data.scoping
                            .find_binding(root, ident.name.as_str())
                            .and_then(|sym_id| data.scoping.rune_kind(sym_id))
                            .is_some_and(|kind| kind.is_derived())
                    });
                }
            }
        }
        if has_derived {
            diags.push(Diagnostic::error(
                DiagnosticKind::DerivedInvalidExport,
                Span::new(export.span.start + offset, export.span.end + offset),
            ));
        }
    }
}

fn validate_state_referenced_locally_derived(data: &AnalysisData, diags: &mut Vec<Diagnostic>) {
    let Some(script) = &data.script else { return };
    for (sym_id, rune_kind) in data.scoping.rune_symbols() {
        if !rune_kind.is_derived() {
            continue;
        }
        let decl_depth = data.scoping.function_depth(data.scoping.symbol_scope_id(sym_id));
        let has_same_depth_read = data
            .scoping
            .resolved_references(sym_id)
            .any(|reference| {
                reference.is_read()
                    && reference.node_id() != OxcNodeId::DUMMY
                    && data.scoping.function_depth(reference.scope_id()) == decl_depth
            });
        if has_same_depth_read {
            let name = data.scoping.symbol_name(sym_id);
            let span = script
                .declarations
                .iter()
                .find(|decl| decl.name.as_str() == name)
                .map(|decl| decl.span)
                .unwrap_or(Span::new(0, 0));
            diags.push(Diagnostic::warning(
                DiagnosticKind::StateReferencedLocally {
                    name: name.to_string(),
                    type_: "closure".into(),
                },
                span,
            ));
        }
    }
}

impl<'a> Visit<'a> for RuneValidator<'_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if self.check_deprecated_rune(call) {
            return;
        }

        let Some(rune) = detect_rune_from_call(call) else {
            walk_call_expression(self, call);
            return;
        };

        if matches!(rune, RuneKind::State | RuneKind::StateRaw | RuneKind::Derived | RuneKind::DerivedBy) {
            let valid = self.in_var_declarator_init
                || self.in_class_property_init
                || self.in_this_assign_rhs;
            if !valid {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::StateInvalidPlacement {
                        rune: rune.display_name().into(),
                    },
                    self.span(call.span),
                ));
            }
        }

        let arg_violation = match rune {
            RuneKind::Derived | RuneKind::DerivedBy | RuneKind::StateEager
                if call.arguments.len() != 1 =>
            {
                Some("exactly one argument")
            }
            RuneKind::State | RuneKind::StateRaw if call.arguments.len() > 1 => {
                Some("zero or one arguments")
            }
            _ => None,
        };
        if let Some(args) = arg_violation {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::RuneInvalidArgumentsLength {
                    rune: rune.display_name().into(),
                    args: args.into(),
                },
                self.span(call.span),
            ));
        }

        walk_call_expression(self, call);
    }

    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        self.visit_binding_pattern(&it.id);
        if let Some(init) = &it.init {
            let prev = self.in_var_declarator_init;
            self.in_var_declarator_init = true;
            self.visit_expression(init);
            self.in_var_declarator_init = prev;
        }
    }

    fn visit_property_definition(&mut self, it: &PropertyDefinition<'a>) {
        if it.r#static || it.computed {
            walk_property_definition(self, it);
            return;
        }
        self.visit_property_key(&it.key);
        if let Some(value) = &it.value {
            let prev = self.in_class_property_init;
            self.in_class_property_init = true;
            self.visit_expression(value);
            self.in_class_property_init = prev;
        }
    }

    fn visit_method_definition(&mut self, it: &oxc_ast::ast::MethodDefinition<'a>) {
        let prev = self.in_constructor_body;
        if it.kind == MethodDefinitionKind::Constructor {
            self.in_constructor_body = true;
        }
        walk_method_definition(self, it);
        self.in_constructor_body = prev;
    }

    fn visit_assignment_expression(&mut self, it: &oxc_ast::ast::AssignmentExpression<'a>) {
        if self.in_constructor_body
            && it.operator == AssignmentOperator::Assign
            && is_this_member_assign(&it.left)
        {
            let prev = self.in_this_assign_rhs;
            self.in_this_assign_rhs = true;
            self.visit_expression(&it.right);
            self.in_this_assign_rhs = prev;
        } else {
            walk_assignment_expression(self, it);
        }
    }
}

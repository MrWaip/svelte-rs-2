//! Rune validation — placement, argument count, deprecated/removed runes.

use oxc_ast::ast::{
    AssignmentOperator, CallExpression, Expression,
    MethodDefinitionKind, PropertyDefinition, VariableDeclarator,
};
use oxc_ast_visit::walk::{
    walk_assignment_expression, walk_call_expression, walk_method_definition,
    walk_property_definition,
};
use oxc_ast_visit::Visit;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::types::script::RuneKind;
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

pub(super) fn validate(program: &oxc_ast::ast::Program<'_>, offset: u32, diags: &mut Vec<Diagnostic>) {
    let mut v = RuneValidator {
        diags,
        offset,
        in_var_declarator_init: false,
        in_class_property_init: false,
        in_constructor_body: false,
        in_this_assign_rhs: false,
    };
    v.visit_program(program);
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

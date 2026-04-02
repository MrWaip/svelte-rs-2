//! Rune validation — placement, argument count, deprecated/removed runes.

use oxc_ast::ast::{
    AssignmentOperator, CallExpression, Expression,
    MethodDefinitionKind, PropertyDefinition, VariableDeclarator,
};
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_expression, walk_call_expression,
    walk_function, walk_method_definition, walk_property_definition,
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
    validate_state_invalid_export(data, program, offset, diags);
    validate_state_referenced_locally_derived(data, program, offset, diags);
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
    for stmt in &program.body {
        let oxc_ast::ast::Statement::ExportNamedDeclaration(export) = stmt else { continue };
        let Some(oxc_ast::ast::Declaration::VariableDeclaration(var_decl)) = &export.declaration else { continue };
        let has_derived = var_decl.declarations.iter().any(|declarator| {
            let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id else {
                return false;
            };
            ident.symbol_id.get()
                .and_then(|sym_id| data.scoping.rune_kind(sym_id))
                .is_some_and(|kind| kind.is_derived())
        });
        if has_derived {
            diags.push(Diagnostic::error(
                DiagnosticKind::DerivedInvalidExport,
                Span::new(export.span.start + offset, export.span.end + offset),
            ));
        }
    }
}

fn validate_state_invalid_export(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    for stmt in &program.body {
        let oxc_ast::ast::Statement::ExportNamedDeclaration(export) = stmt else { continue };
        let Some(oxc_ast::ast::Declaration::VariableDeclaration(var_decl)) = &export.declaration else { continue };
        let has_reassigned_state = var_decl.declarations.iter().any(|declarator| {
            let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id else {
                return false;
            };
            let Some(sym_id) = ident.symbol_id.get() else { return false };
            data.scoping.rune_kind(sym_id)
                .is_some_and(|k| matches!(k, RuneKind::State | RuneKind::StateRaw))
                && data.scoping.is_mutated(sym_id)
        });
        if has_reassigned_state {
            diags.push(Diagnostic::error(
                DiagnosticKind::StateInvalidExport,
                Span::new(export.span.start + offset, export.span.end + offset),
            ));
        }
    }
}

fn validate_state_referenced_locally_derived(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = StateRefLocallyValidator { data, offset, diags, in_state_rune_arg: false, derived_call_depth: 0, _phantom: std::marker::PhantomData };
    v.visit_program(program);
}

struct StateRefLocallyValidator<'a, 'b> {
    data: &'b AnalysisData,
    offset: u32,
    diags: &'b mut Vec<Diagnostic>,
    /// True when currently inside arguments of a `$state`/`$state.raw` call,
    /// without a function boundary in between. Determines `type_` in the diagnostic.
    in_state_rune_arg: bool,
    /// Incremented when entering `$derived`/`$derived.by` call arguments.
    /// Mirrors the reference compiler's `function_depth += 1` for `$derived` calls:
    /// references inside `$derived(...)` are semantically deeper and should not warn.
    derived_call_depth: usize,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Visit<'a> for StateRefLocallyValidator<'a, '_> {
    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else { return };
        if self.data.scoping.is_template_reference(ref_id) { return }
        let reference = self.data.scoping.get_reference(ref_id);
        // Skip if not a read, or if it's a write context (UpdateExpression, compound assignment LHS).
        // Mirrors reference compiler: only warn for pure reads, not read-write operations.
        if !reference.is_read() || reference.is_write() { return }
        let Some(sym_id) = reference.symbol_id() else { return };
        let should_warn = match self.data.scoping.rune_kind(sym_id) {
            Some(k) if k.is_derived() => true,
            Some(RuneKind::StateRaw) => true,
            Some(RuneKind::State) => {
                self.data.scoping.is_mutated(sym_id)
                    || !self.data.scoping.is_proxy_init_state(sym_id)
            }
            _ => false,
        };
        if !should_warn { return }
        let decl_depth = self.data.scoping.function_depth(self.data.scoping.symbol_scope_id(sym_id));
        // Add derived_call_depth to mirror the reference compiler's function_depth += 1 for $derived:
        // references inside $derived(...) are semantically one level deeper and should not warn.
        let ref_depth = self.data.scoping.function_depth(reference.scope_id()) + self.derived_call_depth;
        if ref_depth != decl_depth { return }
        let name = self.data.scoping.symbol_name(sym_id);
        let type_ = if self.in_state_rune_arg { "derived" } else { "closure" };
        self.diags.push(Diagnostic::warning(
            DiagnosticKind::StateReferencedLocally {
                name: name.to_string(),
                type_: type_.into(),
            },
            Span::new(ident.span.start + self.offset, ident.span.end + self.offset),
        ));
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        match detect_rune_from_call(call) {
            Some(k) if matches!(k, RuneKind::State | RuneKind::StateRaw) => {
                self.visit_expression(&call.callee);
                let prev = std::mem::replace(&mut self.in_state_rune_arg, true);
                for arg in &call.arguments {
                    self.visit_argument(arg);
                }
                self.in_state_rune_arg = prev;
            }
            Some(k) if k.is_derived() => {
                self.visit_expression(&call.callee);
                self.derived_call_depth += 1;
                for arg in &call.arguments {
                    self.visit_argument(arg);
                }
                self.derived_call_depth -= 1;
            }
            _ => walk_call_expression(self, call),
        }
    }

    fn visit_arrow_function_expression(&mut self, arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
        let prev_state_arg = std::mem::replace(&mut self.in_state_rune_arg, false);
        let prev_derived_depth = std::mem::replace(&mut self.derived_call_depth, 0);
        walk_arrow_function_expression(self, arrow);
        self.in_state_rune_arg = prev_state_arg;
        self.derived_call_depth = prev_derived_depth;
    }

    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, flags: oxc_semantic::ScopeFlags) {
        let prev_state_arg = std::mem::replace(&mut self.in_state_rune_arg, false);
        let prev_derived_depth = std::mem::replace(&mut self.derived_call_depth, 0);
        walk_function(self, func, flags);
        self.in_state_rune_arg = prev_state_arg;
        self.derived_call_depth = prev_derived_depth;
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

//! Rune validation — placement, argument count, deprecated/removed runes.

use oxc_ast::ast::{
    AssignmentOperator, BindingPattern, CallExpression, ExportDefaultDeclaration,
    ExportNamedDeclaration, Expression, ExpressionStatement, MemberExpression,
    MethodDefinitionKind, ModuleExportName, PropertyDefinition, VariableDeclarator,
};
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_expression, walk_call_expression,
    walk_expression_statement, walk_function, walk_member_expression, walk_method_definition,
    walk_property_definition,
};
use oxc_ast_visit::Visit;
use oxc_span::GetSpan;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::utils::script_info::{detect_rune, detect_rune_from_call};
use crate::{types::script::RuneKind, AnalysisData};

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
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = RuneValidator::new(data, diags, offset, runes, true);
    v.visit_program(program);
    validate_derived_invalid_export(data, program, offset, diags);
    validate_state_invalid_export(data, program, offset, diags);
    validate_state_referenced_locally_derived(data, program, offset, diags);
    validate_rest_prop_illegal_access(data, program, offset, diags);
}

pub(super) fn validate_module_props_runes(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = RuneValidator::new(data, diags, offset, runes, false);
    v.visit_program(program);
}

struct RuneValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    offset: u32,
    runes: bool,
    in_var_declarator_init: bool,
    in_class_property_init: bool,
    in_constructor_body: bool,
    /// RHS of `this.x = ...` inside a constructor — valid rune placement.
    in_this_assign_rhs: bool,
    /// True only when we are visiting the direct expression of an ExpressionStatement.
    /// Reset to false whenever we descend into a nested call expression.
    in_expression_statement_expr: bool,
    /// Span of the ExpressionStatement currently being visited, if any.
    current_expr_stmt_span: Option<oxc_span::Span>,
    /// Span of the first statement of the nearest enclosing function body.
    /// None when not inside any function.
    fn_body_first_stmt_span: Option<oxc_span::Span>,
    /// True when currently inside a generator function.
    in_generator: bool,
    /// 0 = top-level scope, incremented inside functions/arrows.
    function_depth: u32,
    /// Duplicate `$props()` detection.
    has_props_rune: bool,
    /// Duplicate `$props.id()` detection.
    has_props_id: bool,
    /// True when visiting the binding pattern of a `$props()` destructure.
    /// Used for `$bindable()` placement validation.
    in_props_destructure: bool,
    /// True for `<script>`, false for `<script module>`.
    is_instance_script: bool,
    custom_element: bool,
}

impl RuneValidator<'_> {
    fn new<'a>(
        data: &AnalysisData,
        diags: &'a mut Vec<Diagnostic>,
        offset: u32,
        runes: bool,
        is_instance_script: bool,
    ) -> RuneValidator<'a> {
        RuneValidator {
            diags,
            offset,
            runes,
            in_var_declarator_init: false,
            in_class_property_init: false,
            in_constructor_body: false,
            in_this_assign_rhs: false,
            in_expression_statement_expr: false,
            current_expr_stmt_span: None,
            fn_body_first_stmt_span: None,
            in_generator: false,
            function_depth: 0,
            has_props_rune: false,
            has_props_id: false,
            in_props_destructure: false,
            is_instance_script,
            custom_element: data.output.custom_element,
        }
    }

    fn span(&self, oxc: oxc_span::Span) -> Span {
        Span::new(oxc.start + self.offset, oxc.end + self.offset)
    }

    /// Validate the binding pattern of a `$props()` declaration.
    /// Rejects computed keys, `$$`-prefixed names, and nested destructures.
    fn validate_props_pattern(&mut self, pattern: &BindingPattern<'_>) {
        let BindingPattern::ObjectPattern(obj) = pattern else {
            if !matches!(pattern, BindingPattern::BindingIdentifier(_)) {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsInvalidIdentifier,
                    self.span(pattern.span()),
                ));
            }
            return;
        };

        for prop in &obj.properties {
            if prop.computed {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsInvalidPattern,
                    self.span(prop.span),
                ));
                continue;
            }

            // Reject `$$`-prefixed property names.
            if let oxc_ast::ast::PropertyKey::StaticIdentifier(key) = &prop.key {
                if key.name.starts_with("$$") {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::PropsIllegalName,
                        self.span(prop.span),
                    ));
                }
            }

            // The value (after stripping AssignmentPattern default) must be a plain identifier.
            let value_pattern = match &prop.value {
                BindingPattern::AssignmentPattern(assign) => &assign.left,
                other => other,
            };
            if !matches!(value_pattern, BindingPattern::BindingIdentifier(_)) {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsInvalidPattern,
                    self.span(prop.span),
                ));
            }
        }
    }

    /// `detect_rune_from_call` only matches known rune names — deprecated forms like
    /// `$state.frozen(...)` are not recognized, so they must be intercepted here first.
    fn check_deprecated_rune(&mut self, call: &CallExpression<'_>) -> bool {
        let Expression::StaticMemberExpression(member) = &call.callee else {
            return false;
        };
        let Expression::Identifier(obj) = &member.object else {
            return false;
        };
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
                    DiagnosticKind::RuneRemoved {
                        name: "$state.is".into(),
                    },
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
    validate_invalid_export(
        data,
        program,
        offset,
        diags,
        || DiagnosticKind::DerivedInvalidExport,
        |data, sym_id| {
            data.scoping
                .rune_kind(sym_id)
                .is_some_and(|kind| kind.is_derived())
        },
    );
}

fn validate_state_invalid_export(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    validate_invalid_export(
        data,
        program,
        offset,
        diags,
        || DiagnosticKind::StateInvalidExport,
        is_reassigned_state_export,
    );
}

fn validate_invalid_export(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
    make_kind: impl Fn() -> DiagnosticKind + Copy,
    predicate: impl Fn(&AnalysisData, oxc_semantic::SymbolId) -> bool + Copy,
) {
    for stmt in &program.body {
        match stmt {
            oxc_ast::ast::Statement::ExportNamedDeclaration(export) => {
                validate_invalid_named_export(data, export, offset, diags, make_kind, predicate);
            }
            oxc_ast::ast::Statement::ExportDefaultDeclaration(export) => {
                validate_invalid_default_export(data, export, offset, diags, make_kind, predicate);
            }
            _ => {}
        }
    }
}

fn validate_invalid_named_export(
    data: &AnalysisData,
    export: &ExportNamedDeclaration<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
    make_kind: impl Fn() -> DiagnosticKind + Copy,
    predicate: impl Fn(&AnalysisData, oxc_semantic::SymbolId) -> bool + Copy,
) {
    let has_invalid_export = export
        .declaration
        .as_ref()
        .is_some_and(|decl| declaration_has_invalid_export(data, decl, predicate))
        || export
            .specifiers
            .iter()
            .filter_map(|spec| export_specifier_symbol(data, spec))
            .any(|sym_id| predicate(data, sym_id));

    if has_invalid_export {
        diags.push(Diagnostic::error(
            make_kind(),
            Span::new(export.span.start + offset, export.span.end + offset),
        ));
    }
}

fn validate_invalid_default_export(
    data: &AnalysisData,
    export: &ExportDefaultDeclaration<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
    make_kind: impl Fn() -> DiagnosticKind + Copy,
    predicate: impl Fn(&AnalysisData, oxc_semantic::SymbolId) -> bool + Copy,
) {
    let oxc_ast::ast::ExportDefaultDeclarationKind::Identifier(ident) = &export.declaration else {
        return;
    };
    let Some(sym_id) =
        resolve_root_identifier_symbol(data, ident.name.as_str(), ident.reference_id.get())
    else {
        return;
    };
    if predicate(data, sym_id) {
        diags.push(Diagnostic::error(
            make_kind(),
            Span::new(export.span.start + offset, export.span.end + offset),
        ));
    }
}

fn declaration_has_invalid_export(
    data: &AnalysisData,
    decl: &oxc_ast::ast::Declaration<'_>,
    predicate: impl Fn(&AnalysisData, oxc_semantic::SymbolId) -> bool + Copy,
) -> bool {
    let oxc_ast::ast::Declaration::VariableDeclaration(var_decl) = decl else {
        return false;
    };
    var_decl.declarations.iter().any(|declarator| {
        let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id else {
            return false;
        };
        ident
            .symbol_id
            .get()
            .is_some_and(|sym_id| predicate(data, sym_id))
    })
}

fn export_specifier_symbol(
    data: &AnalysisData,
    spec: &oxc_ast::ast::ExportSpecifier<'_>,
) -> Option<oxc_semantic::SymbolId> {
    let ModuleExportName::IdentifierReference(ident) = &spec.local else {
        return None;
    };
    resolve_root_identifier_symbol(data, ident.name.as_str(), ident.reference_id.get())
}

fn resolve_root_identifier_symbol(
    data: &AnalysisData,
    name: &str,
    ref_id: Option<oxc_semantic::ReferenceId>,
) -> Option<oxc_semantic::SymbolId> {
    ref_id
        .and_then(|ref_id| data.scoping.try_get_reference(ref_id))
        .and_then(|reference| reference.symbol_id())
        .or_else(|| {
            data.scoping
                .find_binding(data.scoping.root_scope_id(), name)
        })
}

fn is_reassigned_state_export(data: &AnalysisData, sym_id: oxc_semantic::SymbolId) -> bool {
    data.scoping
        .rune_kind(sym_id)
        .is_some_and(|k| matches!(k, RuneKind::State | RuneKind::StateRaw))
        && data.scoping.is_mutated(sym_id)
}

fn validate_state_referenced_locally_derived(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = StateRefLocallyValidator {
        data,
        offset,
        diags,
        in_state_rune_arg: false,
        call_depth_offset: 0,
        _phantom: std::marker::PhantomData,
    };
    v.visit_program(program);
}

struct StateRefLocallyValidator<'a, 'b> {
    data: &'b AnalysisData,
    offset: u32,
    diags: &'b mut Vec<Diagnostic>,
    /// True when currently inside arguments of a `$state`/`$state.raw` call,
    /// without a function boundary in between. Determines `type_` in the diagnostic.
    in_state_rune_arg: bool,
    /// Incremented when entering call arguments that the reference compiler treats
    /// as one function-depth deeper for `state_referenced_locally`.
    call_depth_offset: u32,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Visit<'a> for StateRefLocallyValidator<'a, '_> {
    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        if self.data.scoping.is_template_reference(ref_id) {
            return;
        }
        let Some(reference) = self.data.scoping.try_get_reference(ref_id) else {
            return;
        };
        // Skip if not a read, or if it's a write context (UpdateExpression, compound assignment LHS).
        // Mirrors reference compiler: only warn for pure reads, not read-write operations.
        if !reference.is_read() || reference.is_write() {
            return;
        }
        let Some(sym_id) = reference.symbol_id() else {
            return;
        };
        let should_warn = match self.data.scoping.rune_kind(sym_id) {
            Some(k) if k.is_derived() => true,
            Some(RuneKind::StateRaw) => true,
            Some(RuneKind::State) => {
                self.data.scoping.is_mutated(sym_id)
                    || !self.data.scoping.is_proxy_init_state(sym_id)
            }
            _ => false,
        };
        if !should_warn {
            return;
        }
        let decl_depth = self
            .data
            .scoping
            .function_depth(self.data.scoping.symbol_scope_id(sym_id));
        // `$derived(...)`, `$derived.by(...)`, and `$inspect(...)` arguments are analyzed
        // one function-depth deeper by the reference compiler for this warning.
        let ref_depth =
            self.data.scoping.function_depth(reference.scope_id()) + self.call_depth_offset;
        if ref_depth != decl_depth {
            return;
        }
        let name = self.data.scoping.symbol_name(sym_id);
        let type_ = if self.in_state_rune_arg {
            "derived"
        } else {
            "closure"
        };
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
                self.call_depth_offset += 1;
                for arg in &call.arguments {
                    self.visit_argument(arg);
                }
                self.call_depth_offset -= 1;
            }
            Some(RuneKind::Inspect) => {
                self.visit_expression(&call.callee);
                self.call_depth_offset += 1;
                for arg in &call.arguments {
                    self.visit_argument(arg);
                }
                self.call_depth_offset -= 1;
            }
            _ => walk_call_expression(self, call),
        }
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        let prev_state_arg = std::mem::replace(&mut self.in_state_rune_arg, false);
        let prev_call_depth = std::mem::replace(&mut self.call_depth_offset, 0);
        walk_arrow_function_expression(self, arrow);
        self.in_state_rune_arg = prev_state_arg;
        self.call_depth_offset = prev_call_depth;
    }

    fn visit_function(
        &mut self,
        func: &oxc_ast::ast::Function<'a>,
        flags: oxc_semantic::ScopeFlags,
    ) {
        let prev_state_arg = std::mem::replace(&mut self.in_state_rune_arg, false);
        let prev_call_depth = std::mem::replace(&mut self.call_depth_offset, 0);
        walk_function(self, func, flags);
        self.in_state_rune_arg = prev_state_arg;
        self.call_depth_offset = prev_call_depth;
    }
}

impl<'a> Visit<'a> for RuneValidator<'_> {
    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement<'a>) {
        let prev = std::mem::replace(&mut self.in_expression_statement_expr, true);
        let prev_span = std::mem::replace(&mut self.current_expr_stmt_span, Some(stmt.span));
        walk_expression_statement(self, stmt);
        self.in_expression_statement_expr = prev;
        self.current_expr_stmt_span = prev_span;
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        // Capture whether this call is the direct expression of an ExpressionStatement,
        // then reset for children — nested calls are never in statement position.
        let is_expr_stmt = std::mem::replace(&mut self.in_expression_statement_expr, false);

        if self.check_deprecated_rune(call) {
            return;
        }

        let Some(rune) = detect_rune_from_call(call) else {
            walk_call_expression(self, call);
            return;
        };

        if !self.is_instance_script {
            match rune {
                RuneKind::Props => self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsInvalidPlacement,
                    self.span(call.span),
                )),
                RuneKind::PropsId => self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsIdInvalidPlacement,
                    self.span(call.span),
                )),
                _ => {}
            }
            walk_call_expression(self, call);
            return;
        }

        if matches!(
            rune,
            RuneKind::State | RuneKind::StateRaw | RuneKind::Derived | RuneKind::DerivedBy
        ) {
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

        if matches!(rune, RuneKind::Effect | RuneKind::EffectPre) && !is_expr_stmt {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::EffectInvalidPlacement,
                self.span(call.span),
            ));
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
            RuneKind::Effect | RuneKind::EffectPre | RuneKind::EffectRoot
                if call.arguments.len() != 1 =>
            {
                Some("exactly one argument")
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

        if matches!(rune, RuneKind::EffectTracking) && !call.arguments.is_empty() {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::RuneInvalidArguments {
                    rune: rune.display_name().into(),
                },
                self.span(call.span),
            ));
        }

        // --- $inspect validation ---
        if matches!(rune, RuneKind::Inspect) && call.arguments.is_empty() {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::RuneInvalidArgumentsLength {
                    rune: rune.display_name().into(),
                    args: "one or more arguments".into(),
                },
                self.span(call.span),
            ));
        }

        if matches!(rune, RuneKind::InspectWith) && call.arguments.len() != 1 {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::RuneInvalidArgumentsLength {
                    rune: rune.display_name().into(),
                    args: "exactly one argument".into(),
                },
                self.span(call.span),
            ));
        }

        if matches!(rune, RuneKind::InspectTrace) {
            if call.arguments.len() > 1 {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::RuneInvalidArgumentsLength {
                        rune: rune.display_name().into(),
                        args: "zero or one arguments".into(),
                    },
                    self.span(call.span),
                ));
            }

            // Must be first statement of a direct function body block.
            // Use `is_expr_stmt` (captured before the reset) not `self.in_expression_statement_expr`.
            let is_valid_placement = is_expr_stmt
                && self
                    .fn_body_first_stmt_span
                    .zip(self.current_expr_stmt_span)
                    .is_some_and(|(first, current)| first == current);
            if !is_valid_placement {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::InspectTraceInvalidPlacement,
                    self.span(call.span),
                ));
            }

            if self.in_generator {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::InspectTraceGenerator,
                    self.span(call.span),
                ));
            }
        }

        // --- $host validation ---
        if matches!(rune, RuneKind::Host) {
            if !call.arguments.is_empty() {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::RuneInvalidArguments {
                        rune: rune.display_name().into(),
                    },
                    self.span(call.span),
                ));
            } else if !self.custom_element {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::HostInvalidPlacement,
                    self.span(call.span),
                ));
            }
        }

        // --- $bindable validation ---
        if matches!(rune, RuneKind::Bindable) {
            if call.arguments.len() > 1 {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::RuneInvalidArgumentsLength {
                        rune: rune.display_name().into(),
                        args: "zero or one arguments".into(),
                    },
                    self.span(call.span),
                ));
            }
            if !self.in_props_destructure {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::BindableInvalidLocation,
                    self.span(call.span),
                ));
            }
        }

        // --- $props validation ---
        if matches!(rune, RuneKind::Props) {
            if self.has_props_rune {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsDuplicate {
                        rune: rune.display_name().into(),
                    },
                    self.span(call.span),
                ));
            } else {
                self.has_props_rune = true;
            }

            if !self.in_var_declarator_init || self.function_depth > 0 {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsInvalidPlacement,
                    self.span(call.span),
                ));
            }

            if !call.arguments.is_empty() {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::RuneInvalidArguments {
                        rune: rune.display_name().into(),
                    },
                    self.span(call.span),
                ));
            }
        }

        // --- $props.id validation ---
        if matches!(rune, RuneKind::PropsId) {
            if self.has_props_id {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsDuplicate {
                        rune: rune.display_name().into(),
                    },
                    self.span(call.span),
                ));
            } else {
                self.has_props_id = true;
            }

            if !self.in_var_declarator_init || self.function_depth > 0 {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsIdInvalidPlacement,
                    self.span(call.span),
                ));
            }

            if !call.arguments.is_empty() {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::RuneInvalidArguments {
                        rune: rune.display_name().into(),
                    },
                    self.span(call.span),
                ));
            }
        }

        walk_call_expression(self, call);
    }

    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        if !self.runes {
            if let Some(Expression::CallExpression(call)) = &it.init {
                if let Expression::Identifier(ident) = &call.callee {
                    if ident.name == "$derived" {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidUsage {
                                rune: "$derived".into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
            }
        }

        let is_props_init = it
            .init
            .as_ref()
            .and_then(|e| detect_rune(e))
            .is_some_and(|r| matches!(r, RuneKind::Props));

        if is_props_init {
            self.validate_props_pattern(&it.id);
        }

        // Set flag so $bindable() calls inside the destructure pattern are valid.
        let prev_props = self.in_props_destructure;
        if is_props_init && matches!(&it.id, BindingPattern::ObjectPattern(_)) {
            self.in_props_destructure = true;
        }

        self.visit_binding_pattern(&it.id);

        self.in_props_destructure = prev_props;

        if let Some(init) = &it.init {
            let prev = self.in_var_declarator_init;
            self.in_var_declarator_init = true;
            self.visit_expression(init);
            self.in_var_declarator_init = prev;
        }
    }

    fn visit_function(
        &mut self,
        func: &oxc_ast::ast::Function<'a>,
        flags: oxc_semantic::ScopeFlags,
    ) {
        self.function_depth += 1;
        let prev_props = std::mem::replace(&mut self.in_props_destructure, false);
        let prev_first = std::mem::replace(
            &mut self.fn_body_first_stmt_span,
            func.body
                .as_ref()
                .and_then(|b| b.statements.first())
                .map(oxc_span::GetSpan::span),
        );
        let prev_generator = std::mem::replace(&mut self.in_generator, func.generator);
        walk_function(self, func, flags);
        self.in_props_destructure = prev_props;
        self.fn_body_first_stmt_span = prev_first;
        self.in_generator = prev_generator;
        self.function_depth -= 1;
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        self.function_depth += 1;
        let prev_props = std::mem::replace(&mut self.in_props_destructure, false);
        // Arrow functions cannot be generators; expression-body arrows have no block.
        let first_stmt = if arrow.expression {
            None
        } else {
            arrow.body.statements.first().map(oxc_span::GetSpan::span)
        };
        let prev_first = std::mem::replace(&mut self.fn_body_first_stmt_span, first_stmt);
        let prev_generator = std::mem::replace(&mut self.in_generator, false);
        walk_arrow_function_expression(self, arrow);
        self.in_props_destructure = prev_props;
        self.fn_body_first_stmt_span = prev_first;
        self.in_generator = prev_generator;
        self.function_depth -= 1;
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

// ---------------------------------------------------------------------------
// RestPropAccessValidator — `rest.$$foo` on rest_prop bindings
// ---------------------------------------------------------------------------

fn validate_rest_prop_illegal_access(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    // Skip if no rest_prop binding exists.
    if !data.scoping.has_rest_prop() {
        return;
    }
    let mut v = RestPropAccessValidator {
        data,
        offset,
        diags,
        _phantom: std::marker::PhantomData,
    };
    v.visit_program(program);
}

struct RestPropAccessValidator<'a, 'b> {
    data: &'b AnalysisData,
    offset: u32,
    diags: &'b mut Vec<Diagnostic>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Visit<'a> for RestPropAccessValidator<'a, '_> {
    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        if let MemberExpression::StaticMemberExpression(member) = expr {
            if let Expression::Identifier(obj) = &member.object {
                if member.property.name.starts_with("$$") {
                    if let Some(sym_id) = obj
                        .reference_id
                        .get()
                        .and_then(|r| self.data.scoping.try_get_reference(r))
                        .and_then(|reference| reference.symbol_id())
                    {
                        if self.data.scoping.is_rest_prop(sym_id) {
                            self.diags.push(Diagnostic::error(
                                DiagnosticKind::PropsIllegalName,
                                Span::new(
                                    member.property.span.start + self.offset,
                                    member.property.span.end + self.offset,
                                ),
                            ));
                        }
                    }
                }
            }
        }
        walk_member_expression(self, expr);
    }
}

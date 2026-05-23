use std::marker::PhantomData;
use std::mem;

use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentExpression, AssignmentOperator, AssignmentPattern,
    AssignmentTarget, BindingPattern, CallExpression, Declaration, ExportDefaultDeclaration,
    ExportDefaultDeclarationKind, ExportSpecifier, Expression, ExpressionStatement, Function,
    IdentifierReference, ImportDeclarationSpecifier, MemberExpression, MethodDefinition,
    MethodDefinitionKind, ModuleExportName, Program, PropertyDefinition, PropertyKey,
    StaticMemberExpression, Statement, VariableDeclarator,
};
use oxc_semantic::{ScopeFlags, ScopeId};
use oxc_span::Span as OxcSpan;
use oxc_syntax::symbol::SymbolId;
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_expression, walk_call_expression,
    walk_export_default_declaration, walk_expression_statement, walk_function,
    walk_member_expression, walk_method_definition, walk_property_definition,
    walk_static_member_expression,
};
use oxc_span::GetSpan;
use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::utils::script_info::{detect_rune, detect_rune_from_call};
use crate::validate::span_already_taken;
use crate::{AnalysisData, BindingSemantics, StateDeclarationSemantics, StateKind, PropBindingKind, PropBindingSemantics, types::script::RuneKind};

fn is_direct_bindable_call(expr: &Expression<'_>) -> bool {
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
        return false;
    };
    matches!(detect_rune_from_call(call), Some(RuneKind::Bindable))
}

fn is_this_member_assign(target: &AssignmentTarget<'_>) -> bool {
    let object = match target {
        AssignmentTarget::StaticMemberExpression(m) => &m.object,
        AssignmentTarget::PrivateFieldExpression(m) => &m.object,
        AssignmentTarget::ComputedMemberExpression(m) => &m.object,
        _ => return false,
    };
    matches!(object.get_inner_expression(), Expression::ThisExpression(_))
}

pub(super) fn validate(
    data: &AnalysisData,
    program: &Program<'_>,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    validate_invalid_lifecycle_imports(program, runes, diags);
    if runes {
        let mut v = RuneValidator::new(data, diags, true);
        v.visit_program(program);
    }
    validate_state_referenced_locally_derived(data, program, diags);
    validate_rest_prop_illegal_access(data, program, diags);
}

fn validate_invalid_lifecycle_imports(
    program: &Program<'_>,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if !runes {
        return;
    }
    'outer: for stmt in &program.body {
        let Statement::ImportDeclaration(import) = stmt else {
            continue;
        };
        if import.source.value.as_str() != "svelte" {
            continue;
        }
        let Some(specifiers) = &import.specifiers else {
            continue;
        };
        for spec in specifiers {
            let ImportDeclarationSpecifier::ImportSpecifier(s) = spec else {
                continue;
            };
            let name = match &s.imported {
                ModuleExportName::IdentifierName(id) => id.name.as_str(),
                ModuleExportName::IdentifierReference(id) => id.name.as_str(),
                ModuleExportName::StringLiteral(lit) => lit.value.as_str(),
            };
            if name == "beforeUpdate" || name == "afterUpdate" {
                diags.push(Diagnostic::error(
                    DiagnosticKind::RunesModeInvalidImport {
                        name: name.to_string(),
                    },
                    Span::new(s.span.start, s.span.end),
                ));
                break 'outer;
            }
        }
    }
}

pub(super) fn validate_module_props_runes(
    data: &AnalysisData,
    program: &Program<'_>,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if !runes {
        return;
    }
    let mut v = RuneValidator::new(data, diags, false);
    v.visit_program(program);
}

struct RuneValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    in_var_declarator_init: bool,
    in_class_property_init: bool,
    in_constructor_body: bool,

    in_this_assign_rhs: bool,

    in_expression_statement_expr: bool,

    current_expr_stmt_span: Option<OxcSpan>,

    fn_body_first_stmt_span: Option<OxcSpan>,

    in_generator: bool,

    function_depth: u32,

    has_props_rune: bool,

    has_props_id: bool,

    in_props_destructure: bool,

    in_props_default_position: bool,

    is_instance_script: bool,
    custom_element: bool,
}

impl RuneValidator<'_> {
    fn new<'a>(
        data: &AnalysisData,
        diags: &'a mut Vec<Diagnostic>,
        is_instance_script: bool,
    ) -> RuneValidator<'a> {
        RuneValidator {
            diags,
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
            in_props_default_position: false,
            in_props_destructure: false,
            is_instance_script,
            custom_element: data.output.is_custom_element_target,
        }
    }

    fn span(&self, oxc: OxcSpan) -> Span {
        Span::new(oxc.start, oxc.end)
    }

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

            if let PropertyKey::StaticIdentifier(key) = &prop.key
                && key.name.starts_with("$$")
            {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::PropsIllegalName,
                    self.span(prop.span),
                ));
            }

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

    fn check_deprecated_rune(&mut self, call: &CallExpression<'_>) -> bool {
        let Expression::StaticMemberExpression(member) = call.callee.get_inner_expression() else {
            return false;
        };
        let Expression::Identifier(obj) = member.object.get_inner_expression() else {
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

pub(super) fn validate_invalid_exports(
    data: &AnalysisData,
    program: &Program<'_>,
    check_decl: bool,
    specifier_scope: Option<ScopeId>,
    diags: &mut Vec<Diagnostic>,
) {
    if !check_decl && specifier_scope.is_none() {
        return;
    }
    for stmt in &program.body {
        let Statement::ExportNamedDeclaration(export) = stmt else {
            continue;
        };
        if let Some(decl) = export.declaration.as_ref() {
            if !check_decl {
                continue;
            }
            if let Some(kind) = declaration_export_kind(data, decl) {
                let span = Span::new(export.span.start, export.span.end);
                push_unique(diags, kind, span);
            }
        } else if let Some(scope) = specifier_scope {
            for spec in &export.specifiers {
                let (name, sp) = match &spec.local {
                    ModuleExportName::IdentifierReference(id) => (id.name.as_str(), id.span),
                    ModuleExportName::IdentifierName(id) => (id.name.as_str(), id.span),
                    ModuleExportName::StringLiteral(_) => continue,
                };
                let Some(sym_id) = data.scoping.find_binding(scope, name) else {
                    continue;
                };
                let Some(kind) = export_kind_for_symbol(data, sym_id) else {
                    continue;
                };
                let span = Span::new(sp.start, sp.end);
                push_unique(diags, kind, span);
            }
        }
    }
}

fn declaration_export_kind(
    data: &AnalysisData,
    decl: &Declaration<'_>,
) -> Option<DiagnosticKind> {
    let Declaration::VariableDeclaration(var_decl) = decl else {
        return None;
    };
    var_decl.declarations.iter().find_map(|declarator| {
        let BindingPattern::BindingIdentifier(ident) = &declarator.id else {
            return None;
        };
        let sym_id = ident.symbol_id.get()?;
        export_kind_for_symbol(data, sym_id)
    })
}

fn export_kind_for_symbol(
    data: &AnalysisData,
    sym_id: SymbolId,
) -> Option<DiagnosticKind> {
    match data.binding_semantics(sym_id) {
        BindingSemantics::Derived(_) => Some(DiagnosticKind::DerivedInvalidExport),
        BindingSemantics::State(StateDeclarationSemantics {
            kind: StateKind::State | StateKind::StateRaw,
            ..
        }) if data.scoping.is_mutated(sym_id) => Some(DiagnosticKind::StateInvalidExport),
        _ => None,
    }
}

fn push_unique(diags: &mut Vec<Diagnostic>, kind: DiagnosticKind, span: Span) {
    if !span_already_taken(diags, span) {
        diags.push(Diagnostic::error(kind, span));
    }
}

fn validate_state_referenced_locally_derived(
    data: &AnalysisData<'_>,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = StateRefLocallyValidator {
        data,
        diags,
        in_state_rune_arg: false,
        call_depth_offset: 0,
        in_illegal_prop_member_object: false,
        _phantom: PhantomData,
    };
    v.visit_program(program);
}

struct StateRefLocallyValidator<'a, 'b> {
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,

    in_state_rune_arg: bool,
    call_depth_offset: u32,

    in_illegal_prop_member_object: bool,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Visit<'a> for StateRefLocallyValidator<'a, '_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        if self.data.scoping.is_template_reference(ref_id) {
            return;
        }
        let Some(reference) = self.data.scoping.try_get_reference(ref_id) else {
            return;
        };

        if !reference.is_read() || reference.is_write() {
            return;
        }
        let Some(sym_id) = reference.symbol_id() else {
            return;
        };
        let declaration_semantics = self.data.binding_semantics(sym_id);
        let should_warn = match declaration_semantics {
            BindingSemantics::Derived(_) => true,
            BindingSemantics::State(state) if state.kind == StateKind::StateRaw => true,
            BindingSemantics::State(state) if state.kind == StateKind::State => {
                self.data.scoping.is_mutated(sym_id) || !state.proxied
            }

            BindingSemantics::OptimizedRune(opt) => match opt.kind {
                StateKind::StateRaw => true,
                StateKind::State => !opt.proxy_init,
                StateKind::StateEager => false,
            },
            BindingSemantics::Prop(_) if !self.in_illegal_prop_member_object => true,
            _ => false,
        };
        if !should_warn {
            return;
        }
        let decl_depth = self
            .data
            .scoping
            .function_depth(self.data.scoping.symbol_scope_id(sym_id));

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
            Span::new(ident.span.start, ident.span.end),
        ));
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        match detect_rune_from_call(call) {
            Some(RuneKind::State | RuneKind::StateRaw) => {
                self.visit_expression(&call.callee);
                let prev = mem::replace(&mut self.in_state_rune_arg, true);
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
        arrow: &ArrowFunctionExpression<'a>,
    ) {
        let prev_state_arg = mem::replace(&mut self.in_state_rune_arg, false);
        let prev_call_depth = mem::replace(&mut self.call_depth_offset, 0);
        walk_arrow_function_expression(self, arrow);
        self.in_state_rune_arg = prev_state_arg;
        self.call_depth_offset = prev_call_depth;
    }

    fn visit_function(
        &mut self,
        func: &Function<'a>,
        flags: ScopeFlags,
    ) {
        let prev_state_arg = mem::replace(&mut self.in_state_rune_arg, false);
        let prev_call_depth = mem::replace(&mut self.call_depth_offset, 0);
        walk_function(self, func, flags);
        self.in_state_rune_arg = prev_state_arg;
        self.call_depth_offset = prev_call_depth;
    }

    fn visit_static_member_expression(&mut self, expr: &StaticMemberExpression<'a>) {
        if is_props_illegal_name_member(expr, self.data) {
            let prev = mem::replace(&mut self.in_illegal_prop_member_object, true);
            self.visit_expression(&expr.object);
            self.in_illegal_prop_member_object = prev;
        } else {
            walk_static_member_expression(self, expr);
        }
    }

    fn visit_export_specifier(&mut self, _spec: &ExportSpecifier<'a>) {}

    fn visit_export_default_declaration(
        &mut self,
        export: &ExportDefaultDeclaration<'a>,
    ) {
        if matches!(
            export.declaration,
            ExportDefaultDeclarationKind::Identifier(_)
        ) {
            return;
        }
        walk_export_default_declaration(self, export);
    }
}

impl<'a> Visit<'a> for RuneValidator<'_> {
    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement<'a>) {
        let prev = mem::replace(&mut self.in_expression_statement_expr, true);
        let prev_span = self.current_expr_stmt_span.replace(stmt.span);
        walk_expression_statement(self, stmt);
        self.in_expression_statement_expr = prev;
        self.current_expr_stmt_span = prev_span;
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        let is_expr_stmt = mem::replace(&mut self.in_expression_statement_expr, false);
        let allowed_bindable_position = mem::take(&mut self.in_props_default_position);

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
                RuneKind::Host => {
                    if !call.arguments.is_empty() {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArguments {
                                rune: rune.display_name().into(),
                            },
                            self.span(call.span),
                        ));
                    } else {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::HostInvalidPlacement,
                            self.span(call.span),
                        ));
                    }
                }
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
            if !allowed_bindable_position {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::BindableInvalidLocation,
                    self.span(call.span),
                ));
            }
        }

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

    fn visit_assignment_pattern(&mut self, pat: &AssignmentPattern<'a>) {
        self.visit_binding_pattern(&pat.left);
        let allow = self.in_props_destructure && is_direct_bindable_call(&pat.right);
        let prev = mem::replace(&mut self.in_props_default_position, allow);
        self.visit_expression(&pat.right);
        self.in_props_default_position = prev;
    }

    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        let is_props_init = it
            .init
            .as_ref()
            .and_then(|e| detect_rune(e))
            .is_some_and(|r| matches!(r, RuneKind::Props));

        if is_props_init {
            self.validate_props_pattern(&it.id);
        }

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
        func: &Function<'a>,
        flags: ScopeFlags,
    ) {
        self.function_depth += 1;
        let prev_props = mem::replace(&mut self.in_props_destructure, false);
        let prev_first = mem::replace(
            &mut self.fn_body_first_stmt_span,
            func.body
                .as_ref()
                .and_then(|b| b.statements.first())
                .map(GetSpan::span),
        );
        let prev_generator = mem::replace(&mut self.in_generator, func.generator);
        walk_function(self, func, flags);
        self.in_props_destructure = prev_props;
        self.fn_body_first_stmt_span = prev_first;
        self.in_generator = prev_generator;
        self.function_depth -= 1;
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &ArrowFunctionExpression<'a>,
    ) {
        self.function_depth += 1;
        let prev_props = mem::replace(&mut self.in_props_destructure, false);

        let first_stmt = if arrow.expression {
            None
        } else {
            arrow.body.statements.first().map(GetSpan::span)
        };
        let prev_first = mem::replace(&mut self.fn_body_first_stmt_span, first_stmt);
        let prev_generator = mem::replace(&mut self.in_generator, false);
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

    fn visit_method_definition(&mut self, it: &MethodDefinition<'a>) {
        let prev = self.in_constructor_body;
        if it.kind == MethodDefinitionKind::Constructor {
            self.in_constructor_body = true;
        }
        walk_method_definition(self, it);
        self.in_constructor_body = prev;
    }

    fn visit_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {
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

fn validate_rest_prop_illegal_access(
    data: &AnalysisData<'_>,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = RestPropAccessValidator {
        data,
        diags,
        _phantom: PhantomData,
    };
    v.visit_program(program);
}

struct RestPropAccessValidator<'a, 'b> {
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Visit<'a> for RestPropAccessValidator<'a, '_> {
    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        if let MemberExpression::StaticMemberExpression(member) = expr
            && is_props_illegal_name_member(member, self.data)
        {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::PropsIllegalName,
                Span::new(
                    member.property.span.start,
                    member.property.span.end,
                ),
            ));
        }
        walk_member_expression(self, expr);
    }
}

fn is_props_illegal_name_member(
    member: &StaticMemberExpression<'_>,
    data: &AnalysisData<'_>,
) -> bool {
    let Expression::Identifier(obj) = member.object.get_inner_expression() else {
        return false;
    };
    if !member.property.name.starts_with("$$") {
        return false;
    }
    let Some(sym_id) = obj
        .reference_id
        .get()
        .and_then(|r| data.scoping.try_get_reference(r))
        .and_then(|reference| reference.symbol_id())
    else {
        return false;
    };
    matches!(
        data.binding_semantics(sym_id),
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Rest,
            ..
        }),
    )
}

pub fn validate_const_tag_runes(
    component: &Component,
    parsed: &crate::types::data::JsAst,
    diags: &mut Vec<Diagnostic>,
) {
    for node in component.store.iter_nodes() {
        let svelte_ast::Node::ConstTag(tag) = node else {
            continue;
        };
        let Some(stmt) = parsed.stmt(tag.decl.id()) else {
            continue;
        };
        let Statement::VariableDeclaration(decl) = stmt else {
            continue;
        };
        for declarator in &decl.declarations {
            let Some(init) = &declarator.init else {
                continue;
            };
            let mut probe = ConstTagRuneProbe { diags };
            probe.visit_expression(init);
        }
    }
}

struct ConstTagRuneProbe<'d> {
    diags: &'d mut Vec<Diagnostic>,
}

impl<'a> Visit<'a> for ConstTagRuneProbe<'_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if let Some(rune) = detect_rune_from_call(call) {
            let span = Span::new(call.span.start, call.span.end);
            let kind = match rune {
                RuneKind::State | RuneKind::StateRaw | RuneKind::Derived | RuneKind::DerivedBy => {
                    Some(DiagnosticKind::StateInvalidPlacement {
                        rune: rune.display_name().into(),
                    })
                }
                RuneKind::Props => Some(DiagnosticKind::PropsInvalidPlacement),
                RuneKind::PropsId => Some(DiagnosticKind::PropsIdInvalidPlacement),
                RuneKind::Bindable => Some(DiagnosticKind::BindableInvalidLocation),
                RuneKind::Host => Some(DiagnosticKind::HostInvalidPlacement),
                RuneKind::Effect | RuneKind::EffectPre => {
                    Some(DiagnosticKind::EffectInvalidPlacement)
                }
                _ => None,
            };
            if let Some(kind) = kind {
                self.diags.push(Diagnostic::error(kind, span));
            }
        }
        walk_call_expression(self, call);
    }
}

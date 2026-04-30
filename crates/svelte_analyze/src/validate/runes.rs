use oxc_ast::ast::{
    AssignmentOperator, BindingPattern, CallExpression, ExportDefaultDeclaration,
    ExportNamedDeclaration, Expression, ExpressionStatement, ImportDeclarationSpecifier,
    MemberExpression, MethodDefinitionKind, ModuleExportName, PropertyDefinition, Statement,
    VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_expression, walk_call_expression,
    walk_expression_statement, walk_function, walk_member_expression, walk_method_definition,
    walk_property_definition,
};
use oxc_span::GetSpan;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::utils::script_info::{detect_rune, detect_rune_from_call};
use crate::{AnalysisData, BindingSemantics, StateKind, types::script::RuneKind};

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
    validate_invalid_lifecycle_imports(program, offset, runes, diags);
    let mut v = RuneValidator::new(data, diags, offset, runes, true);
    v.visit_program(program);
    validate_derived_invalid_export(data, program, offset, diags);
    validate_state_invalid_export(data, program, offset, diags);
    validate_state_referenced_locally_derived(data, program, offset, diags);
    validate_rest_prop_illegal_access(data, program, offset, diags);
}

fn validate_invalid_lifecycle_imports(
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
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
                    Span::new(s.span.start + offset, s.span.end + offset),
                ));
                break 'outer;
            }
        }
    }
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

    in_this_assign_rhs: bool,

    in_expression_statement_expr: bool,

    current_expr_stmt_span: Option<oxc_span::Span>,

    fn_body_first_stmt_span: Option<oxc_span::Span>,

    in_generator: bool,

    function_depth: u32,

    has_props_rune: bool,

    has_props_id: bool,

    in_props_destructure: bool,

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

            if let oxc_ast::ast::PropertyKey::StaticIdentifier(key) = &prop.key
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
        |data, sym_id| matches!(data.binding_semantics(sym_id), BindingSemantics::Derived(_)),
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
        let span = Span::new(export.span.start + offset, export.span.end + offset);
        if !crate::validate::span_already_taken(diags, span) {
            diags.push(Diagnostic::error(make_kind(), span));
        }
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

fn is_reassigned_state_export(data: &AnalysisData<'_>, sym_id: oxc_semantic::SymbolId) -> bool {
    matches!(
        data.binding_semantics(sym_id),
        BindingSemantics::State(crate::StateDeclarationSemantics {
            kind: StateKind::State | StateKind::StateRaw,
            ..
        })
    ) && data.scoping.is_mutated(sym_id)
}

fn validate_state_referenced_locally_derived(
    data: &AnalysisData<'_>,
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
    data: &'b AnalysisData<'a>,
    offset: u32,
    diags: &'b mut Vec<Diagnostic>,

    in_state_rune_arg: bool,

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
            Span::new(ident.span.start + self.offset, ident.span.end + self.offset),
        ));
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        match detect_rune_from_call(call) {
            Some(RuneKind::State | RuneKind::StateRaw) => {
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
        let prev_span = self.current_expr_stmt_span.replace(stmt.span);
        walk_expression_statement(self, stmt);
        self.in_expression_statement_expr = prev;
        self.current_expr_stmt_span = prev_span;
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
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
            if !self.in_props_destructure {
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

    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        if !self.runes
            && let Some(Expression::CallExpression(call)) = &it.init
            && let Expression::Identifier(ident) = &call.callee
            && ident.name == "$derived"
        {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::RuneInvalidUsage {
                    rune: "$derived".into(),
                },
                self.span(call.span),
            ));
        }

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

fn validate_rest_prop_illegal_access(
    data: &AnalysisData<'_>,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = RestPropAccessValidator {
        data,
        offset,
        diags,
        _phantom: std::marker::PhantomData,
    };
    v.visit_program(program);
}

struct RestPropAccessValidator<'a, 'b> {
    data: &'b AnalysisData<'a>,
    offset: u32,
    diags: &'b mut Vec<Diagnostic>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Visit<'a> for RestPropAccessValidator<'a, '_> {
    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        if let MemberExpression::StaticMemberExpression(member) = expr
            && let Expression::Identifier(obj) = &member.object
            && member.property.name.starts_with("$$")
            && let Some(sym_id) = obj
                .reference_id
                .get()
                .and_then(|r| self.data.scoping.try_get_reference(r))
                .and_then(|reference| reference.symbol_id())
            && matches!(
                self.data.binding_semantics(sym_id),
                crate::types::data::BindingSemantics::Prop(
                    crate::types::data::PropBindingSemantics {
                        kind: crate::types::data::PropBindingKind::Rest,
                        ..
                    },
                ),
            )
        {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::PropsIllegalName,
                Span::new(
                    member.property.span.start + self.offset,
                    member.property.span.end + self.offset,
                ),
            ));
        }
        walk_member_expression(self, expr);
    }
}

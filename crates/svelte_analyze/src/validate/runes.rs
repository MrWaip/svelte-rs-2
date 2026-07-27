use std::marker::PhantomData;
use std::mem;

use oxc_ast::ast::{
    Argument, ArrowFunctionExpression, AssignmentExpression, AssignmentOperator, AssignmentPattern,
    AssignmentTarget, BindingPattern, CallExpression, Declaration, ExportDefaultDeclarationKind,
    Expression, ExpressionStatement, Function, IdentifierReference, ImportDeclarationSpecifier,
    MethodDefinition, MethodDefinitionKind, ModuleExportName, Program, PropertyDefinition,
    PropertyKey, Statement, StaticMemberExpression, VariableDeclarator,
};
use oxc_ast::{AstKind, AstType};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_assignment_expression, walk_call_expression,
    walk_expression, walk_expression_statement, walk_function, walk_method_definition,
    walk_property_definition,
};
use oxc_semantic::{ScopeFlags, ScopeId};
use oxc_span::GetSpan;
use oxc_span::Span as OxcSpan;
use oxc_syntax::symbol::SymbolId;
use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::js_walker::{JsFlow, JsNodeMask, JsVisitor};
use crate::reactivity_semantics::data::{
    DeclaratorSemantics, DerivedKind, ReactivitySemantics, RuntimeRuneKind,
};
use crate::validate::span_already_taken;
use crate::{AnalysisData, BindingSemantics, StateDeclarationSemantics, StateKind};

fn rune_display_name(sem: &DeclaratorSemantics) -> Option<&'static str> {
    Some(match sem {
        DeclaratorSemantics::RuneState { kind } => match kind {
            StateKind::State => "$state",
            StateKind::StateRaw => "$state.raw",
            StateKind::StateEager => "$state.eager",
        },
        DeclaratorSemantics::RuneDerived { kind, .. } => match kind {
            DerivedKind::Derived => "$derived",
            DerivedKind::DerivedBy => "$derived.by",
        },
        DeclaratorSemantics::RuneProps => "$props",
        DeclaratorSemantics::RuntimeRuneCall { kind } => kind.display_name(),
        DeclaratorSemantics::None
        | DeclaratorSemantics::LegacyProps
        | DeclaratorSemantics::LegacyState
        | DeclaratorSemantics::ConstTag { .. }
        | DeclaratorSemantics::LetCarrier { .. }
        | DeclaratorSemantics::EachItem
        | DeclaratorSemantics::AwaitValue
        | DeclaratorSemantics::SnippetParam
        | DeclaratorSemantics::ClassFieldState(_)
        | DeclaratorSemantics::ClassFieldDerived(_) => return None,
    })
}

fn is_direct_bindable_call(reactivity: &ReactivitySemantics, expr: &Expression<'_>) -> bool {
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
        return false;
    };
    reactivity
        .declarator_semantics(call.node_id())
        .is_bindable_call()
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
    is_instance_script: bool,
    diags: &mut Vec<Diagnostic>,
) {
    validate_invalid_lifecycle_imports(program, runes, diags);
    if runes {
        let mut v = RuneValidator::new(data, diags, is_instance_script);
        v.visit_program(program);
    }
    if runes {
        validate_rune_names(data, program, diags);
    }
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
        if import.source.value.as_str().starts_with("svelte/internal") {
            diags.push(Diagnostic::error(
                DiagnosticKind::ImportSvelteInternalForbidden,
                Span::new(import.span.start, import.span.end),
            ));
        }
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
    reactivity: &'a ReactivitySemantics,
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
    ce_config_has_props: bool,
}

impl RuneValidator<'_> {
    fn new<'a>(
        data: &'a AnalysisData,
        diags: &'a mut Vec<Diagnostic>,
        is_instance_script: bool,
    ) -> RuneValidator<'a> {
        RuneValidator {
            reactivity: &data.reactivity,
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
            custom_element: data.custom_element.is_target,
            ce_config_has_props: data
                .script
                .ce_config
                .as_ref()
                .is_some_and(|config| !config.props.is_empty()),
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

    fn check_custom_element_props_pattern(&mut self, pattern: &BindingPattern<'_>) {
        if !self.custom_element || self.ce_config_has_props || !self.is_instance_script {
            return;
        }
        let span = match pattern {
            BindingPattern::BindingIdentifier(id) => id.span,
            BindingPattern::ObjectPattern(obj) => match &obj.rest {
                Some(rest) => rest.span,
                None => return,
            },
            BindingPattern::ArrayPattern(_) | BindingPattern::AssignmentPattern(_) => return,
        };
        self.diags.push(Diagnostic::warning(
            DiagnosticKind::CustomElementPropsIdentifier,
            Span::new(span.start, span.end),
        ));
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

pub(super) fn validate_default_export_state(
    data: &AnalysisData,
    program: &Program<'_>,
    scope: ScopeId,
    diags: &mut Vec<Diagnostic>,
) {
    for stmt in &program.body {
        let Statement::ExportDefaultDeclaration(export) = stmt else {
            continue;
        };
        let ExportDefaultDeclarationKind::Identifier(id) = &export.declaration else {
            continue;
        };
        let Some(sym_id) = data.scoping.find_binding(scope, id.name.as_str()) else {
            continue;
        };
        let Some(kind) = export_kind_for_symbol(data, sym_id) else {
            continue;
        };
        push_unique(diags, kind, Span::new(export.span.start, export.span.end));
    }
}

fn declaration_export_kind(data: &AnalysisData, decl: &Declaration<'_>) -> Option<DiagnosticKind> {
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

fn export_kind_for_symbol(data: &AnalysisData, sym_id: SymbolId) -> Option<DiagnosticKind> {
    match data.binding_semantics(sym_id) {
        BindingSemantics::Derived(_) | BindingSemantics::OptimizedDerived(_) => {
            Some(DiagnosticKind::DerivedInvalidExport)
        }
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

pub(super) fn new_state_ref_locally_validator<'a, 'b>(
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,
) -> StateRefLocallyValidator<'a, 'b> {
    StateRefLocallyValidator {
        data,
        diags,
        in_state_rune_arg: false,
        state_rune_arg_stack: Vec::new(),
        call_depth_offset: 0,
        call_depth_stack: Vec::new(),
        in_illegal_prop_member_object: false,
        illegal_prop_member_stack: Vec::new(),
        _phantom: PhantomData,
    }
}

pub(super) struct StateRefLocallyValidator<'a, 'b> {
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,

    in_state_rune_arg: bool,
    state_rune_arg_stack: Vec<bool>,
    call_depth_offset: u32,
    call_depth_stack: Vec<u32>,

    in_illegal_prop_member_object: bool,
    illegal_prop_member_stack: Vec<bool>,
    _phantom: PhantomData<&'a ()>,
}

enum StateRuneCallKind {
    StateArgument,
    DeferredArgument,
    Plain,
}

impl<'a> StateRefLocallyValidator<'a, '_> {
    fn call_kind(&self, call: &CallExpression<'a>) -> StateRuneCallKind {
        match self.data.reactivity.declarator_semantics(call.node_id()) {
            DeclaratorSemantics::RuneState {
                kind: StateKind::State | StateKind::StateRaw,
            } => StateRuneCallKind::StateArgument,
            DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::RuntimeRuneCall {
                kind: RuntimeRuneKind::Inspect,
            } => StateRuneCallKind::DeferredArgument,
            _ => StateRuneCallKind::Plain,
        }
    }

    fn check_identifier(&mut self, ident: &IdentifierReference<'a>) {
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
            BindingSemantics::Derived(_) | BindingSemantics::OptimizedDerived(_) => true,
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
}

const STATE_REF_LOCALLY_LEAVE_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::CallExpression,
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::StaticMemberExpression,
    AstType::ExportSpecifier,
    AstType::ExportDefaultDeclaration,
]);

const STATE_REF_LOCALLY_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::IdentifierReference,
    AstType::CallExpression,
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::StaticMemberExpression,
    AstType::ExportSpecifier,
    AstType::ExportDefaultDeclaration,
]);

impl<'a> JsVisitor<'a> for StateRefLocallyValidator<'a, '_> {
    fn enter_interests(&self) -> JsNodeMask {
        STATE_REF_LOCALLY_INTERESTS
    }

    fn leave_interests(&self) -> JsNodeMask {
        STATE_REF_LOCALLY_LEAVE_INTERESTS
    }

    fn enter_js_node(&mut self, kind: AstKind<'a>) -> JsFlow {
        match kind {
            AstKind::IdentifierReference(ident) => self.check_identifier(ident),
            AstKind::CallExpression(call) => match self.call_kind(call) {
                StateRuneCallKind::StateArgument => {
                    self.state_rune_arg_stack.push(self.in_state_rune_arg);
                    self.in_state_rune_arg = true;
                }
                StateRuneCallKind::DeferredArgument => {
                    self.call_depth_offset += 1;
                }
                StateRuneCallKind::Plain => {}
            },
            AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => {
                self.state_rune_arg_stack.push(self.in_state_rune_arg);
                self.call_depth_stack.push(self.call_depth_offset);
                self.in_state_rune_arg = false;
                self.call_depth_offset = 0;
            }
            AstKind::StaticMemberExpression(expr) => {
                self.illegal_prop_member_stack
                    .push(self.in_illegal_prop_member_object);
                if is_props_illegal_name_member(expr, self.data) {
                    self.in_illegal_prop_member_object = true;
                }
            }
            AstKind::ExportSpecifier(_) => return JsFlow::SkipSubtree,
            AstKind::ExportDefaultDeclaration(export) => {
                if matches!(
                    export.declaration,
                    ExportDefaultDeclarationKind::Identifier(_)
                ) {
                    return JsFlow::SkipSubtree;
                }
            }
            _ => {}
        }
        JsFlow::Continue
    }

    fn leave_js_node(&mut self, kind: AstKind<'a>) {
        match kind {
            AstKind::CallExpression(call) => match self.call_kind(call) {
                StateRuneCallKind::StateArgument => {
                    if let Some(prev) = self.state_rune_arg_stack.pop() {
                        self.in_state_rune_arg = prev;
                    }
                }
                StateRuneCallKind::DeferredArgument => {
                    self.call_depth_offset -= 1;
                }
                StateRuneCallKind::Plain => {}
            },
            AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => {
                if let Some(prev) = self.state_rune_arg_stack.pop() {
                    self.in_state_rune_arg = prev;
                }
                if let Some(prev) = self.call_depth_stack.pop() {
                    self.call_depth_offset = prev;
                }
            }
            AstKind::StaticMemberExpression(_) => {
                if let Some(prev) = self.illegal_prop_member_stack.pop() {
                    self.in_illegal_prop_member_object = prev;
                }
            }
            _ => {}
        }
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

        let sem = self.reactivity.declarator_semantics(call.node_id());
        let Some(rune_name) = rune_display_name(&sem) else {
            walk_call_expression(self, call);
            return;
        };

        if rune_name != "$inspect"
            && call
                .arguments
                .iter()
                .any(|arg| matches!(arg, Argument::SpreadElement(_)))
        {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::RuneInvalidSpread {
                    rune: rune_name.into(),
                },
                self.span(call.span),
            ));
            return;
        }

        match &sem {
            DeclaratorSemantics::RuneState {
                kind: StateKind::State | StateKind::StateRaw,
            } => {
                let valid = self.in_var_declarator_init
                    || self.in_class_property_init
                    || self.in_this_assign_rhs;
                if !valid {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::StateInvalidPlacement {
                            rune: rune_name.into(),
                        },
                        self.span(call.span),
                    ));
                }
                if call.arguments.len() > 1 {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::RuneInvalidArgumentsLength {
                            rune: rune_name.into(),
                            args: "zero or one arguments".into(),
                        },
                        self.span(call.span),
                    ));
                }
            }
            DeclaratorSemantics::RuneState {
                kind: StateKind::StateEager,
            } => {
                if call.arguments.len() != 1 {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::RuneInvalidArgumentsLength {
                            rune: rune_name.into(),
                            args: "exactly one argument".into(),
                        },
                        self.span(call.span),
                    ));
                }
            }
            DeclaratorSemantics::RuneDerived { .. } => {
                let valid = self.in_var_declarator_init
                    || self.in_class_property_init
                    || self.in_this_assign_rhs;
                if !valid {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::StateInvalidPlacement {
                            rune: rune_name.into(),
                        },
                        self.span(call.span),
                    ));
                }
                if call.arguments.len() != 1 {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::RuneInvalidArgumentsLength {
                            rune: rune_name.into(),
                            args: "exactly one argument".into(),
                        },
                        self.span(call.span),
                    ));
                }
            }
            DeclaratorSemantics::RuneProps => {
                if self.has_props_rune {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::PropsDuplicate {
                            rune: rune_name.into(),
                        },
                        self.span(call.span),
                    ));
                } else {
                    self.has_props_rune = true;
                }
                if !self.is_instance_script
                    || !self.in_var_declarator_init
                    || self.function_depth > 0
                {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::PropsInvalidPlacement,
                        self.span(call.span),
                    ));
                }
                if !call.arguments.is_empty() {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::RuneInvalidArguments {
                            rune: rune_name.into(),
                        },
                        self.span(call.span),
                    ));
                }
            }
            DeclaratorSemantics::RuntimeRuneCall { kind } => match kind {
                RuntimeRuneKind::StateEager => {
                    if call.arguments.len() != 1 {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArgumentsLength {
                                rune: rune_name.into(),
                                args: "exactly one argument".into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::Effect | RuntimeRuneKind::EffectPre => {
                    if !is_expr_stmt {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::EffectInvalidPlacement,
                            self.span(call.span),
                        ));
                    }
                    if call.arguments.len() != 1 {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArgumentsLength {
                                rune: rune_name.into(),
                                args: "exactly one argument".into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::EffectRoot => {
                    if call.arguments.len() != 1 {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArgumentsLength {
                                rune: rune_name.into(),
                                args: "exactly one argument".into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::EffectTracking => {
                    if !call.arguments.is_empty() {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArguments {
                                rune: rune_name.into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::Inspect => {
                    if call.arguments.is_empty() {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArgumentsLength {
                                rune: rune_name.into(),
                                args: "one or more arguments".into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::InspectWith => {
                    if call.arguments.len() != 1 {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArgumentsLength {
                                rune: rune_name.into(),
                                args: "exactly one argument".into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::InspectTrace => {
                    if call.arguments.len() > 1 {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArgumentsLength {
                                rune: rune_name.into(),
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
                RuntimeRuneKind::Host => {
                    if !call.arguments.is_empty() {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArguments {
                                rune: rune_name.into(),
                            },
                            self.span(call.span),
                        ));
                    } else if !self.is_instance_script || !self.custom_element {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::HostInvalidPlacement,
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::Bindable => {
                    if call.arguments.len() > 1 {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArgumentsLength {
                                rune: rune_name.into(),
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
                RuntimeRuneKind::PropsId => {
                    if self.has_props_id {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::PropsDuplicate {
                                rune: rune_name.into(),
                            },
                            self.span(call.span),
                        ));
                    } else {
                        self.has_props_id = true;
                    }
                    if !self.is_instance_script
                        || !self.in_var_declarator_init
                        || self.function_depth > 0
                    {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::PropsIdInvalidPlacement,
                            self.span(call.span),
                        ));
                    }
                    if !call.arguments.is_empty() {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArguments {
                                rune: rune_name.into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::StateSnapshot => {
                    if call.arguments.len() != 1 {
                        self.diags.push(Diagnostic::error(
                            DiagnosticKind::RuneInvalidArgumentsLength {
                                rune: rune_name.into(),
                                args: "exactly one argument".into(),
                            },
                            self.span(call.span),
                        ));
                    }
                }
                RuntimeRuneKind::EffectPending => {}
            },
            _ => {}
        }

        walk_call_expression(self, call);
    }

    fn visit_assignment_pattern(&mut self, pat: &AssignmentPattern<'a>) {
        self.visit_binding_pattern(&pat.left);
        let allow =
            self.in_props_destructure && is_direct_bindable_call(self.reactivity, &pat.right);
        let prev = mem::replace(&mut self.in_props_default_position, allow);
        self.visit_expression(&pat.right);
        self.in_props_default_position = prev;
    }

    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        let is_props_init = it.init.as_ref().is_some_and(|init| {
            let Expression::CallExpression(call) = init.get_inner_expression() else {
                return false;
            };
            self.reactivity
                .declarator_semantics(call.node_id())
                .is_rune_props()
        });

        if is_props_init {
            self.validate_props_pattern(&it.id);
            self.check_custom_element_props_pattern(&it.id);
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

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
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

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
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

fn rune_root<'x, 'a>(expr: &'x Expression<'a>) -> Option<&'x IdentifierReference<'a>> {
    match expr.get_inner_expression() {
        Expression::Identifier(id) => Some(id),
        Expression::StaticMemberExpression(m) => rune_root(&m.object),
        Expression::ComputedMemberExpression(m) => rune_root(&m.object),
        _ => None,
    }
}

fn validate_rune_names(
    data: &AnalysisData<'_>,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = RuneNameValidator {
        data,
        diags,
        _phantom: PhantomData,
    };
    v.visit_program(program);
}

struct RuneNameValidator<'a, 'b> {
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> RuneNameValidator<'a, '_> {
    fn is_unbound(&self, id: &IdentifierReference<'a>) -> bool {
        id.reference_id
            .get()
            .and_then(|r| self.data.scoping.try_get_reference(r))
            .and_then(|reference| reference.symbol_id())
            .is_none()
    }

    fn process_chain(&mut self, expr: &Expression<'a>, called: bool) -> bool {
        let inner = expr.get_inner_expression();
        let Some(root) = rune_root(inner) else {
            return false;
        };
        if !svelte_ast::is_rune_name(root.name.as_str()) || !self.is_unbound(root) {
            return false;
        }

        let mut name = root.name.to_string();
        let stopped = self.walk_rune_members(inner, &mut name, called);

        if !stopped && !called {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::RuneMissingParentheses,
                Span::new(inner.span().start, inner.span().end),
            ));
        }
        true
    }

    fn walk_rune_members(
        &mut self,
        expr: &Expression<'a>,
        name: &mut String,
        called: bool,
    ) -> bool {
        match expr {
            Expression::StaticMemberExpression(m) => {
                if self.walk_rune_members(m.object.get_inner_expression(), name, called) {
                    return true;
                }
                name.push('.');
                name.push_str(m.property.name.as_str());
                self.check_rune_keypath(name, m.span, called)
            }
            Expression::ComputedMemberExpression(m) => {
                if self.walk_rune_members(m.object.get_inner_expression(), name, called) {
                    return true;
                }
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::RuneInvalidComputedProperty,
                    Span::new(m.span.start, m.span.end),
                ));
                true
            }
            _ => false,
        }
    }

    fn check_rune_keypath(&mut self, name: &str, span: OxcSpan, called: bool) -> bool {
        let (kind, tolerated) = match svelte_ast::classify_rune_keypath(name) {
            svelte_ast::RuneKeypath::Valid => return false,
            svelte_ast::RuneKeypath::Renamed {
                replacement,
                tolerated_when_called,
            } => (
                DiagnosticKind::RuneRenamed {
                    name: name.to_string(),
                    replacement: replacement.into(),
                },
                tolerated_when_called,
            ),
            svelte_ast::RuneKeypath::Removed {
                tolerated_when_called,
            } => (
                DiagnosticKind::RuneRemoved {
                    name: name.to_string(),
                },
                tolerated_when_called,
            ),
            svelte_ast::RuneKeypath::Unknown => (
                DiagnosticKind::RuneInvalidName {
                    name: name.to_string(),
                },
                false,
            ),
        };
        if called && tolerated {
            return true;
        }
        self.diags
            .push(Diagnostic::error(kind, Span::new(span.start, span.end)));
        true
    }
}

impl<'a> Visit<'a> for RuneNameValidator<'a, '_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if self.process_chain(&call.callee, true) {
            self.visit_arguments(&call.arguments);
            return;
        }
        walk_call_expression(self, call);
    }

    fn visit_expression(&mut self, expr: &Expression<'a>) {
        if self.process_chain(expr, false) {
            return;
        }
        walk_expression(self, expr);
    }
}

pub(super) fn new_rest_prop_validator<'a, 'b>(
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,
) -> RestPropAccessValidator<'a, 'b> {
    RestPropAccessValidator {
        data,
        diags,
        _phantom: PhantomData,
    }
}

pub(super) struct RestPropAccessValidator<'a, 'b> {
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,
    _phantom: PhantomData<&'a ()>,
}

const REST_PROP_INTERESTS: JsNodeMask = JsNodeMask::new(&[AstType::StaticMemberExpression]);

impl<'a> JsVisitor<'a> for RestPropAccessValidator<'a, '_> {
    fn enter_interests(&self) -> JsNodeMask {
        REST_PROP_INTERESTS
    }

    fn enter_js_node(&mut self, kind: AstKind<'a>) -> JsFlow {
        if let AstKind::StaticMemberExpression(member) = kind
            && is_props_illegal_name_member(member, self.data)
        {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::PropsIllegalName,
                Span::new(member.property.span.start, member.property.span.end),
            ));
        }
        JsFlow::Continue
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
    data.binding_semantics(sym_id).is_rest_props()
}

pub fn validate_const_tag_runes(
    component: &Component,
    parsed: &crate::types::data::JsAst,
    data: &AnalysisData,
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
            let mut probe = ConstTagRuneProbe {
                reactivity: &data.reactivity,
                diags,
            };
            probe.visit_expression(init);
        }
    }
}

pub fn validate_declaration_tag_runes(
    component: &Component,
    parsed: &crate::types::data::JsAst,
    data: &AnalysisData,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if !runes {
        return;
    }
    for node in component.store.iter_nodes() {
        let svelte_ast::Node::DeclarationTag(tag) = node else {
            continue;
        };
        let Some(stmt) = parsed.stmt(tag.declaration.id()) else {
            continue;
        };
        let mut v = RuneValidator::new(data, diags, false);
        v.visit_statement(stmt);
    }
}

struct ConstTagRuneProbe<'d> {
    reactivity: &'d ReactivitySemantics,
    diags: &'d mut Vec<Diagnostic>,
}

impl<'a> Visit<'a> for ConstTagRuneProbe<'_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        let sem = self.reactivity.declarator_semantics(call.node_id());
        let kind = match &sem {
            DeclaratorSemantics::RuneState {
                kind: StateKind::State | StateKind::StateRaw,
            }
            | DeclaratorSemantics::RuneDerived { .. } => {
                Some(DiagnosticKind::StateInvalidPlacement {
                    rune: rune_display_name(&sem).unwrap_or_default().into(),
                })
            }
            DeclaratorSemantics::RuneProps => Some(DiagnosticKind::PropsInvalidPlacement),
            DeclaratorSemantics::RuntimeRuneCall {
                kind: RuntimeRuneKind::PropsId,
            } => Some(DiagnosticKind::PropsIdInvalidPlacement),
            DeclaratorSemantics::RuntimeRuneCall {
                kind: RuntimeRuneKind::Bindable,
            } => Some(DiagnosticKind::BindableInvalidLocation),
            DeclaratorSemantics::RuntimeRuneCall {
                kind: RuntimeRuneKind::Host,
            } => Some(DiagnosticKind::HostInvalidPlacement),
            DeclaratorSemantics::RuntimeRuneCall {
                kind: RuntimeRuneKind::Effect | RuntimeRuneKind::EffectPre,
            } => Some(DiagnosticKind::EffectInvalidPlacement),
            _ => None,
        };
        if let Some(kind) = kind {
            self.diags.push(Diagnostic::error(
                kind,
                Span::new(call.span.start, call.span.end),
            ));
        }
        walk_call_expression(self, call);
    }
}

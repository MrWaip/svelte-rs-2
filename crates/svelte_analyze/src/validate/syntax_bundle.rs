use oxc_ast::ast::{
    AccessorProperty, ArrowFunctionExpression, CallExpression, Class, Declaration, Decorator,
    Expression, FormalParameter, Function, LabeledStatement, MethodDefinition,
    MethodDefinitionKind, NewExpression, Program, TSEnumDeclaration, TSModuleDeclaration,
    VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_call_expression, walk_class, walk_declaration,
    walk_function, walk_labeled_statement, walk_method_definition, walk_new_expression,
    walk_variable_declarator,
};
use oxc_syntax::scope::ScopeFlags;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::types::data::AnalysisData;

use super::class_state_fields::check_class;
use super::stores::{check_legacy_rune_invalid_usage, check_store_rune_conflict};
use super::typescript::namespace_has_non_type_node;

pub(super) fn validate_instance(
    data: &AnalysisData,
    program: &Program<'_>,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let mut validator = SyntaxValidator {
        diags,
        data,
        in_constructor: false,
        function_depth: 0,
        base_function_depth: 1,
        check_class_state: runes,
        check_legacy_placement: !runes,
        check_stores: true,
    };
    validator.visit_program(program);
}

pub(super) fn validate_module(
    data: &AnalysisData,
    program: &Program<'_>,
    check_class_state: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let mut validator = SyntaxValidator {
        diags,
        data,
        in_constructor: false,
        function_depth: 0,
        base_function_depth: 0,
        check_class_state,
        check_legacy_placement: false,
        check_stores: false,
    };
    validator.visit_program(program);
}

struct SyntaxValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    data: &'a AnalysisData<'a>,
    in_constructor: bool,
    function_depth: u32,
    base_function_depth: u32,
    check_class_state: bool,
    check_legacy_placement: bool,
    check_stores: bool,
}

impl SyntaxValidator<'_> {
    fn push_ts(&mut self, feature: &str, span: oxc_span::Span) {
        self.diags.push(Diagnostic::error(
            DiagnosticKind::TypescriptInvalidFeature {
                feature: feature.to_string(),
            },
            Span::new(span.start, span.end),
        ));
    }
}

impl<'a> Visit<'a> for SyntaxValidator<'_> {
    fn visit_function(&mut self, function: &Function<'a>, flags: ScopeFlags) {
        self.function_depth += 1;
        walk_function(self, function, flags);
        self.function_depth -= 1;
    }

    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression<'a>) {
        self.function_depth += 1;
        walk_arrow_function_expression(self, expr);
        self.function_depth -= 1;
    }

    fn visit_class(&mut self, class: &Class<'a>) {
        if self.check_class_state {
            check_class(class, &self.data.reactivity, self.diags);
        }
        walk_class(self, class);
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if self.check_stores {
            check_store_rune_conflict(call, self.data, self.diags);
        }
        walk_call_expression(self, call);
    }

    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if self.check_stores {
            check_legacy_rune_invalid_usage(declarator, self.data, self.diags);
        }
        walk_variable_declarator(self, declarator);
    }

    fn visit_declaration(&mut self, decl: &Declaration<'a>) {
        if let Declaration::ClassDeclaration(class) = decl
            && self.function_depth > 0
        {
            self.diags.push(Diagnostic::warning(
                DiagnosticKind::PerfAvoidNestedClass,
                Span::new(class.span.start, class.span.end),
            ));
        }
        walk_declaration(self, decl);
    }

    fn visit_new_expression(&mut self, expr: &NewExpression<'a>) {
        if self.base_function_depth + self.function_depth > 0
            && matches!(
                expr.callee.get_inner_expression(),
                Expression::ClassExpression(_)
            )
        {
            self.diags.push(Diagnostic::warning(
                DiagnosticKind::PerfAvoidInlineClass,
                Span::new(expr.span.start, expr.span.end),
            ));
        }
        walk_new_expression(self, expr);
    }

    fn visit_labeled_statement(&mut self, stmt: &LabeledStatement<'a>) {
        if self.check_legacy_placement && self.function_depth > 0 && stmt.label.name == "$" {
            self.diags.push(Diagnostic::warning(
                DiagnosticKind::ReactiveDeclarationInvalidPlacement,
                Span::new(stmt.span.start, stmt.span.end),
            ));
        }
        walk_labeled_statement(self, stmt);
    }

    fn visit_method_definition(&mut self, it: &MethodDefinition<'a>) {
        let prev = self.in_constructor;
        if it.kind == MethodDefinitionKind::Constructor {
            self.in_constructor = true;
        }
        walk_method_definition(self, it);
        self.in_constructor = prev;
    }

    fn visit_formal_parameter(&mut self, it: &FormalParameter<'a>) {
        if self.in_constructor && (it.accessibility.is_some() || it.readonly) {
            self.push_ts("accessibility modifiers on constructor parameters", it.span);
        }
    }

    fn visit_decorator(&mut self, it: &Decorator<'a>) {
        self.push_ts(
            "decorators (related TSC proposal is not stage 4 yet)",
            it.span,
        );
    }

    fn visit_accessor_property(&mut self, it: &AccessorProperty<'a>) {
        self.push_ts(
            "accessor fields (related TSC proposal is not stage 4 yet)",
            it.span,
        );
    }

    fn visit_ts_enum_declaration(&mut self, it: &TSEnumDeclaration<'a>) {
        self.push_ts("enums", it.span);
    }

    fn visit_ts_module_declaration(&mut self, it: &TSModuleDeclaration<'a>) {
        if namespace_has_non_type_node(it) {
            self.push_ts("namespaces with non-type nodes", it.span);
        }
    }
}

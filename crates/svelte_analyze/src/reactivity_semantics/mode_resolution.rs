use oxc_ast::ast::{
    ArrowFunctionExpression, AwaitExpression, BindingPattern, Declaration, ForOfStatement,
    Function, MethodDefinition, ModuleExportName, Program, Statement, VariableDeclarationKind,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_await_expression, walk_for_of_statement, walk_function,
    walk_method_definition, walk_program,
};
use oxc_semantic::ScopeFlags;
use rustc_hash::FxHashSet;
use svelte_ast::{RunesMode, RunesOption, is_rune_name};
use svelte_parser::JsAst;

use crate::scope::ComponentScoping;

pub(crate) fn resolve(
    scoping: &ComponentScoping,
    parsed: &JsAst<'_>,
    inline: Option<bool>,
    compile: RunesOption,
) -> RunesMode {
    if let Some(true) = inline {
        return RunesMode::Runes;
    }
    if matches!(inline, Some(false)) {
        return RunesMode::HardLegacy;
    }
    match compile {
        RunesOption::Runes => return RunesMode::Runes,
        RunesOption::Legacy => return RunesMode::HardLegacy,
        RunesOption::Auto => {}
    }
    if resolves_to_runes_via_signals(scoping, parsed) {
        return RunesMode::Runes;
    }
    if has_legacy_signals(scoping, parsed) {
        RunesMode::HardLegacy
    } else {
        RunesMode::SoftLegacy
    }
}

fn resolves_to_runes_via_signals(scoping: &ComponentScoping, parsed: &JsAst<'_>) -> bool {
    if scoping
        .root_unresolved_references()
        .keys()
        .any(|name| is_rune_name(name))
    {
        return true;
    }
    has_top_level_await(parsed.module_program.as_ref())
        || has_top_level_await(parsed.program.as_ref())
}

fn has_legacy_signals(scoping: &ComponentScoping, parsed: &JsAst<'_>) -> bool {
    let unresolved = scoping.root_unresolved_references();
    if unresolved.contains_key("$$props") || unresolved.contains_key("$$restProps") {
        return true;
    }
    let Some(program) = parsed.program.as_ref() else {
        return false;
    };
    let mut top_level_let_names: FxHashSet<&str> = FxHashSet::default();
    for stmt in &program.body {
        if let Statement::VariableDeclaration(decl) = stmt
            && decl.kind == VariableDeclarationKind::Let
        {
            for declarator in &decl.declarations {
                collect_pattern_names(&declarator.id, &mut top_level_let_names);
            }
        }
    }
    for stmt in &program.body {
        match stmt {
            Statement::LabeledStatement(label) if label.label.name == "$" => return true,
            Statement::ExportNamedDeclaration(export) => {
                if let Some(Declaration::VariableDeclaration(decl)) = &export.declaration
                    && decl.kind == VariableDeclarationKind::Let
                {
                    return true;
                }
                for spec in &export.specifiers {
                    if let ModuleExportName::IdentifierName(id) = &spec.local
                        && top_level_let_names.contains(id.name.as_str())
                    {
                        return true;
                    }
                    if let ModuleExportName::IdentifierReference(id) = &spec.local
                        && top_level_let_names.contains(id.name.as_str())
                    {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn collect_pattern_names<'a>(
    pattern: &'a BindingPattern<'a>,
    out: &mut FxHashSet<&'a str>,
) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            out.insert(id.name.as_str());
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_pattern_names(&prop.value, out);
            }
            if let Some(rest) = &obj.rest {
                collect_pattern_names(&rest.argument, out);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for element in arr.elements.iter().flatten() {
                collect_pattern_names(element, out);
            }
            if let Some(rest) = &arr.rest {
                collect_pattern_names(&rest.argument, out);
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            collect_pattern_names(&assign.left, out);
        }
    }
}

fn has_top_level_await(program: Option<&Program<'_>>) -> bool {
    let Some(program) = program else {
        return false;
    };
    let mut visitor = TopLevelAwaitVisitor {
        depth: 0,
        found: false,
    };
    visitor.visit_program(program);
    visitor.found
}

struct TopLevelAwaitVisitor {
    depth: u32,
    found: bool,
}

impl<'a> Visit<'a> for TopLevelAwaitVisitor {
    fn visit_program(&mut self, program: &Program<'a>) {
        walk_program(self, program);
    }

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        self.depth += 1;
        walk_function(self, func, flags);
        self.depth -= 1;
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        self.depth += 1;
        walk_arrow_function_expression(self, arrow);
        self.depth -= 1;
    }

    fn visit_method_definition(&mut self, method: &MethodDefinition<'a>) {
        self.depth += 1;
        walk_method_definition(self, method);
        self.depth -= 1;
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        if self.depth == 0 {
            self.found = true;
            return;
        }
        walk_await_expression(self, expr);
    }

    fn visit_for_of_statement(&mut self, stmt: &ForOfStatement<'a>) {
        if self.depth == 0 && stmt.r#await {
            self.found = true;
            return;
        }
        walk_for_of_statement(self, stmt);
    }
}

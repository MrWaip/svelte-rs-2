use oxc_ast::ast::{
    ArrowFunctionExpression, AwaitExpression, BindingPattern, Declaration, Expression, Function,
    MethodDefinition, ModuleExportName, Program, Statement, VariableDeclarationKind,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_await_expression, walk_function, walk_method_definition,
    walk_program,
};
use oxc_semantic::ScopeFlags;
use rustc_hash::FxHashSet;
use svelte_ast::{RunesMode, RunesOption, is_rune_name};
use svelte_parser::JsAst;

use crate::scope::{ComponentScoping, SymbolId};

pub(crate) fn resolve(
    scoping: &ComponentScoping,
    parsed: &JsAst<'_>,
    inline: Option<bool>,
    compile: RunesOption,
    svelte_store_rune_import: Option<SymbolId>,
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
    if resolves_to_runes_via_signals(scoping, parsed, svelte_store_rune_import) {
        return RunesMode::Runes;
    }
    if has_legacy_signals(scoping, parsed) {
        RunesMode::HardLegacy
    } else {
        RunesMode::SoftLegacy
    }
}

fn resolves_to_runes_via_signals(
    scoping: &ComponentScoping,
    parsed: &JsAst<'_>,
    svelte_store_rune_import: Option<SymbolId>,
) -> bool {
    let mut store_autosub_bases: FxHashSet<String> = FxHashSet::default();
    collect_top_level_non_rune_init_names(parsed.module_program.as_ref(), &mut store_autosub_bases);
    collect_top_level_non_rune_init_names(parsed.program.as_ref(), &mut store_autosub_bases);

    if scoping.root_unresolved_references().keys().any(|name| {
        if !is_rune_name(name) {
            return false;
        }
        let base = &name[1..];
        if store_autosub_bases.contains(base) {
            return false;
        }
        !base_is_import_store_base(scoping, base, svelte_store_rune_import)
    }) {
        return true;
    }
    has_instance_await(parsed) || has_template_await(parsed)
}

fn collect_top_level_non_rune_init_names(
    program: Option<&Program<'_>>,
    out: &mut FxHashSet<String>,
) {
    let Some(program) = program else {
        return;
    };
    for stmt in &program.body {
        let decl = match stmt {
            Statement::VariableDeclaration(d) => &**d,
            Statement::ExportNamedDeclaration(e) => match &e.declaration {
                Some(Declaration::VariableDeclaration(d)) => &**d,
                _ => continue,
            },
            _ => continue,
        };
        for declarator in &decl.declarations {
            if init_is_rune_call(declarator.init.as_ref()) {
                continue;
            }
            collect_pattern_name_strings(&declarator.id, out);
        }
    }
}

fn base_is_import_store_base(
    scoping: &ComponentScoping,
    base: &str,
    svelte_store_rune_import: Option<SymbolId>,
) -> bool {
    let Some(sym) = scoping.find_binding(scoping.root_scope_id(), base) else {
        return false;
    };
    scoping.is_import(sym) && Some(sym) != svelte_store_rune_import
}

fn init_is_rune_call(init: Option<&Expression<'_>>) -> bool {
    let Some(Expression::CallExpression(call)) = init else {
        return false;
    };
    callee_is_rune(&call.callee)
}

fn callee_is_rune(callee: &Expression<'_>) -> bool {
    match callee {
        Expression::Identifier(id) => is_rune_name(id.name.as_str()),
        Expression::StaticMemberExpression(member) => callee_is_rune(&member.object),
        _ => false,
    }
}

fn collect_pattern_name_strings(pattern: &BindingPattern<'_>, out: &mut FxHashSet<String>) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            out.insert(id.name.to_string());
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_pattern_name_strings(&prop.value, out);
            }
            if let Some(rest) = &obj.rest {
                collect_pattern_name_strings(&rest.argument, out);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for element in arr.elements.iter().flatten() {
                collect_pattern_name_strings(element, out);
            }
            if let Some(rest) = &arr.rest {
                collect_pattern_name_strings(&rest.argument, out);
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            collect_pattern_name_strings(&assign.left, out);
        }
    }
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

fn collect_pattern_names<'a>(pattern: &'a BindingPattern<'a>, out: &mut FxHashSet<&'a str>) {
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

fn has_instance_await(parsed: &JsAst<'_>) -> bool {
    let Some(program) = parsed.program.as_ref() else {
        return false;
    };
    let mut visitor = SuspendingAwaitVisitor::default();
    visitor.visit_program(program);
    visitor.found
}

fn has_template_await(parsed: &JsAst<'_>) -> bool {
    let mut visitor = SuspendingAwaitVisitor::default();
    for expr in parsed.iter_exprs() {
        visitor.visit_expression(expr);
        if visitor.found {
            return true;
        }
    }
    for stmt in parsed.iter_stmts() {
        visitor.visit_statement(stmt);
        if visitor.found {
            return true;
        }
    }
    false
}

#[derive(Default)]
struct SuspendingAwaitVisitor {
    depth: u32,
    found: bool,
}

impl<'a> Visit<'a> for SuspendingAwaitVisitor {
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
}

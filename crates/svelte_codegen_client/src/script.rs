use rustc_hash::FxHashSet;

use oxc_allocator::Allocator;
use oxc_ast::ast::{
    ArrowFunctionExpression, Expression, FunctionBody, Program, Statement, VariableDeclarator,
};
use oxc_parser::Parser as OxcParser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};

use svelte_analyze::{ComponentScoping, PropsAnalysis};
use svelte_ast::ScriptLanguage;
use svelte_js::RuneKind;

use crate::builder::{Arg, Builder};
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Props flag constants (must match svelte/src/constants.js)
// ---------------------------------------------------------------------------

const PROPS_IS_IMMUTABLE: u32 = 1;
const PROPS_IS_RUNES: u32 = 1 << 1;
const PROPS_IS_UPDATED: u32 = 1 << 2;
const PROPS_IS_BINDABLE: u32 = 1 << 3;
const PROPS_IS_LAZY_INITIAL: u32 = 1 << 4;

/// Parse and transform the script block.
///
/// Returns `(imports, body, has_tracing)` — imports are extracted separately so they can
/// be hoisted to the top of the generated module.
pub fn gen_script<'a>(ctx: &mut Ctx<'a>, dev: bool) -> (Vec<Statement<'a>>, Vec<Statement<'a>>, bool) {
    if ctx.component.script.is_none() {
        return (vec![], vec![], false);
    };

    let allocator = ctx.b.ast.allocator;
    let component_scoping = &ctx.analysis.scoping;
    let props = ctx.analysis.props.as_ref();
    let component_source = &ctx.component.source;
    let script_content_start = ctx.component.script.as_ref().unwrap().content_span.start;

    // Take pre-parsed Program from analysis (avoids double-parsing)
    if let Some(program) = ctx.parsed.script_program.take() {
        return transform_program(
            allocator,
            program,
            component_scoping,
            props,
            dev,
            component_source,
            script_content_start,
        );
    }

    // Fallback: no pre-parsed program (e.g. tests calling codegen without analysis)
    let script = ctx.component.script.as_ref().unwrap();
    let is_ts = script.language == ScriptLanguage::TypeScript;
    let script_text = ctx.component.source_text(script.content_span);
    transform_script_text(
        allocator,
        script_text,
        is_ts,
        component_scoping,
        props,
        true,
        dev,
        component_source,
        script_content_start,
    )
}

/// Transform a standalone JS/TS module (`.svelte.js`/`.svelte.ts`) applying rune rewrites.
/// Unlike component scripts, exports are preserved (not stripped).
pub fn transform_module_script<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    component_scoping: &ComponentScoping,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let (imports, body, _has_tracing) = transform_script_text(
        allocator,
        source,
        is_ts,
        component_scoping,
        None,
        false,
        false,
        source,
        0,
    );
    (imports, body)
}

/// Parse the script source and apply rune transformations, returning (imports, body, has_tracing).
fn transform_script_text<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    component_scoping: &ComponentScoping,
    props: Option<&PropsAnalysis>,
    strip_exports: bool,
    dev: bool,
    component_source: &str,
    script_content_start: u32,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>, bool) {
    let src_type = if is_ts {
        SourceType::default().with_typescript(true).with_module(true)
    } else {
        SourceType::mjs()
    };
    let result = OxcParser::new(allocator, source, src_type).parse();

    let b = Builder::new(allocator);
    let mut program = result.program;

    // SemanticBuilder populates symbol_id/reference_id on AST nodes,
    // enabling reference resolution during traverse.
    let sem = SemanticBuilder::new().build(&program);
    let scoping = sem.semantic.into_scoping();

    let props_gen = props.map(|pa| PropsGenInfo::from_analysis(pa));

    let mut transformer = ScriptTransformer {
        b: &b,
        component_scoping,
        scoping,
        props_gen,
        derived_pending: FxHashSet::default(),
        strip_exports,
        dev,
        function_info_stack: Vec::new(),
        has_tracing: false,
        component_source,
        script_content_start,
        next_arrow_name: None,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_scoping, ());

    // Post-traverse: wrap $derived arguments in thunks
    if !transformer.derived_pending.is_empty() {
        wrap_derived_thunks(&b, &mut program, &transformer.derived_pending);
    }

    let has_tracing = transformer.has_tracing;

    let mut imports = vec![];
    let mut body = vec![];

    for stmt in program.body {
        if matches!(
            stmt,
            Statement::TSTypeAliasDeclaration(_)
                | Statement::TSInterfaceDeclaration(_)
                | Statement::TSEnumDeclaration(_)
        ) {
            continue;
        }
        if matches!(stmt, Statement::ImportDeclaration(_)) {
            imports.push(stmt);
        } else {
            body.push(stmt);
        }
    }

    (imports, body, has_tracing)
}

/// Transform a pre-parsed Program AST (from analysis), applying rune transformations.
fn transform_program<'a>(
    allocator: &'a Allocator,
    mut program: Program<'a>,
    component_scoping: &ComponentScoping,
    props: Option<&PropsAnalysis>,
    dev: bool,
    component_source: &str,
    script_content_start: u32,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>, bool) {
    let b = Builder::new(allocator);

    // Re-run SemanticBuilder to get fresh scoping matching current AST state
    let sem = SemanticBuilder::new().build(&program);
    let scoping = sem.semantic.into_scoping();

    let props_gen = props.map(|pa| PropsGenInfo::from_analysis(pa));

    let mut transformer = ScriptTransformer {
        b: &b,
        component_scoping,
        scoping,
        props_gen,
        derived_pending: FxHashSet::default(),
        strip_exports: true,
        dev,
        function_info_stack: Vec::new(),
        has_tracing: false,
        component_source,
        script_content_start,
        next_arrow_name: None,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_scoping, ());

    if !transformer.derived_pending.is_empty() {
        wrap_derived_thunks(&b, &mut program, &transformer.derived_pending);
    }

    let has_tracing = transformer.has_tracing;

    let mut imports = vec![];
    let mut body = vec![];

    for stmt in program.body {
        if matches!(
            stmt,
            Statement::TSTypeAliasDeclaration(_)
                | Statement::TSInterfaceDeclaration(_)
                | Statement::TSEnumDeclaration(_)
        ) {
            continue;
        }
        if matches!(stmt, Statement::ImportDeclaration(_)) {
            imports.push(stmt);
        } else {
            body.push(stmt);
        }
    }

    (imports, body, has_tracing)
}

enum PropKind {
    Source,
    NonSource(String),
}

struct PropsGenInfo {
    props: Vec<PropGenItem>,
}

impl PropsGenInfo {
    fn from_analysis(pa: &PropsAnalysis) -> Self {
        PropsGenInfo {
            props: pa.props.iter().map(|p| PropGenItem {
                local_name: p.local_name.clone(),
                prop_name: p.prop_name.clone(),
                is_prop_source: p.is_prop_source,
                is_bindable: p.is_bindable,
                is_rest: p.is_rest,
                is_mutated: p.is_mutated,
                default_text: p.default_text.clone(),
                is_lazy_default: p.is_lazy_default,
            }).collect(),
        }
    }
}

struct PropGenItem {
    local_name: String,
    prop_name: String,
    is_prop_source: bool,
    is_bindable: bool,
    is_rest: bool,
    is_mutated: bool,
    default_text: Option<String>,
    is_lazy_default: bool,
}

struct FunctionInfo {
    is_async: bool,
    name: Option<String>,
    /// Byte offset of the function keyword in the script source (for auto-label location).
    span_start: u32,
}

struct ScriptTransformer<'b, 'a> {
    b: &'b Builder<'a>,
    /// ComponentScoping — source of truth for rune kind + mutation status.
    component_scoping: &'b ComponentScoping,
    /// OXC scoping from SemanticBuilder — used to resolve references to symbols.
    scoping: Scoping,
    props_gen: Option<PropsGenInfo>,
    /// SymbolIds of $derived/$derived.by runes whose init needs post-traverse wrapping.
    derived_pending: FxHashSet<oxc_semantic::SymbolId>,
    /// Whether to strip `export` keywords from declarations. True for component scripts,
    /// false for module compilation where exports must be preserved.
    strip_exports: bool,
    /// Whether dev-mode transforms are enabled ($inspect → $.inspect).
    dev: bool,
    /// Stack tracking enclosing functions for $inspect.trace() context.
    function_info_stack: Vec<FunctionInfo>,
    /// Whether any $inspect.trace() was found (dev mode), triggers tracing import.
    has_tracing: bool,
    /// Full component source for line/col computation.
    component_source: &'b str,
    /// Byte offset of script content within the full component source.
    script_content_start: u32,
    /// Captured variable name for arrow functions (from VariableDeclarator).
    next_arrow_name: Option<String>,
}

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    /// Resolve a binding identifier to its rune kind and mutated status.
    /// Only root-scope symbols are considered runes (skips shadowing parameters).
    fn rune_for_binding(
        &self,
        id: &oxc_ast::ast::BindingIdentifier<'a>,
    ) -> Option<(RuneKind, bool)> {
        let sym_id = id.symbol_id.get()?;
        if self.scoping.symbol_scope_id(sym_id) != self.scoping.root_scope_id() {
            return None;
        }
        // OXC SemanticBuilder produces identical SymbolIds for the same script source,
        // so we can use sym_id directly against ComponentScoping without name round-trip.
        let kind = self.component_scoping.rune_kind(sym_id)?;
        Some((kind, self.component_scoping.is_mutated(sym_id)))
    }

    /// Resolve a reference identifier to its rune kind and mutated status.
    fn rune_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<(RuneKind, bool)> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.scoping.get_reference(ref_id).symbol_id()?;
        if self.scoping.symbol_scope_id(sym_id) != self.scoping.root_scope_id() {
            return None;
        }
        let kind = self.component_scoping.rune_kind(sym_id)?;
        Some((kind, self.component_scoping.is_mutated(sym_id)))
    }

    /// Resolve a reference identifier to its prop kind (source or non-source).
    fn prop_kind_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<PropKind> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.scoping.get_reference(ref_id).symbol_id()?;
        if self.scoping.symbol_scope_id(sym_id) != self.scoping.root_scope_id() {
            return None;
        }
        if self.component_scoping.is_prop_source(sym_id) {
            Some(PropKind::Source)
        } else if let Some(prop_name) = self.component_scoping.prop_non_source_name(sym_id) {
            Some(PropKind::NonSource(prop_name.to_string()))
        } else {
            None
        }
    }

    fn should_proxy(e: &Expression) -> bool {
        if e.is_literal() {
            return false;
        }
        if matches!(
            e,
            Expression::TemplateLiteral(_)
                | Expression::ArrowFunctionExpression(_)
                | Expression::FunctionExpression(_)
                | Expression::UnaryExpression(_)
                | Expression::BinaryExpression(_)
        ) {
            return false;
        }
        if let Expression::Identifier(id) = e {
            if id.name == "undefined" {
                return false;
            }
        }
        true
    }

    fn is_props_declaration(decl: &oxc_ast::ast::VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            if let oxc_ast::ast::BindingPattern::ObjectPattern(_) = &d.id {
                if let Some(init) = &d.init {
                    if let Expression::CallExpression(call) = init {
                        if let Expression::Identifier(ident) = &call.callee {
                            return ident.name.as_str() == "$props";
                        }
                    }
                }
            }
            false
        })
    }

    fn is_props_id_declaration(decl: &oxc_ast::ast::VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            if let oxc_ast::ast::BindingPattern::BindingIdentifier(_) = &d.id {
                if let Some(Expression::CallExpression(call)) = &d.init {
                    if let Expression::StaticMemberExpression(member) = &call.callee {
                        if let Expression::Identifier(obj) = &member.object {
                            return obj.name.as_str() == "$props"
                                && member.property.name.as_str() == "id";
                        }
                    }
                }
            }
            false
        })
    }

    fn gen_props_statements(&self) -> Vec<Statement<'a>> {
        let Some(props_gen) = &self.props_gen else {
            return vec![];
        };

        let mut declarators: Vec<(&str, Expression<'a>)> = Vec::new();
        let mut seen_names: Vec<String> = vec![
            "$$slots".to_string(),
            "$$events".to_string(),
            "$$legacy".to_string(),
        ];

        for prop in &props_gen.props {
            seen_names.push(prop.prop_name.clone());

            if prop.is_rest {
                let excluded: Vec<Arg<'a, '_>> = seen_names.iter()
                    .filter(|n| *n != &prop.local_name)
                    .map(|n| Arg::Str(n.clone()))
                    .collect();
                let arr_expr = self.b.array_from_args(excluded);
                let init = self.b.call_expr("$.rest_props", [
                    Arg::Ident("$$props"),
                    Arg::Expr(arr_expr),
                ]);
                declarators.push((self.b.alloc_str(&prop.local_name), init));
                continue;
            }

            if !prop.is_prop_source {
                continue;
            }

            let mut flags: u32 = PROPS_IS_IMMUTABLE | PROPS_IS_RUNES;
            if prop.is_bindable {
                flags |= PROPS_IS_BINDABLE;
            }
            if prop.is_mutated {
                flags |= PROPS_IS_UPDATED;
            }

            let mut args: Vec<Arg<'a, '_>> = vec![
                Arg::Ident("$$props"),
                Arg::Str(prop.prop_name.clone()),
            ];

            if let Some(default_text) = &prop.default_text {
                if prop.is_lazy_default {
                    flags |= PROPS_IS_LAZY_INITIAL;
                }

                args.push(Arg::Num(flags as f64));

                // Parse default expression
                let default_expr = parse_expression(self.b, default_text);
                // Wrap $bindable() defaults in $.proxy() when needed
                let default_expr = if prop.is_bindable && Self::should_proxy(&default_expr) {
                    self.b.call_expr("$.proxy", [Arg::Expr(default_expr)])
                } else {
                    default_expr
                };
                if !prop.is_lazy_default {
                    args.push(Arg::Expr(default_expr));
                } else {
                    args.push(Arg::Expr(wrap_lazy(self.b, default_expr)));
                }
            } else {
                if flags != 0 {
                    args.push(Arg::Num(flags as f64));
                }
            }

            let name: &'a str = self.b.alloc_str(&prop.local_name);
            declarators.push((name, self.b.call_expr("$.prop", args)));
        }

        if declarators.is_empty() {
            return vec![];
        }

        vec![self.b.let_multi_stmt(declarators)]
    }
}

/// Post-traverse: wrap `$.derived(expr)` → `$.derived(() => expr)` for $derived runes.
fn wrap_derived_thunks<'a>(
    b: &Builder<'a>,
    program: &mut oxc_ast::ast::Program<'a>,
    pending: &FxHashSet<oxc_semantic::SymbolId>,
) {
    use oxc_ast::ast::Statement;
    for stmt in program.body.iter_mut() {
        if let Statement::VariableDeclaration(decl) = stmt {
            for declarator in decl.declarations.iter_mut() {
                let sym_id = match &declarator.id {
                    oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                        match id.symbol_id.get() {
                            Some(s) => s,
                            None => continue,
                        }
                    }
                    _ => continue,
                };
                if !pending.contains(&sym_id) {
                    continue;
                }
                if let Some(Expression::CallExpression(call)) = &mut declarator.init {
                    if !call.arguments.is_empty() {
                        let mut dummy = oxc_ast::ast::Argument::from(b.cheap_expr());
                        std::mem::swap(&mut call.arguments[0], &mut dummy);
                        let arg_expr = dummy.into_expression();
                        let thunk = b.thunk(arg_expr);
                        call.arguments[0] = oxc_ast::ast::Argument::from(thunk);
                    }
                }
            }
        }
    }
}

fn parse_expression<'a>(b: &Builder<'a>, text: &str) -> Expression<'a> {
    let alloc = b.ast.allocator;
    // Allocate text in the arena so it lives long enough for OXC parsing
    let arena_text: &'a str = alloc.alloc_str(text);
    match OxcParser::new(alloc, arena_text, SourceType::default()).parse_expression() {
        Ok(expr) => expr,
        Err(_) => {
            debug_assert!(false, "codegen: failed to parse expression: {text}");
            eprintln!("[svelte-rs] warning: failed to parse expression in script codegen: {text}");
            b.str_expr(text)
        }
    }
}

/// Wrap a non-simple default expression for lazy evaluation.
fn wrap_lazy<'a>(b: &Builder<'a>, expr: Expression<'a>) -> Expression<'a> {
    // Zero-arg call foo() → use callee directly (already lazy)
    if let Expression::CallExpression(call) = &expr {
        if call.arguments.is_empty() {
            if let Expression::Identifier(_) = &call.callee {
                return b.clone_expr(&call.callee);
            }
        }
    }
    // Otherwise wrap: () => expr
    b.arrow_expr(b.no_params(), [b.expr_stmt(expr)])
}

/// Check if an expression is a `$inspect(...)`, `$inspect(...).with(...)`, or `$inspect.trace(...)` call.
fn is_inspect_call(expr: &Expression) -> bool {
    match expr {
        Expression::CallExpression(call) => {
            if let Expression::Identifier(id) = &call.callee {
                if id.name.as_str() == "$inspect" {
                    return true;
                }
            }
            if let Expression::StaticMemberExpression(member) = &call.callee {
                // $inspect(...).with(...)
                if member.property.name.as_str() == "with" {
                    if let Expression::CallExpression(inner) = &member.object {
                        if let Expression::Identifier(id) = &inner.callee {
                            return id.name.as_str() == "$inspect";
                        }
                    }
                }
                // $inspect.trace(...)
                if member.property.name.as_str() == "trace" {
                    if let Expression::Identifier(id) = &member.object {
                        return id.name.as_str() == "$inspect";
                    }
                }
            }
            false
        }
        _ => false,
    }
}

/// Check if an expression is specifically a `$inspect.trace(...)` call.
fn is_inspect_trace_call(expr: &Expression) -> bool {
    if let Expression::CallExpression(call) = expr {
        if let Expression::StaticMemberExpression(member) = &call.callee {
            if member.property.name.as_str() == "trace" {
                if let Expression::Identifier(id) = &member.object {
                    return id.name.as_str() == "$inspect";
                }
            }
        }
    }
    false
}

/// Compute 1-based line and column from source text and byte offset.
fn compute_line_col(source: &str, offset: u32) -> (usize, usize) {
    let offset = offset as usize;
    let bytes = source.as_bytes();
    let mut line = 1;
    let mut col = 0;
    for i in 0..offset.min(bytes.len()) {
        if bytes[i] == b'\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

impl<'a> Traverse<'a, ()> for ScriptTransformer<'_, 'a> {
    fn enter_function(
        &mut self,
        node: &mut oxc_ast::ast::Function<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let name = node.id.as_ref().map(|id| id.name.to_string());
        self.function_info_stack.push(FunctionInfo {
            is_async: node.r#async,
            name,
            span_start: node.span.start,
        });
    }

    fn exit_function(
        &mut self,
        _node: &mut oxc_ast::ast::Function<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.function_info_stack.pop();
    }

    fn enter_arrow_function_expression(
        &mut self,
        node: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let name = self.next_arrow_name.take();
        self.function_info_stack.push(FunctionInfo {
            is_async: node.r#async,
            name,
            span_start: node.span.start,
        });
    }

    fn exit_arrow_function_expression(
        &mut self,
        _node: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.function_info_stack.pop();
    }

    fn exit_function_body(
        &mut self,
        body: &mut FunctionBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if !self.dev {
            return;
        }
        let Some(Statement::ExpressionStatement(es)) = body.statements.first() else {
            return;
        };
        if !is_inspect_trace_call(&es.expression) {
            return;
        }

        let info = self.function_info_stack.last()
            .unwrap_or_else(|| panic!("$inspect.trace() outside function"));

        // Extract explicit label argument if present
        let trace_stmt = body.statements.remove(0);
        let Statement::ExpressionStatement(es) = trace_stmt else { unreachable!() };
        let Expression::CallExpression(call) = es.unbox().expression else { unreachable!() };
        let mut call = call.unbox();

        let label_expr = if !call.arguments.is_empty() {
            // Explicit label: $inspect.trace("custom") → use the argument directly
            let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
            std::mem::swap(&mut call.arguments[0], &mut dummy);
            dummy.into_expression()
        } else {
            // Auto-label: "funcName (line:col)"
            let func_name = info.name.as_deref().unwrap_or("trace");
            let full_offset = self.script_content_start + info.span_start;
            let (line, col) = compute_line_col(self.component_source, full_offset);
            let label = format!("{func_name} ((unknown):{line}:{col})");
            self.b.str_expr(&label)
        };
        let label_thunk = self.b.thunk(label_expr);

        let is_async = info.is_async;

        // Take remaining statements, build body thunk
        let remaining: Vec<Statement<'a>> = body.statements.drain(..).collect();
        let body_thunk = if is_async {
            self.b.async_thunk_block(remaining)
        } else {
            self.b.thunk_block(remaining)
        };

        // Build: return [await] $.trace(label, bodyThunk)
        let trace_call = self.b.call_expr("$.trace", [
            Arg::Expr(label_thunk),
            Arg::Expr(body_thunk),
        ]);
        let return_expr = if is_async {
            self.b.await_expr(trace_call)
        } else {
            trace_call
        };
        body.statements.push(self.b.return_stmt(return_expr));

        self.has_tracing = true;
    }

    fn exit_statements(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Strip `export` keyword: ExportNamedDeclaration → inner declaration
        // (only for component scripts; module compilation preserves exports)
        if self.strip_exports {
            let mut i = 0;
            while i < stmts.len() {
                if let Statement::ExportNamedDeclaration(_) = &stmts[i] {
                    let stmt = stmts.remove(i);
                    if let Statement::ExportNamedDeclaration(export) = stmt {
                        if let Some(decl) = export.unbox().declaration {
                            stmts.insert(i, Statement::from(decl));
                            i += 1;
                        }
                        // else: `export { x }` form — just remove
                    }
                } else {
                    i += 1;
                }
            }
        }

        // Prod strip: $inspect.trace() → remove entirely; $inspect()/$inspect().with() → 2 EmptyStatements
        if !self.dev {
            let mut i = 0;
            while i < stmts.len() {
                if let Statement::ExpressionStatement(es) = &stmts[i] {
                    if is_inspect_trace_call(&es.expression) {
                        stmts.remove(i);
                        continue;
                    }
                    if is_inspect_call(&es.expression) {
                        stmts[i] = Statement::EmptyStatement(self.b.ast.alloc_empty_statement(oxc_span::SPAN));
                        stmts.insert(i + 1, Statement::EmptyStatement(self.b.ast.alloc_empty_statement(oxc_span::SPAN)));
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }
        }

        // Strip $props.id() declarations (regenerated at top of component fn_body)
        stmts.retain(|stmt| {
            if let Statement::VariableDeclaration(decl) = stmt {
                if Self::is_props_id_declaration(decl) {
                    return false;
                }
            }
            true
        });

        // Replace $props() destructuring
        if self.props_gen.is_none() {
            return;
        }

        let mut idx = None;
        for (j, stmt) in stmts.iter().enumerate() {
            if let Statement::VariableDeclaration(decl) = stmt {
                if Self::is_props_declaration(decl) {
                    idx = Some(j);
                    break;
                }
            }
        }

        let Some(j) = idx else { return };

        let replacement = self.gen_props_statements();
        stmts.remove(j);
        for (k, stmt) in replacement.into_iter().enumerate() {
            stmts.insert(j + k, stmt);
        }
    }

    fn enter_variable_declarator(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Capture variable name for arrow functions (used by $inspect.trace auto-label)
        if let Some(Expression::ArrowFunctionExpression(_)) = &node.init {
            if let oxc_ast::ast::BindingPattern::BindingIdentifier(id) = &node.id {
                self.next_arrow_name = Some(id.name.to_string());
            }
        }

        let rune_info = match &node.id {
            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                self.rune_for_binding(id)
            }
            _ => return,
        };

        let Some((kind, mutated)) = rune_info else {
            return;
        };

        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);

        if let Expression::CallExpression(mut call) = init_expr {
            match kind {
                RuneKind::Derived => {
                    // $derived(expr) → $.derived(() => expr)
                    // Just rewrite callee here; thunk wrapping happens post-traverse
                    // to avoid OXC scope_id issues with new arrow nodes.
                    call.callee = self.b.rid_expr("$.derived");
                    if let oxc_ast::ast::BindingPattern::BindingIdentifier(bid) = &node.id {
                        if let Some(sym_id) = bid.symbol_id.get() {
                            self.derived_pending.insert(sym_id);
                        }
                    }
                    node.init = Some(Expression::CallExpression(call));
                }
                RuneKind::DerivedBy => {
                    // $derived.by(fn) → $.derived(fn)
                    call.callee = self.b.rid_expr("$.derived");
                    node.init = Some(Expression::CallExpression(call));
                }
                RuneKind::State | RuneKind::StateRaw => {
                    if mutated {
                        call.callee = self.b.rid_expr("$.state");

                        if call.arguments.is_empty() {
                            let void_zero = self.b.ast.expression_unary(
                                oxc_span::SPAN,
                                oxc_ast::ast::UnaryOperator::Void,
                                self.b.num_expr(0.0),
                            );
                            call.arguments.push(void_zero.into());
                        }

                        let state_expr = Expression::CallExpression(call);
                        // In dev mode, wrap $.state() in $.tag() for debugging
                        node.init = if self.dev {
                            let var_name = match &node.id {
                                oxc_ast::ast::BindingPattern::BindingIdentifier(id) => id.name.as_str(),
                                _ => "state",
                            };
                            Some(self.b.call_expr("$.tag", [Arg::Expr(state_expr), Arg::Str(var_name.to_string())]))
                        } else {
                            Some(state_expr)
                        };
                    } else {
                        let value = if call.arguments.is_empty() {
                            self.b.ast.expression_unary(
                                oxc_span::SPAN,
                                oxc_ast::ast::UnaryOperator::Void,
                                self.b.num_expr(0.0),
                            )
                        } else {
                            let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                            std::mem::swap(&mut call.arguments[0], &mut dummy);
                            dummy.into_expression()
                        };
                        let value = if kind == RuneKind::State && Self::should_proxy(&value) {
                            self.b.call_expr("$.proxy", [Arg::Expr(value)])
                        } else {
                            value
                        };
                        node.init = Some(value);
                    }
                }
                _ => {
                    // Other rune kinds — put back the call unchanged
                    node.init = Some(Expression::CallExpression(call));
                }
            }
        }
    }

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        match node {
            Expression::AssignmentExpression(_) => {
                self.transform_assignment(node, ctx);
            }
            Expression::UpdateExpression(_) => {
                self.transform_update(node, ctx);
            }
            Expression::CallExpression(_) => {
                let Expression::CallExpression(call) = node else { return };

                // $host() → $$props.$$host (entire expression replacement, not callee rename)
                if let Expression::Identifier(id) = &call.callee {
                    if id.name.as_str() == "$host" {
                        *node = self.b.static_member_expr(
                            self.b.rid_expr("$$props"),
                            "$$host",
                        );
                        return;
                    }
                }

                let new_callee = match &call.callee {
                    Expression::Identifier(id) if id.name.as_str() == "$effect" => {
                        Some("$.user_effect")
                    }
                    Expression::StaticMemberExpression(member) => {
                        if let Expression::Identifier(obj) = &member.object {
                            match (obj.name.as_str(), member.property.name.as_str()) {
                                ("$effect", "pre") => Some("$.user_pre_effect"),
                                ("$effect", "root") => Some("$.effect_root"),
                                ("$state", "snapshot") => Some("$.snapshot"),
                                ("$effect", "tracking") => Some("$.effect_tracking"),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(callee_name) = new_callee {
                    let Expression::CallExpression(call) = node else { unreachable!() };
                    call.callee = self.b.rid_expr(callee_name);
                }
            }
            Expression::Identifier(id) => {
                // Check props first
                if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                    match prop_kind {
                        PropKind::Source => {
                            let name = id.name.as_str().to_string();
                            *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                        }
                        PropKind::NonSource(prop_name) => {
                            *node = self.b.static_member_expr(
                                self.b.rid_expr("$$props"),
                                &prop_name,
                            );
                        }
                    }
                    return;
                }
                // Store subscription read: $count → $count()
                // Only transform original source identifiers (with reference_id),
                // not synthetic ones we create during transformation.
                let id_name = id.name.as_str();
                if id.reference_id.get().is_some() && self.component_scoping.is_store_ref(id_name) {
                    let name = id_name.to_string();
                    *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                    return;
                }
                // Regular rune check
                let Some((kind, mutated)) = self.rune_for_ref(id) else {
                    return;
                };
                let needs_get = mutated
                    || kind.is_derived();
                if needs_get {
                    let name = id.name.as_str().to_string();
                    *node = svelte_transform::rune_refs::make_rune_get(self.b.ast.allocator, &name);
                }
            }
            _ => {}
        }
    }

    fn exit_expression(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        if self.dev {
            if let Some(replacement) = self.transform_inspect(node) {
                *node = replacement;
                return;
            }
            // await expr → (await $.track_reactivity_loss(expr))()
            if let Expression::AwaitExpression(await_expr) = node {
                let arg = self.b.move_expr(&mut await_expr.argument);
                let track_call = self.b.call_expr("$.track_reactivity_loss", [Arg::Expr(arg)]);
                let awaited = self.b.await_expr(track_call);
                *node = self.b.call_expr_callee(awaited, std::iter::empty::<Arg<'a, '_>>());
            }
        }
    }
}

impl<'a> ScriptTransformer<'_, 'a> {
    /// Transform `$inspect(args)` → `$.inspect(thunk, inspector, true)`
    /// and `$inspect(args).with(cb)` → `$.inspect(thunk, inspector)`.
    ///
    /// Called in exit_expression (bottom-up), so for `.with()` the inner
    /// `$inspect(...)` has already been transformed to `$.inspect(...)`.
    fn transform_inspect(&self, node: &mut Expression<'a>) -> Option<Expression<'a>> {
        let Expression::CallExpression(outer_call) = node else { return None };

        // Case 1: $.inspect(thunk, inspector, true).with(cb)
        // The inner call was already transformed from $inspect → $.inspect by exit_expression
        if let Expression::StaticMemberExpression(member) = &outer_call.callee {
            if member.property.name.as_str() == "with" {
                if let Expression::CallExpression(inner_call) = &member.object {
                    if let Expression::Identifier(id) = &inner_call.callee {
                        if id.name.as_str() == "$.inspect" {
                            let Expression::CallExpression(outer_call) = node else { unreachable!() };

                            let cb = if outer_call.arguments.is_empty() {
                                self.b.rid_expr("undefined")
                            } else {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut outer_call.arguments[0], &mut dummy);
                                dummy.into_expression()
                            };

                            // Take the thunk from the already-transformed inner call
                            let Expression::StaticMemberExpression(member) = self.b.move_expr(&mut outer_call.callee) else { unreachable!() };
                            let member = member.unbox();
                            let Expression::CallExpression(inner_call) = member.object else { unreachable!() };
                            let mut inner_call = inner_call.unbox();

                            // First arg of inner $.inspect is the thunk
                            let thunk = {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut inner_call.arguments[0], &mut dummy);
                                dummy.into_expression()
                            };

                            let inspector = self.build_inspect_arrow(cb);
                            return Some(self.b.call_expr("$.inspect", [
                                Arg::Expr(thunk),
                                Arg::Expr(inspector),
                            ]));
                        }
                    }
                }
            }
        }

        // Case 2: plain $inspect(args)
        if let Expression::Identifier(id) = &outer_call.callee {
            if id.name.as_str() == "$inspect" {
                let Expression::CallExpression(call) = node else { unreachable!() };
                let inspect_args: Vec<Expression<'a>> = call.arguments.drain(..)
                    .map(|a| a.into_expression())
                    .collect();

                let thunk = self.build_inspect_thunk(inspect_args);
                let console_log = self.b.static_member_expr(self.b.rid_expr("console"), "log");
                let log_call = self.b.call_expr_callee(console_log, [
                    Arg::Spread(self.b.rid_expr("$$args")),
                ]);
                let inspector = self.b.arrow_expr(
                    self.b.rest_params("$$args"),
                    [self.b.expr_stmt(log_call)],
                );

                return Some(self.b.call_expr("$.inspect", [
                    Arg::Expr(thunk),
                    Arg::Expr(inspector),
                    Arg::Bool(true),
                ]));
            }
        }

        None
    }

    /// Build `() => [arg1, arg2, ...]` thunk for inspect args.
    fn build_inspect_thunk(&self, args: Vec<Expression<'a>>) -> Expression<'a> {
        let array_args: Vec<Arg<'a, '_>> = args.into_iter().map(Arg::Expr).collect();
        let array = self.b.array_from_args(array_args);
        self.b.arrow_expr(self.b.no_params(), [self.b.expr_stmt(array)])
    }

    /// Build `(...$$args) => cb(...$$args)` arrow for inspect callback.
    fn build_inspect_arrow(&self, cb: Expression<'a>) -> Expression<'a> {
        let call = self.b.call_expr_callee(cb, [
            Arg::Spread(self.b.rid_expr("$$args")),
        ]);
        self.b.arrow_expr(
            self.b.rest_params("$$args"),
            [self.b.expr_stmt(call)],
        )
    }

    fn transform_assignment(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        let Expression::AssignmentExpression(assign) = node else {
            return;
        };

        if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
            if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                if matches!(prop_kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);
                    *node = self.b.call_expr(&name, [Arg::Expr(right)]);
                    return;
                }
            }
            // Store subscription assignment: $count = val → $.store_set(count, val)
            // Compound: $count += val → $.store_set(count, $count() + val)
            let id_name = id.name.as_str();
            if self.component_scoping.is_store_ref(id_name) {
                let base_name: &str = self.b.alloc_str(&id_name[1..]);
                let dollar_name: &str = self.b.alloc_str(id_name);
                let right = self.b.move_expr(&mut assign.right);

                let value = if assign.operator.is_assign() {
                    right
                } else {
                    // Read current value via thunk call: $count()
                    let current = self.b.call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                    if let Some(bin_op) = assign.operator.to_binary_operator() {
                        self.b.ast.expression_binary(oxc_span::SPAN, current, bin_op, right)
                    } else if let Some(log_op) = assign.operator.to_logical_operator() {
                        self.b.ast.expression_logical(oxc_span::SPAN, current, log_op, right)
                    } else {
                        unreachable!("all compound assignment operators are either binary or logical")
                    }
                };

                *node = self.b.call_expr("$.store_set", [
                    Arg::Ident(base_name),
                    Arg::Expr(value),
                ]);
                return;
            }
            if let Some((kind, mutated)) = self.rune_for_ref(id) {
                if mutated {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);

                    // Expand compound assignments: value += x → $.set(value, $.get(value) + x)
                    let value = if assign.operator.is_assign() {
                        right
                    } else {
                        let left_get = svelte_transform::rune_refs::make_rune_get(self.b.ast.allocator, &name);
                        if let Some(bin_op) = assign.operator.to_binary_operator() {
                            self.b.ast.expression_binary(oxc_span::SPAN, left_get, bin_op, right)
                        } else if let Some(log_op) = assign.operator.to_logical_operator() {
                            self.b.ast.expression_logical(oxc_span::SPAN, left_get, log_op, right)
                        } else {
                            unreachable!("all compound assignment operators are either binary or logical")
                        }
                    };

                    let needs_proxy = kind != RuneKind::StateRaw && Self::should_proxy(&value);
                    *node = svelte_transform::rune_refs::make_rune_set(self.b.ast.allocator, &name, value, needs_proxy);
                    return;
                }
            }
        }
    }

    fn transform_update(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };

        if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
            if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                if matches!(prop_kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let fn_name = if upd.prefix { "$.update_pre_prop" } else { "$.update_prop" };
                    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&name)];
                    if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                        args.push(Arg::Num(-1.0));
                    }
                    *node = self.b.call_expr(fn_name, args);
                    return;
                }
            }
            // Store subscription update: $count++ → $.update_store(count, $count())
            // ++$count → $.update_pre_store(count, $count())
            // $count-- → $.update_store(count, $count(), -1)
            let id_name = id.name.as_str();
            if self.component_scoping.is_store_ref(id_name) {
                let base_name: &str = self.b.alloc_str(&id_name[1..]);
                let dollar_name: &str = self.b.alloc_str(id_name);
                let fn_name = if upd.prefix { "$.update_pre_store" } else { "$.update_store" };
                let thunk_call = self.b.call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                let mut args: Vec<Arg<'a, '_>> = vec![
                    Arg::Ident(base_name),
                    Arg::Expr(thunk_call),
                ];
                if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                    args.push(Arg::Num(-1.0));
                }
                *node = self.b.call_expr(fn_name, args);
                return;
            }
            if let Some((_, mutated)) = self.rune_for_ref(id) {
                if mutated {
                    let name = id.name.as_str().to_string();
                    let is_increment = upd.operator == oxc_ast::ast::UpdateOperator::Increment;
                    *node = svelte_transform::rune_refs::make_rune_update(
                        self.b.ast.allocator, &name, upd.prefix, is_increment,
                    );
                    return;
                }
            }
        }
    }
}

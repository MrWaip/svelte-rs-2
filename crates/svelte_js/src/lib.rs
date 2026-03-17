//! OXC facade for JavaScript analysis.
//!
//! All OXC allocator lifetimes are contained within function calls.
//! Only owned data is returned.

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan as _, SourceType};

use compact_str::CompactString;
use oxc_semantic::SymbolId;
use svelte_span::Span;
use svelte_diagnostics::Diagnostic;

/// Convert OXC Atom to CompactString without intermediate String allocation.
#[inline]
fn compact(s: &str) -> CompactString {
    CompactString::from(s)
}

// ---------------------------------------------------------------------------
// Public types (owned, no lifetime)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ExpressionInfo {
    pub kind: ExpressionKind,
    pub references: Vec<Reference>,
    pub has_side_effects: bool,
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub name: CompactString,
    pub span: Span,
    pub flags: ReferenceFlags,
    /// Resolved after `resolve_references` pass. `None` for globals/unresolved.
    pub symbol_id: Option<SymbolId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceFlags {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    Identifier(CompactString),
    Literal,
    CallExpression { callee: CompactString },
    MemberExpression,
    ArrowFunction,
    Assignment,
    Other,
}

#[derive(Debug, Clone)]
pub struct PropInfo {
    pub local_name: CompactString,
    pub prop_name: CompactString,
    pub default_span: Option<Span>,
    /// The raw text of the default expression (for codegen to parse).
    pub default_text: Option<String>,
    pub is_bindable: bool,
    pub is_rest: bool,
}

#[derive(Debug, Clone)]
pub struct PropsDeclaration {
    pub props: Vec<PropInfo>,
}

#[derive(Debug, Clone)]
pub struct ExportInfo {
    pub name: CompactString,
    pub alias: Option<CompactString>,
}

#[derive(Debug, Clone)]
pub struct ScriptInfo {
    pub declarations: Vec<DeclarationInfo>,
    pub props_declaration: Option<PropsDeclaration>,
    pub exports: Vec<ExportInfo>,
    /// True when the script contains `$effect(...)` or `$effect.pre(...)` calls.
    pub has_effects: bool,
    /// Base names of `$`-prefixed identifiers found in the script body
    /// (e.g. `"count"` for `$count`). Used to detect store subscriptions.
    pub store_candidates: Vec<CompactString>,
}

#[derive(Debug, Clone)]
pub struct DeclarationInfo {
    pub name: CompactString,
    pub span: Span,
    pub kind: DeclarationKind,
    pub init_span: Option<Span>,
    pub is_rune: Option<RuneKind>,
    /// For $derived/$derived.by: names referenced in the init expression.
    pub rune_init_refs: Vec<CompactString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    Let,
    Const,
    Var,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuneKind {
    State,
    StateRaw,
    Derived,
    DerivedBy,
    Effect,
    EffectTracking,
    Props,
    Bindable,
    Inspect,
    Host,
}

impl RuneKind {
    pub fn is_derived(&self) -> bool {
        matches!(self, RuneKind::Derived | RuneKind::DerivedBy)
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse a `{@const name = expr}` declaration via OXC.
///
/// `source` is the raw declaration text (e.g. `"doubled = item * 2"` or `"{a, b} = obj"`).
/// `offset` is `declaration_span.start` in the original .svelte file.
///
/// Returns binding names, references from the init expression, and the init `Expression` AST.
pub fn parse_const_declaration_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
) -> Result<(Vec<CompactString>, Vec<Reference>, Expression<'a>), Diagnostic> {
    // Wrap as "const {source};" so OXC can parse it as a full statement
    let wrapped_owned = format!("const {};", source);
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);
    let prefix_len: u32 = 6; // "const "

    let result = OxcParser::new(alloc, wrapped_str, SourceType::default()).parse();

    if !result.errors.is_empty() {
        return Err(Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32)));
    }

    let program = result.program;
    let stmt = program.body.into_iter().next()
        .ok_or_else(|| Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32)))?;

    let oxc_ast::ast::Statement::VariableDeclaration(mut var_decl) = stmt else {
        return Err(Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32)));
    };

    let mut declarator = var_decl.declarations.remove(0);

    let mut names = Vec::new();
    extract_all_binding_names(&declarator.id, &mut names);

    let init = declarator.init.take()
        .ok_or_else(|| Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32)))?;

    // OXC spans are relative to the wrapped string; adjust by subtracting the prefix
    let ref_offset = offset.wrapping_sub(prefix_len);
    let mut references = Vec::new();
    collect_references(&init, ref_offset, &mut references);

    Ok((names, references, init))
}

fn extract_all_binding_names(pattern: &oxc_ast::ast::BindingPattern<'_>, names: &mut Vec<CompactString>) {
    use oxc_ast::ast::BindingPattern;
    match pattern {
        BindingPattern::BindingIdentifier(id) => names.push(compact(&id.name)),
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                extract_all_binding_names(&prop.value, names);
            }
            if let Some(rest) = &obj.rest {
                extract_all_binding_names(&rest.argument, names);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                extract_all_binding_names(elem, names);
            }
            if let Some(rest) = &arr.rest {
                extract_all_binding_names(&rest.argument, names);
            }
        }
        BindingPattern::AssignmentPattern(assign) => extract_all_binding_names(&assign.left, names),
    }
}

/// Parse a JS expression into a provided allocator, returning both metadata and AST.
///
/// The `Expression<'a>` lives in the provided allocator (not destroyed after call).
/// Use this when you need to keep the parsed AST for later transformation/codegen.
pub fn analyze_expression_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
) -> Result<(ExpressionInfo, Expression<'a>), Diagnostic> {
    let parser = OxcParser::new(alloc, source, SourceType::default());
    let expr = parser
        .parse_expression()
        .map_err(|_| Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32)))?;
    let info = extract_expression_info(&expr, offset);
    Ok((info, expr))
}

/// Parse a JS expression and return owned analysis info.
///
/// `source` is the raw expression text (e.g., "count + 1").
/// `offset` is the byte offset in the original .svelte file (for Span adjustment).
///
/// OXC allocator is created and destroyed inside this function.
pub fn analyze_expression(source: &str, offset: u32) -> Result<ExpressionInfo, Diagnostic> {
    let allocator = Allocator::default();
    let parser = OxcParser::new(&allocator, source, SourceType::default());

    let expr = parser
        .parse_expression()
        .map_err(|_| Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32)))?;

    let info = extract_expression_info(&expr, offset);
    // allocator drops here — all OXC data freed
    Ok(info)
}

fn extract_script_info(program: &oxc_ast::ast::Program<'_>, offset: u32, source: &str) -> ScriptInfo {
    let mut declarations = Vec::new();
    let mut props_declaration = None;
    let mut exports = Vec::new();
    let mut has_effects = false;

    for stmt in &program.body {
        use oxc_ast::ast::Statement;

        match stmt {
            Statement::ExportNamedDeclaration(export) => {
                // `export { x, y as z }` form
                for spec in &export.specifiers {
                    let local = compact(&spec.local.name());
                    let exported = compact(&spec.exported.name());
                    let alias = if local != exported { Some(exported) } else { None };
                    exports.push(ExportInfo { name: local, alias });
                }
                // `export const/function/class ...` form
                if let Some(decl) = &export.declaration {
                    collect_export_names_from_declaration(decl, &mut exports);
                    collect_declarations_from_declaration(decl, offset, source, &mut declarations, &mut props_declaration);
                }
            }
            Statement::VariableDeclaration(decl) => {
                collect_var_declarations(decl, offset, source, &mut declarations, &mut props_declaration);
            }
            Statement::FunctionDeclaration(func) => {
                collect_func_declaration(func, offset, &mut declarations);
            }
            Statement::ExpressionStatement(es) => {
                // $effect(fn) and $effect.pre(fn) need context (push/pop).
                // $effect.tracking() does NOT — it's a pure read.
                if is_effect_call(&es.expression) {
                    has_effects = true;
                }
            }
            _ => {}
        }
    }

    ScriptInfo { declarations, props_declaration, exports, has_effects, store_candidates: Vec::new() }
}

/// Enrich ScriptInfo from OXC's unresolved references in one pass.
/// Detects store candidates ($count etc) from unresolved `$`-prefixed references.
fn enrich_script_info_from_unresolved(scoping: &oxc_semantic::Scoping, info: &mut ScriptInfo) {
    for key in scoping.root_unresolved_references().keys() {
        let name = key.as_str();
        if name.starts_with('$') && name.len() > 1 && !name.starts_with("$$") && !is_rune_name(name) {
            info.store_candidates.push(compact(&name[1..]));
        }
    }
}

fn collect_export_names_from_declaration(
    decl: &oxc_ast::ast::Declaration<'_>,
    exports: &mut Vec<ExportInfo>,
) {
    match decl {
        oxc_ast::ast::Declaration::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                if let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id {
                    exports.push(ExportInfo { name: compact(&ident.name), alias: None });
                }
            }
        }
        oxc_ast::ast::Declaration::FunctionDeclaration(func) => {
            if let Some(ident) = &func.id {
                exports.push(ExportInfo { name: compact(&ident.name), alias: None });
            }
        }
        oxc_ast::ast::Declaration::ClassDeclaration(cls) => {
            if let Some(ident) = &cls.id {
                exports.push(ExportInfo { name: compact(&ident.name), alias: None });
            }
        }
        _ => {}
    }
}

fn collect_declarations_from_declaration(
    decl: &oxc_ast::ast::Declaration<'_>,
    offset: u32,
    source: &str,
    declarations: &mut Vec<DeclarationInfo>,
    props_declaration: &mut Option<PropsDeclaration>,
) {
    match decl {
        oxc_ast::ast::Declaration::VariableDeclaration(var_decl) => {
            collect_var_declarations(var_decl, offset, source, declarations, props_declaration);
        }
        oxc_ast::ast::Declaration::FunctionDeclaration(func) => {
            collect_func_declaration(func, offset, declarations);
        }
        _ => {}
    }
}

fn collect_func_declaration(
    func: &oxc_ast::ast::Function<'_>,
    offset: u32,
    declarations: &mut Vec<DeclarationInfo>,
) {
    if let Some(ident) = &func.id {
        declarations.push(DeclarationInfo {
            name: compact(&ident.name),
            span: Span::new(ident.span.start + offset, ident.span.end + offset),
            kind: DeclarationKind::Function,
            init_span: None,
            is_rune: None,
            rune_init_refs: vec![],
        });
    }
}

fn collect_var_declarations(
    decl: &oxc_ast::ast::VariableDeclaration<'_>,
    offset: u32,
    source: &str,
    declarations: &mut Vec<DeclarationInfo>,
    props_declaration: &mut Option<PropsDeclaration>,
) {
    let kind = match decl.kind {
        oxc_ast::ast::VariableDeclarationKind::Let => DeclarationKind::Let,
        oxc_ast::ast::VariableDeclarationKind::Const => DeclarationKind::Const,
        oxc_ast::ast::VariableDeclarationKind::Var => DeclarationKind::Var,
        _ => DeclarationKind::Var,
    };

    for declarator in &decl.declarations {
        match &declarator.id {
            oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => {
                let name = compact(&ident.name);
                let decl_span = Span::new(
                    ident.span.start + offset,
                    ident.span.end + offset,
                );

                let (init_span, is_rune, rune_init_refs) = if let Some(init) = &declarator.init {
                    let init_sp = Span::new(
                        init.span().start + offset,
                        init.span().end + offset,
                    );
                    let rune = detect_rune(init);
                    let refs = if matches!(rune, Some(RuneKind::Derived | RuneKind::DerivedBy)) {
                        collect_derived_refs(init)
                    } else {
                        vec![]
                    };
                    (Some(init_sp), rune, refs)
                } else {
                    (None, None, vec![])
                };

                declarations.push(DeclarationInfo {
                    name,
                    span: decl_span,
                    kind,
                    init_span,
                    is_rune,
                    rune_init_refs,
                });
            }
            oxc_ast::ast::BindingPattern::ObjectPattern(obj_pat) => {
                let is_props = declarator.init.as_ref()
                    .and_then(|init| detect_rune(init))
                    .map(|r| r == RuneKind::Props)
                    .unwrap_or(false);

                if is_props {
                    let mut props = Vec::new();

                    for prop in &obj_pat.properties {
                        let key_name = extract_property_key_name(&prop.key);
                        let Some(key_name) = key_name else { continue };

                        let local_name = extract_binding_name(&prop.value);
                        let local_name = local_name.unwrap_or_else(|| key_name.clone());

                        let (default_span, default_text, is_bindable) = extract_prop_default(&prop.value, offset, source);

                        let decl_span = Span::new(
                            prop.span.start + offset,
                            prop.span.end + offset,
                        );

                        declarations.push(DeclarationInfo {
                            name: local_name.clone(),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::Props),
                            rune_init_refs: vec![],
                        });

                        props.push(PropInfo {
                            local_name,
                            prop_name: key_name,
                            default_span,
                            default_text,
                            is_bindable,
                            is_rest: false,
                        });
                    }

                    if let Some(rest) = &obj_pat.rest {
                        if let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &rest.argument {
                            let rest_name = compact(&ident.name);
                            let decl_span = Span::new(
                                ident.span.start + offset,
                                ident.span.end + offset,
                            );
                            declarations.push(DeclarationInfo {
                                name: rest_name.clone(),
                                span: decl_span,
                                kind,
                                init_span: None,
                                is_rune: Some(RuneKind::Props),
                                rune_init_refs: vec![],
                            });
                            props.push(PropInfo {
                                local_name: rest_name.clone(),
                                prop_name: rest_name,
                                default_span: None,
                                default_text: None,
                                is_bindable: false,
                                is_rest: true,
                            });
                        }
                    }

                    *props_declaration = Some(PropsDeclaration { props });
                }
            }
            _ => {}
        }
    }
}

/// Parse a `<script>` block once and return both owned analysis info and OXC Scoping.
///
/// This avoids double-parsing: the single OXC parse produces both the `ScriptInfo`
/// (declarations, props) and the `Scoping` (symbol table + scope tree for semantic analysis).
pub fn analyze_script_with_scoping(
    source: &str,
    offset: u32,
    typescript: bool,
) -> Result<(ScriptInfo, oxc_semantic::Scoping), Vec<Diagnostic>> {
    let allocator = Allocator::default();
    let source_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };

    let parser = OxcParser::new(&allocator, source, source_type);
    let result = parser.parse();

    if !result.errors.is_empty() {
        return Err(result
            .errors
            .iter()
            .map(|_| {
                Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
            })
            .collect());
    }

    let program = &result.program;

    // Extract ScriptInfo by walking the AST
    let mut script_info = extract_script_info(program, offset, source);

    // Build semantic analysis and extract Scoping
    let sem = oxc_semantic::SemanticBuilder::new().build(program);

    enrich_script_info_from_unresolved(&sem.semantic.scoping(), &mut script_info);

    let scoping = sem.semantic.into_scoping();

    Ok((script_info, scoping))
}

/// Parse a `<script>` block once in a caller-provided allocator and return
/// analysis info, scoping, and the live `Program` AST.
///
/// The returned `Program<'a>` is reused by codegen, eliminating double-parsing.
pub fn analyze_script_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<(ScriptInfo, oxc_semantic::Scoping, oxc_ast::ast::Program<'a>), Vec<Diagnostic>> {
    let source_type = if typescript {
        SourceType::mjs().with_typescript(true)
    } else {
        SourceType::mjs()
    };

    let result = OxcParser::new(alloc, source, source_type).parse();

    if !result.errors.is_empty() {
        return Err(result
            .errors
            .iter()
            .map(|_| {
                Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
            })
            .collect());
    }

    let program = result.program;
    let mut script_info = extract_script_info(&program, offset, source);
    let sem = oxc_semantic::SemanticBuilder::new().build(&program);

    // Extract has_effects + store_candidates from unresolved references in one pass.
    // $effect → has_effects; $count (non-rune) → store candidate.
    enrich_script_info_from_unresolved(&sem.semantic.scoping(), &mut script_info);

    let scoping = sem.semantic.into_scoping();

    Ok((script_info, scoping, program))
}

/// Check if an expression text represents a "simple" expression that can be
/// eagerly evaluated (no side effects). Matches Svelte's `is_simple_expression()`.
///
/// Simple expressions: literals, identifiers, functions, and combinations of
/// binary/logical/conditional expressions composed of simples.
pub fn is_simple_expression(text: &str) -> bool {
    let alloc = Allocator::default();
    let Ok(expr) = OxcParser::new(&alloc, text, SourceType::default()).parse_expression() else {
        return false;
    };
    is_simple_expr(&expr)
}

fn is_simple_expr(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::Identifier(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_) => true,
        Expression::ConditionalExpression(c) => {
            is_simple_expr(&c.test) && is_simple_expr(&c.consequent) && is_simple_expr(&c.alternate)
        }
        Expression::BinaryExpression(b) => {
            is_simple_expr(&b.left) && is_simple_expr(&b.right)
        }
        Expression::LogicalExpression(l) => {
            is_simple_expr(&l.left) && is_simple_expr(&l.right)
        }
        _ => false,
    }
}

/// Events that Svelte delegates to the document root.
pub fn is_delegatable_event(name: &str) -> bool {
    matches!(
        name,
        "click"
            | "input"
            | "change"
            | "submit"
            | "focus"
            | "blur"
            | "keydown"
            | "keyup"
            | "keypress"
            | "mousedown"
            | "mouseup"
            | "mousemove"
            | "mouseenter"
            | "mouseleave"
            | "mouseover"
            | "mouseout"
            | "touchstart"
            | "touchend"
            | "touchmove"
            | "pointerdown"
            | "pointerup"
            | "pointermove"
            | "focusin"
            | "focusout"
            | "dblclick"
            | "contextmenu"
            | "auxclick"
    )
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

fn extract_expression_info(expr: &Expression<'_>, offset: u32) -> ExpressionInfo {
    let kind = match expr {
        Expression::Identifier(ident) => ExpressionKind::Identifier(compact(&ident.name)),
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_) => ExpressionKind::Literal,
        Expression::CallExpression(call) => {
            let callee = match &call.callee {
                Expression::Identifier(ident) => compact(&ident.name),
                _ => CompactString::default(),
            };
            ExpressionKind::CallExpression { callee }
        }
        Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_) => {
            ExpressionKind::MemberExpression
        }
        Expression::ArrowFunctionExpression(_) => ExpressionKind::ArrowFunction,
        Expression::AssignmentExpression(_) => ExpressionKind::Assignment,
        _ => ExpressionKind::Other,
    };

    let mut references = Vec::new();
    collect_references(expr, offset, &mut references);

    let has_side_effects = matches!(
        expr,
        Expression::CallExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::UpdateExpression(_)
    );

    ExpressionInfo {
        kind,
        references,
        has_side_effects,
    }
}

fn collect_references(expr: &Expression<'_>, offset: u32, refs: &mut Vec<Reference>) {
    match expr {
        Expression::Identifier(ident) => {
            refs.push(Reference {
                name: compact(&ident.name),
                span: Span::new(
                    ident.span.start + offset,
                    ident.span.end + offset,
                ),
                flags: ReferenceFlags::Read,
                symbol_id: None,
            });
        }
        Expression::AssignmentExpression(assign) => {
            // LHS is write, RHS is read
            if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(ident) = &assign.left {
                refs.push(Reference {
                    name: compact(&ident.name),
                    span: Span::new(
                        ident.span.start + offset,
                        ident.span.end + offset,
                    ),
                    flags: ReferenceFlags::Write,
                    symbol_id: None,
                });
            }
            collect_references(&assign.right, offset, refs);
        }
        Expression::BinaryExpression(bin) => {
            collect_references(&bin.left, offset, refs);
            collect_references(&bin.right, offset, refs);
        }
        Expression::LogicalExpression(log) => {
            collect_references(&log.left, offset, refs);
            collect_references(&log.right, offset, refs);
        }
        Expression::UnaryExpression(un) => {
            collect_references(&un.argument, offset, refs);
        }
        Expression::UpdateExpression(upd) => {
            if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) =
                &upd.argument
            {
                refs.push(Reference {
                    name: compact(&ident.name),
                    span: Span::new(ident.span.start + offset, ident.span.end + offset),
                    flags: ReferenceFlags::Write,
                    symbol_id: None,
                });
            }
        }
        Expression::CallExpression(call) => {
            collect_references(&call.callee, offset, refs);
            for arg in &call.arguments {
                if let oxc_ast::ast::Argument::SpreadElement(spread) = arg {
                    collect_references(&spread.argument, offset, refs);
                } else if let Some(expr) = arg.as_expression() {
                    collect_references(expr, offset, refs);
                }
            }
        }
        Expression::ConditionalExpression(cond) => {
            collect_references(&cond.test, offset, refs);
            collect_references(&cond.consequent, offset, refs);
            collect_references(&cond.alternate, offset, refs);
        }
        Expression::StaticMemberExpression(mem) => {
            collect_references(&mem.object, offset, refs);
        }
        Expression::ComputedMemberExpression(mem) => {
            collect_references(&mem.object, offset, refs);
            collect_references(&mem.expression, offset, refs);
        }
        Expression::TemplateLiteral(tl) => {
            for expr in &tl.expressions {
                collect_references(expr, offset, refs);
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            collect_references(&paren.expression, offset, refs);
        }
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                match elem {
                    oxc_ast::ast::ArrayExpressionElement::SpreadElement(spread) => {
                        collect_references(&spread.argument, offset, refs);
                    }
                    _ => {
                        if let Some(expr) = elem.as_expression() {
                            collect_references(expr, offset, refs);
                        }
                    }
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    oxc_ast::ast::ObjectPropertyKind::ObjectProperty(p) => {
                        collect_references(&p.value, offset, refs);
                    }
                    oxc_ast::ast::ObjectPropertyKind::SpreadProperty(spread) => {
                        collect_references(&spread.argument, offset, refs);
                    }
                }
            }
        }
        Expression::ArrowFunctionExpression(arrow) => {
            for stmt in &arrow.body.statements {
                collect_statement_references(stmt, offset, refs);
            }
        }
        Expression::SequenceExpression(seq) => {
            for expr in &seq.expressions {
                collect_references(expr, offset, refs);
            }
        }
        _ => {}
    }
}

fn collect_statement_references(stmt: &oxc_ast::ast::Statement<'_>, offset: u32, refs: &mut Vec<Reference>) {
    use oxc_ast::ast::Statement;
    match stmt {
        Statement::ExpressionStatement(es) => collect_references(&es.expression, offset, refs),
        Statement::ReturnStatement(ret) => {
            if let Some(arg) = &ret.argument {
                collect_references(arg, offset, refs);
            }
        }
        Statement::BlockStatement(block) => {
            for s in &block.body {
                collect_statement_references(s, offset, refs);
            }
        }
        Statement::IfStatement(if_stmt) => {
            collect_references(&if_stmt.test, offset, refs);
            collect_statement_references(&if_stmt.consequent, offset, refs);
            if let Some(alt) = &if_stmt.alternate {
                collect_statement_references(alt, offset, refs);
            }
        }
        Statement::VariableDeclaration(decl) => {
            for d in &decl.declarations {
                if let Some(init) = &d.init {
                    collect_references(init, offset, refs);
                }
            }
        }
        _ => {}
    }
}

fn extract_property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<CompactString> {
    match key {
        oxc_ast::ast::PropertyKey::StaticIdentifier(ident) => Some(compact(&ident.name)),
        oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(compact(&s.value)),
        _ => None,
    }
}

fn extract_binding_name(pattern: &oxc_ast::ast::BindingPattern<'_>) -> Option<CompactString> {
    match pattern {
        oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => Some(compact(&ident.name)),
        oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
            extract_binding_name(&assign.left)
        }
        _ => None,
    }
}

/// Extract default span, default text, and bindable flag from a prop's binding pattern.
fn extract_prop_default(pattern: &oxc_ast::ast::BindingPattern<'_>, offset: u32, source: &str) -> (Option<Span>, Option<String>, bool) {
    if let oxc_ast::ast::BindingPattern::AssignmentPattern(assign) = pattern {
        let right = &assign.right;
        // Check if default is $bindable(expr) or $bindable()
        if let Expression::CallExpression(call) = right {
            if let Expression::Identifier(ident) = &call.callee {
                if ident.name.as_str() == "$bindable" {
                    let (default_span, default_text) = if let Some(arg) = call.arguments.first() {
                        let sp = arg.span();
                        let text = &source[sp.start as usize..sp.end as usize];
                        (Some(Span::new(sp.start + offset, sp.end + offset)), Some(text.to_string()))
                    } else {
                        (None, None)
                    };
                    return (default_span, default_text, true);
                }
            }
        }
        let sp = right.span();
        let text = &source[sp.start as usize..sp.end as usize];
        (Some(Span::new(sp.start + offset, sp.end + offset)), Some(text.to_string()), false)
    } else {
        (None, None, false)
    }
}

/// Returns true for `$effect(fn)` and `$effect.pre(fn)` calls — these need
/// `$.push`/`$.pop` context wrapping. Does NOT match `$effect.tracking()`.
fn is_effect_call(expr: &Expression<'_>) -> bool {
    if let Expression::CallExpression(call) = expr {
        match &call.callee {
            Expression::Identifier(id) if id.name.as_str() == "$effect" => return true,
            Expression::StaticMemberExpression(member) => {
                if let Expression::Identifier(obj) = &member.object {
                    if obj.name.as_str() == "$effect" && member.property.name.as_str() == "pre" {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn detect_rune(expr: &Expression<'_>) -> Option<RuneKind> {
    if let Expression::CallExpression(call) = expr {
        match &call.callee {
            Expression::Identifier(ident) => {
                return match ident.name.as_str() {
                    "$state" => Some(RuneKind::State),
                    "$derived" => Some(RuneKind::Derived),
                    "$effect" => Some(RuneKind::Effect),
                    "$props" => Some(RuneKind::Props),
                    "$bindable" => Some(RuneKind::Bindable),
                    "$inspect" => Some(RuneKind::Inspect),
                    "$host" => Some(RuneKind::Host),
                    _ => None,
                };
            }
            Expression::StaticMemberExpression(member) => {
                if let Expression::Identifier(obj) = &member.object {
                    let prop = member.property.name.as_str();
                    return match (obj.name.as_str(), prop) {
                        ("$derived", "by") => Some(RuneKind::DerivedBy),
                        ("$state", "raw") => Some(RuneKind::StateRaw),
                        ("$effect", "tracking") => Some(RuneKind::EffectTracking),
                        _ => None,
                    };
                }
            }
            _ => {}
        }
    }
    None
}

/// Collect identifier references from a $derived/$derived.by call's argument.
/// Returns deduplicated list — avoids redundant `is_dynamic_by_id` lookups.
fn collect_derived_refs(expr: &Expression<'_>) -> Vec<CompactString> {
    let Expression::CallExpression(call) = expr else {
        return vec![];
    };
    if call.arguments.is_empty() {
        return vec![];
    }
    let Some(arg_expr) = call.arguments[0].as_expression() else {
        return vec![];
    };
    let mut refs = Vec::new();
    collect_idents_recursive(arg_expr, &mut refs);
    let mut seen = std::collections::HashSet::new();
    refs.retain(|r| seen.insert(r.clone()));
    refs
}

/// Check if a `$`-prefixed name is a known rune (not a store candidate).
fn is_rune_name(name: &str) -> bool {
    matches!(name, "$state" | "$derived" | "$effect" | "$props" | "$bindable" | "$inspect" | "$host")
}

fn collect_idents_recursive(expr: &Expression<'_>, refs: &mut Vec<CompactString>) {
    use oxc_ast::ast::Expression::*;
    match expr {
        Identifier(id) => {
            let name = id.name.as_str();
            if !name.starts_with('$') {
                refs.push(compact(name));
            }
        }
        BinaryExpression(bin) => {
            collect_idents_recursive(&bin.left, refs);
            collect_idents_recursive(&bin.right, refs);
        }
        CallExpression(call) => {
            collect_idents_recursive(&call.callee, refs);
            for arg in &call.arguments {
                if let Some(e) = arg.as_expression() {
                    collect_idents_recursive(e, refs);
                }
            }
        }
        ArrowFunctionExpression(arrow) => {
            // Collect refs from arrow body — skip params
            for stmt in &arrow.body.statements {
                match stmt {
                    oxc_ast::ast::Statement::ExpressionStatement(es) => {
                        collect_idents_recursive(&es.expression, refs);
                    }
                    oxc_ast::ast::Statement::ReturnStatement(ret) => {
                        if let Some(arg) = &ret.argument {
                            collect_idents_recursive(arg, refs);
                        }
                    }
                    _ => {}
                }
            }
        }
        UnaryExpression(unary) => {
            collect_idents_recursive(&unary.argument, refs);
        }
        ConditionalExpression(cond) => {
            collect_idents_recursive(&cond.test, refs);
            collect_idents_recursive(&cond.consequent, refs);
            collect_idents_recursive(&cond.alternate, refs);
        }
        LogicalExpression(log) => {
            collect_idents_recursive(&log.left, refs);
            collect_idents_recursive(&log.right, refs);
        }
        StaticMemberExpression(m) => {
            collect_idents_recursive(&m.object, refs);
        }
        ComputedMemberExpression(m) => {
            collect_idents_recursive(&m.object, refs);
            collect_idents_recursive(&m.expression, refs);
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_simple_identifier() {
        let info = analyze_expression("count", 0).unwrap();
        assert_eq!(info.kind, ExpressionKind::Identifier(compact("count")));
        assert_eq!(info.references.len(), 1);
        assert_eq!(info.references[0].name, "count");
    }

    #[test]
    fn analyze_binary_expression() {
        let info = analyze_expression("count + 1", 0).unwrap();
        assert_eq!(info.references.len(), 1);
        assert_eq!(info.references[0].name, "count");
    }

    #[test]
    fn analyze_call_expression() {
        let info = analyze_expression("foo(a, b)", 0).unwrap();
        assert!(matches!(info.kind, ExpressionKind::CallExpression { .. }));
        assert_eq!(info.references.len(), 3); // foo, a, b
        assert!(info.has_side_effects);
    }

    #[test]
    fn analyze_assignment() {
        let info = analyze_expression("count = 10", 0).unwrap();
        assert_eq!(info.kind, ExpressionKind::Assignment);
        assert!(info.references.iter().any(|r| r.name == "count" && matches!(r.flags, ReferenceFlags::Write)));
    }

    #[test]
    fn analyze_script_basic() {
        let (info, _scoping) = analyze_script_with_scoping("let count = $state(0); const name = 'test';", 0, false).unwrap();
        assert_eq!(info.declarations.len(), 2);
        assert_eq!(info.declarations[0].name, "count");
        assert_eq!(info.declarations[0].is_rune, Some(RuneKind::State));
        assert_eq!(info.declarations[1].name, "name");
        assert_eq!(info.declarations[1].is_rune, None);
    }

    #[test]
    fn analyze_with_offset() {
        let info = analyze_expression("x", 100).unwrap();
        assert_eq!(info.references[0].span.start, 100);
        assert_eq!(info.references[0].span.end, 101);
    }

    #[test]
    fn parse_const_declaration_simple() {
        let alloc = Allocator::default();
        let source = alloc.alloc_str("doubled = item * 2");
        let (names, refs, _expr) = parse_const_declaration_with_alloc(&alloc, source, 10).unwrap();
        assert_eq!(names, vec![compact("doubled")]);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].name, "item");
        // "const doubled = item * 2" — "item" starts at byte 16 in wrapped string
        // offset adjustment: 10 - 6 = 4, so span = 16 + 4 = 20
        assert_eq!(refs[0].span.start, 20);
    }

    #[test]
    fn parse_const_declaration_destructuring() {
        let alloc = Allocator::default();
        // offset >= 6 required (compensates "const " prefix in wrapping arithmetic)
        let source = alloc.alloc_str("{a, b} = obj");
        let (names, refs, _expr) = parse_const_declaration_with_alloc(&alloc, source, 10).unwrap();
        assert_eq!(names, vec![compact("a"), compact("b")]);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].name, "obj");
    }

    #[test]
    fn parse_const_declaration_multiple_equals() {
        let alloc = Allocator::default();
        let source = alloc.alloc_str("a = b === c");
        let (names, refs, _expr) = parse_const_declaration_with_alloc(&alloc, source, 10).unwrap();
        assert_eq!(names, vec![compact("a")]);
        assert_eq!(refs.len(), 2);
        assert!(refs.iter().any(|r| r.name == "b"));
        assert!(refs.iter().any(|r| r.name == "c"));
    }

    #[test]
    fn analyze_script_exports() {
        let (info, _) = analyze_script_with_scoping(
            "export const PI = 3.14; export function greet(name) { return name; }",
            0, false
        ).unwrap();
        assert_eq!(info.exports.len(), 2);
        assert_eq!(info.exports[0].name, "PI");
        assert_eq!(info.exports[1].name, "greet");
        // Declarations are also extracted from exported statements
        assert!(info.declarations.iter().any(|d| d.name == "PI"));
        assert!(info.declarations.iter().any(|d| d.name == "greet"));
    }
}

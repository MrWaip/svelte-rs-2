//! Shared types and OXC utilities for the Svelte compiler.
//!
//! Leaf crate providing domain types (`ExpressionInfo`, `RuneKind`, `ScriptInfo`, etc.)
//! and OXC parsing helpers used across parser, analyze, transform, and codegen.

use oxc_ast::ast::Expression;
use oxc_span::GetSpan as _;

use compact_str::CompactString;
use oxc_semantic::SymbolId;
use rustc_hash::{FxHashMap, FxHashSet};
use svelte_ast::NodeId;
use svelte_span::Span;

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
    pub has_call: bool,
    /// Set when the expression contains `$effect.pending()` — forces the expression to be dynamic.
    pub has_state_rune: bool,
    /// Set when the expression contains a deep mutation on a `$`-prefixed identifier
    /// (e.g., `$store.field = val` or `$store.count++`). Used to determine if component
    /// needs `$.push/$.pop` for `$.store_mutate` support.
    pub has_store_member_mutation: bool,
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

impl ExpressionKind {
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::Identifier(_) | Self::MemberExpression)
    }
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
    /// True when the default expression is simple (literal, identifier, arrow).
    /// Pre-computed to avoid re-parsing in analyze.
    pub is_simple_default: bool,
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
    /// True when the script contains class fields with `$state()`/`$state.raw()` initializers.
    pub has_class_state_fields: bool,
    /// Base names of `$`-prefixed identifiers found in the script body
    /// (e.g. `"count"` for `$count`). Used to detect store subscriptions.
    pub store_candidates: Vec<CompactString>,
    /// True when script contains deep mutations on `$`-prefixed identifiers
    /// (e.g., `$store.field = val`). Triggers `$.push/$.pop` for `$.store_mutate`.
    pub has_store_member_mutations: bool,
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
    StateEager,
    EffectPending,
    Inspect,
    Host,
    PropsId,
}

impl RuneKind {
    pub fn is_derived(&self) -> bool {
        matches!(self, RuneKind::Derived | RuneKind::DerivedBy)
    }
}

// ---------------------------------------------------------------------------
// ParsedExprs — parsed JS expression ASTs in a shared OXC allocator
// ---------------------------------------------------------------------------

/// Parsed JS expression ASTs, stored in a shared OXC allocator.
/// Separate from AnalysisData to avoid lifetime propagation.
pub struct ParsedExprs<'a> {
    /// Template expressions: ExpressionTag, IfBlock test, EachBlock expr, RenderTag, HtmlTag.
    pub exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Attribute expressions, keyed by attribute NodeId.
    pub attr_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// ConcatenationAttribute dynamic parts: (attr_id, part_index).
    pub concat_part_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,
    /// EachBlock key expressions: keyed by EachBlock NodeId.
    pub key_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Pre-parsed script Program AST. Consumed by codegen via `Option::take()`.
    pub script_program: Option<oxc_ast::ast::Program<'a>>,
    /// DebugTag identifier expressions: (debug_tag_id, identifier_index) → transformed expression.
    pub debug_tag_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,
    /// Pre-parsed custom element `extend` expression. Consumed by codegen via `Option::take()`.
    pub ce_extend_expr: Option<Expression<'a>>,
    /// Pre-parsed prop default expressions, indexed by prop position in PropsDeclaration.
    /// Consumed by codegen via clone/take.
    pub prop_default_exprs: Vec<Option<Expression<'a>>>,
    /// Pre-parsed each-block destructuring context bindings, keyed by EachBlock NodeId.
    /// Consumed by codegen via `remove()`.
    pub each_context_bindings: FxHashMap<NodeId, EachContextBinding<'a>>,
    /// Pre-parsed directive name expressions (use:, transition:, animate:).
    /// Keyed by directive NodeId. Consumed by codegen via `remove()`.
    pub directive_name_exprs: FxHashMap<NodeId, Expression<'a>>,
}

impl<'a> ParsedExprs<'a> {
    pub fn new() -> Self {
        Self {
            exprs: FxHashMap::default(),
            attr_exprs: FxHashMap::default(),
            concat_part_exprs: FxHashMap::default(),
            key_exprs: FxHashMap::default(),
            script_program: None,
            debug_tag_exprs: FxHashMap::default(),
            ce_extend_expr: None,
            prop_default_exprs: Vec::new(),
            each_context_bindings: FxHashMap::default(),
            directive_name_exprs: FxHashMap::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// JsParseResult — intermediate JS parsing results passed from parser to analyze
// ---------------------------------------------------------------------------

/// All data produced by JS expression parsing.
/// Created by `svelte_parser::parse_with_js()`, consumed by `svelte_analyze::analyze()`.
pub struct JsParseResult<'a> {
    pub parsed: ParsedExprs<'a>,
    /// Expression metadata (ExpressionInfo) for template expressions.
    pub expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Expression metadata for attribute expressions.
    pub attr_expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Parsed script info.
    pub script: Option<ScriptInfo>,
    /// Exported names from script.
    pub exports: Vec<ExportInfo>,
    /// Component needs runtime context (has $effect calls or class state fields).
    pub needs_context: bool,
    /// Script contains class declarations with $state/$state.raw fields.
    pub has_class_state_fields: bool,
    /// Raw OXC scoping from script parsing, before ComponentScoping is built.
    pub scoping: Option<oxc_semantic::Scoping>,
    /// Each blocks where key expression uses the index variable.
    pub each_key_uses_index: FxHashSet<NodeId>,
    /// Each blocks where body expressions use the index variable.
    pub each_body_uses_index: FxHashSet<NodeId>,
    /// Const tag declared binding names.
    pub const_tag_names: FxHashMap<NodeId, Vec<String>>,
    /// Await block then-binding patterns.
    pub await_values: FxHashMap<NodeId, AwaitBindingInfo>,
    /// Await block catch-binding patterns.
    pub await_errors: FxHashMap<NodeId, AwaitBindingInfo>,
    /// Render tags with ChainExpression callee.
    pub render_tag_is_chain: FxHashSet<NodeId>,
    /// Callee identifier name for render tags.
    pub render_tag_callee_name: FxHashMap<NodeId, String>,
    /// Per-argument has_call flags for render tag expressions.
    pub render_tag_arg_has_call: FxHashMap<NodeId, Vec<bool>>,
    /// Per-argument identifier name (if the arg is a plain identifier).
    pub render_tag_arg_idents: FxHashMap<NodeId, Vec<Option<String>>>,
    /// Attribute/directive whose expression is a simple identifier matching the name.
    pub expression_shorthand: FxHashSet<NodeId>,
    /// class={expr} attributes that need clsx resolution.
    pub needs_clsx: FxHashSet<NodeId>,
    /// Parsed custom element config.
    pub ce_config: Option<ParsedCeConfig>,
}

impl<'a> JsParseResult<'a> {
    pub fn new() -> Self {
        Self {
            parsed: ParsedExprs::new(),
            expressions: FxHashMap::default(),
            attr_expressions: FxHashMap::default(),
            script: None,
            exports: Vec::new(),
            needs_context: false,
            has_class_state_fields: false,
            scoping: None,
            each_key_uses_index: FxHashSet::default(),
            each_body_uses_index: FxHashSet::default(),
            const_tag_names: FxHashMap::default(),
            await_values: FxHashMap::default(),
            await_errors: FxHashMap::default(),
            render_tag_is_chain: FxHashSet::default(),
            render_tag_callee_name: FxHashMap::default(),
            render_tag_arg_has_call: FxHashMap::default(),
            render_tag_arg_idents: FxHashMap::default(),
            expression_shorthand: FxHashSet::default(),
            needs_clsx: FxHashSet::default(),
            ce_config: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Each context binding info
// ---------------------------------------------------------------------------

/// Parsed destructuring context for `{#each items as { name, value }}` or `[a, b]`.
pub struct EachContextBinding<'a> {
    pub is_array: bool,
    pub bindings: Vec<EachBindingEntry<'a>>,
}

/// Single entry in a parsed destructuring context pattern.
pub struct EachBindingEntry<'a> {
    /// Binding name (alias if renamed, e.g. `alias` for `{ prop: alias }`).
    pub name: CompactString,
    /// Property key for object patterns (None = shorthand, i.e. `name == key_name`).
    pub key_name: Option<CompactString>,
    /// Pre-parsed default expression (e.g. `'N/A'` for `{ value = 'N/A' }`).
    pub default_expr: Option<Expression<'a>>,
}

// ---------------------------------------------------------------------------
// Await binding info
// ---------------------------------------------------------------------------

/// Destructuring kind for await block bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestructureKind {
    Array,
    Object,
}

/// Parsed binding pattern for `{:then value}` / `{:catch error}`.
#[derive(Debug, Clone, PartialEq)]
pub enum AwaitBindingInfo {
    /// Simple identifier: `{:then value}`
    Simple(String),
    /// Destructured: `{:then { name, age }}` or `{:then [a, b]}`
    Destructured {
        kind: DestructureKind,
        names: Vec<String>,
    },
}

impl AwaitBindingInfo {
    /// All binding names regardless of variant.
    pub fn names(&self) -> Vec<&str> {
        match self {
            Self::Simple(name) => vec![name.as_str()],
            Self::Destructured { names, .. } => names.iter().map(|s| s.as_str()).collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// Custom element config parsing
// ---------------------------------------------------------------------------

/// Shadow root mode for custom elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CeShadowMode {
    Open,
    None,
}

/// Single prop definition within a custom element config.
#[derive(Debug, Clone)]
pub struct CePropConfig {
    pub name: String,
    pub attribute: Option<String>,
    pub reflect: bool,
    pub prop_type: Option<String>,
}

/// Parsed custom element config from `<svelte:options customElement={{ ... }}>`.
#[derive(Debug, Clone)]
pub struct ParsedCeConfig {
    pub tag: Option<String>,
    pub shadow: CeShadowMode,
    /// Ordered list of prop definitions, preserving config order.
    pub props: Vec<CePropConfig>,
    /// Span of the `extend` expression value (absolute, within original source).
    pub extend_span: Option<Span>,
}

pub fn extract_all_binding_names(pattern: &oxc_ast::ast::BindingPattern<'_>, names: &mut Vec<CompactString>) {
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

pub fn extract_script_info(program: &oxc_ast::ast::Program<'_>, offset: u32, source: &str) -> ScriptInfo {
    let mut declarations = Vec::new();
    let mut props_declaration = None;
    let mut exports = Vec::new();
    let mut has_effects = false;
    let mut has_class_state_fields = false;

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
            Statement::ClassDeclaration(class) => {
                if has_class_state_runes(&class.body) {
                    has_class_state_fields = true;
                }
            }
            _ => {}
        }
    }

    ScriptInfo { declarations, props_declaration, exports, has_effects, has_class_state_fields, store_candidates: Vec::new(), has_store_member_mutations: false }
}

/// Enrich ScriptInfo from OXC's unresolved references in one pass.
/// Detects store candidates ($count etc) from unresolved `$`-prefixed references.
pub fn enrich_script_info_from_unresolved(scoping: &oxc_semantic::Scoping, info: &mut ScriptInfo) {
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
                let rune = declarator.init.as_ref().and_then(|init| detect_rune(init));

                if rune == Some(RuneKind::Props) {
                    let mut props = Vec::new();

                    for prop in &obj_pat.properties {
                        let key_name = extract_property_key_name(&prop.key);
                        let Some(key_name) = key_name else { continue };

                        let local_name = extract_binding_name(&prop.value);
                        let local_name = local_name.unwrap_or_else(|| key_name.clone());

                        let (default_span, default_text, is_bindable, is_simple_default) = extract_prop_default(&prop.value, offset, source);

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
                            is_simple_default,
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
                                is_simple_default: true,
                            });
                        }
                    }

                    *props_declaration = Some(PropsDeclaration { props });
                } else if matches!(rune, Some(RuneKind::State | RuneKind::StateRaw)) {
                    // Destructured $state/$state.raw: register each leaf binding.
                    // Use StateRaw for analysis so all bindings are considered dynamic
                    // (proxied values are always reactive, even if the binding isn't mutated).
                    let mut names = Vec::new();
                    extract_all_binding_names(&declarator.id, &mut names);
                    for name in names {
                        let decl_span = Span::new(
                            declarator.span.start + offset,
                            declarator.span.end + offset,
                        );
                        declarations.push(DeclarationInfo {
                            name,
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::StateRaw),
                            rune_init_refs: vec![],
                        });
                    }
                }
            }
            oxc_ast::ast::BindingPattern::ArrayPattern(_) => {
                // Destructured $state/$state.raw: register each leaf binding.
                // Use StateRaw so all bindings are considered dynamic in analysis.
                let rune = declarator.init.as_ref().and_then(|init| detect_rune(init));
                if let Some(rune_kind) = rune {
                    if matches!(rune_kind, RuneKind::State | RuneKind::StateRaw) {
                        let mut names = Vec::new();
                        extract_all_binding_names(&declarator.id, &mut names);
                        for name in names {
                            let decl_span = Span::new(
                                declarator.span.start + offset,
                                declarator.span.end + offset,
                            );
                            declarations.push(DeclarationInfo {
                                name,
                                span: decl_span,
                                kind,
                                init_span: None,
                                is_rune: Some(RuneKind::StateRaw),
                                rune_init_refs: vec![],
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn is_simple_expr(expr: &Expression<'_>) -> bool {
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

/// Check if a string is a simple JS identifier (no member access, no computed access).
pub fn is_simple_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$')
        && s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

/// Events that Svelte delegates to the document root.
pub fn is_delegatable_event(name: &str) -> bool {
    matches!(
        name,
        "beforeinput"
            | "click"
            | "change"
            | "dblclick"
            | "contextmenu"
            | "focusin"
            | "focusout"
            | "input"
            | "keydown"
            | "keyup"
            | "mousedown"
            | "mousemove"
            | "mouseout"
            | "mouseover"
            | "mouseup"
            | "pointerdown"
            | "pointermove"
            | "pointerout"
            | "pointerover"
            | "pointerup"
            | "touchend"
            | "touchmove"
            | "touchstart"
    )
}

pub fn is_capture_event(name: &str) -> bool {
    name.ends_with("capture")
        && name != "gotpointercapture"
        && name != "lostpointercapture"
}

pub fn strip_capture_event(name: &str) -> Option<&str> {
    if is_capture_event(name) {
        Some(&name[..name.len() - 7])
    } else {
        None
    }
}

pub fn is_passive_event(name: &str) -> bool {
    matches!(name, "touchstart" | "touchmove")
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

pub fn extract_expression_info(expr: &Expression<'_>, offset: u32) -> ExpressionInfo {
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

    let has_call = expression_has_call(expr);

    let has_state_rune = expression_has_rune(expr, RuneKind::EffectPending)
        || expression_has_rune(expr, RuneKind::StateEager);

    let has_store_member_mutation = has_deep_store_mutation(expr);

    ExpressionInfo {
        kind,
        references,
        has_side_effects,
        has_call,
        has_state_rune,
        has_store_member_mutation,
    }
}

pub fn expression_has_call(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::CallExpression(_) => true,
        Expression::ConditionalExpression(c) => {
            expression_has_call(&c.test)
                || expression_has_call(&c.consequent)
                || expression_has_call(&c.alternate)
        }
        Expression::BinaryExpression(b) => {
            expression_has_call(&b.left) || expression_has_call(&b.right)
        }
        Expression::LogicalExpression(l) => {
            expression_has_call(&l.left) || expression_has_call(&l.right)
        }
        Expression::StaticMemberExpression(m) => expression_has_call(&m.object),
        Expression::ComputedMemberExpression(m) => {
            expression_has_call(&m.object) || expression_has_call(&m.expression)
        }
        Expression::UnaryExpression(u) => expression_has_call(&u.argument),
        Expression::SequenceExpression(s) => s.expressions.iter().any(|e| expression_has_call(e)),
        // Function boundaries are opaque
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => false,
        _ => false,
    }
}

/// Check if the expression (or any sub-expression) contains a call to a specific rune.
fn expression_has_rune(expr: &Expression<'_>, target: RuneKind) -> bool {
    match expr {
        Expression::CallExpression(_) => detect_rune(expr) == Some(target),
        Expression::ConditionalExpression(c) => {
            expression_has_rune(&c.test, target)
                || expression_has_rune(&c.consequent, target)
                || expression_has_rune(&c.alternate, target)
        }
        Expression::BinaryExpression(b) => {
            expression_has_rune(&b.left, target) || expression_has_rune(&b.right, target)
        }
        Expression::LogicalExpression(l) => {
            expression_has_rune(&l.left, target) || expression_has_rune(&l.right, target)
        }
        Expression::SequenceExpression(s) => s.expressions.iter().any(|e| expression_has_rune(e, target)),
        _ => false,
    }
}

/// Check if expression contains a deep mutation on a $-prefixed identifier
/// (e.g., `$store.field = val` or `$store.count++`).
pub fn has_deep_store_mutation(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::AssignmentExpression(assign) => {
            let has_store_member_lhs = match &assign.left {
                oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) => {
                    member_root_is_store(&m.object)
                }
                oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) => {
                    member_root_is_store(&m.object)
                }
                _ => false,
            };
            has_store_member_lhs || has_deep_store_mutation(&assign.right)
        }
        Expression::UpdateExpression(upd) => {
            match &upd.argument {
                oxc_ast::ast::SimpleAssignmentTarget::StaticMemberExpression(m) => {
                    member_root_is_store(&m.object)
                }
                oxc_ast::ast::SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                    member_root_is_store(&m.object)
                }
                _ => false,
            }
        }
        Expression::ArrowFunctionExpression(arrow) => {
            arrow.body.statements.iter().any(|stmt| {
                if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
                    has_deep_store_mutation(&es.expression)
                } else {
                    false
                }
            })
        }
        Expression::SequenceExpression(seq) => {
            seq.expressions.iter().any(|e| has_deep_store_mutation(e))
        }
        Expression::ConditionalExpression(c) => {
            has_deep_store_mutation(&c.test)
                || has_deep_store_mutation(&c.consequent)
                || has_deep_store_mutation(&c.alternate)
        }
        _ => false,
    }
}

/// Check if the root of a member expression chain is a $-prefixed identifier.
fn member_root_is_store(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Identifier(id) => id.name.starts_with('$') && id.name.len() > 1,
        Expression::StaticMemberExpression(m) => member_root_is_store(&m.object),
        Expression::ComputedMemberExpression(m) => member_root_is_store(&m.object),
        _ => false,
    }
}

pub fn collect_references(expr: &Expression<'_>, offset: u32, refs: &mut Vec<Reference>) {
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
            // LHS: collect write reference from identifier or read reference from member chain root
            match &assign.left {
                oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(ident) => {
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
                oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) => {
                    collect_references(&m.object, offset, refs);
                }
                oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) => {
                    collect_references(&m.object, offset, refs);
                    collect_references(&m.expression, offset, refs);
                }
                _ => {}
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
            match &upd.argument {
                oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) => {
                    refs.push(Reference {
                        name: compact(&ident.name),
                        span: Span::new(ident.span.start + offset, ident.span.end + offset),
                        flags: ReferenceFlags::Write,
                        symbol_id: None,
                    });
                }
                // Walk member chain to collect root identifier (e.g., $store in $store.count++)
                oxc_ast::ast::SimpleAssignmentTarget::StaticMemberExpression(m) => {
                    collect_references(&m.object, offset, refs);
                }
                oxc_ast::ast::SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                    collect_references(&m.object, offset, refs);
                    collect_references(&m.expression, offset, refs);
                }
                _ => {}
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

/// Extract default span, default text, bindable flag, and simplicity flag from a prop's binding pattern.
fn extract_prop_default(pattern: &oxc_ast::ast::BindingPattern<'_>, offset: u32, source: &str) -> (Option<Span>, Option<String>, bool, bool) {
    if let oxc_ast::ast::BindingPattern::AssignmentPattern(assign) = pattern {
        let right = &assign.right;
        // Check if default is $bindable(expr) or $bindable()
        if let Expression::CallExpression(call) = right {
            if let Expression::Identifier(ident) = &call.callee {
                if ident.name.as_str() == "$bindable" {
                    let (default_span, default_text, is_simple) = if let Some(arg) = call.arguments.first() {
                        let sp = arg.span();
                        let text = &source[sp.start as usize..sp.end as usize];
                        let expr = arg.as_expression().expect("argument should be expression");
                        (Some(Span::new(sp.start + offset, sp.end + offset)), Some(text.to_string()), is_simple_expr(expr))
                    } else {
                        (None, None, true)
                    };
                    return (default_span, default_text, true, is_simple);
                }
            }
        }
        let sp = right.span();
        let text = &source[sp.start as usize..sp.end as usize];
        let is_simple = is_simple_expr(right);
        (Some(Span::new(sp.start + offset, sp.end + offset)), Some(text.to_string()), false, is_simple)
    } else {
        (None, None, false, true)
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

/// Check if a class body contains any PropertyDefinition with $state/$state.raw initializer,
/// or constructor assignments like `this.x = $state(...)`.
fn has_class_state_runes(body: &oxc_ast::ast::ClassBody<'_>) -> bool {
    for element in &body.body {
        match element {
            oxc_ast::ast::ClassElement::PropertyDefinition(prop) => {
                if let Some(value) = &prop.value {
                    if let Some(kind) = detect_rune(value) {
                        if matches!(kind, RuneKind::State | RuneKind::StateRaw) {
                            return true;
                        }
                    }
                }
            }
            oxc_ast::ast::ClassElement::MethodDefinition(method) => {
                if method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor {
                    if let Some(body) = &method.value.body {
                        for stmt in &body.statements {
                            if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
                                if let Expression::AssignmentExpression(assign) = &es.expression {
                                    if let Some(kind) = detect_rune(&assign.right) {
                                        if matches!(kind, RuneKind::State | RuneKind::StateRaw) {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
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
                        ("$state", "eager") => Some(RuneKind::StateEager),
                        ("$effect", "tracking") => Some(RuneKind::EffectTracking),
                        ("$effect", "pending") => Some(RuneKind::EffectPending),
                        ("$props", "id") => Some(RuneKind::PropsId),
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
    let mut seen = FxHashSet::default();
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

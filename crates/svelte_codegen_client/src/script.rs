use rustc_hash::FxHashSet;

use oxc_allocator::{Allocator, CloneIn};
use oxc_ast::{NONE, ast::{
    ArrowFunctionExpression, ClassElement, Expression, FunctionBody,
    ImportDeclarationSpecifier, MethodDefinitionType, Program,
    PropertyDefinitionType, Statement, VariableDeclarator,
}};
use oxc_parser::Parser as OxcParser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::{GetSpan, SourceType};
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

    let filename = ctx.filename;

    // Take pre-parsed Program from analysis (avoids double-parsing)
    if let Some(program) = ctx.parsed.script_program.take() {
        // Clone default expressions for the transformer; originals stay for CE setter in lib.rs
        let prop_defaults: Vec<Option<Expression<'a>>> = ctx.parsed.prop_default_exprs
            .iter()
            .map(|opt| opt.as_ref().map(|e| ctx.b.clone_expr(e)))
            .collect();
        return transform_program(
            allocator,
            program,
            component_scoping,
            props,
            prop_defaults,
            dev,
            component_source,
            script_content_start,
            filename,
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
        filename,
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
        "(unknown)",
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
    filename: &str,
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
        is_ts,
        function_info_stack: Vec::new(),
        has_tracing: false,
        component_source,
        script_content_start,
        filename,
        next_arrow_name: None,
        ident_counter: 0,
        class_state_stack: Vec::new(),
        prop_default_exprs: Vec::new(),
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
        match &stmt {
            Statement::ImportDeclaration(_) => imports.push(stmt),
            _ => body.push(stmt),
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
    prop_default_exprs: Vec<Option<Expression<'a>>>,
    dev: bool,
    component_source: &str,
    script_content_start: u32,
    filename: &str,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>, bool) {
    let b = Builder::new(allocator);

    // Detect TypeScript from the program's source_type
    let is_ts = program.source_type.is_typescript();

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
        is_ts,
        function_info_stack: Vec::new(),
        has_tracing: false,
        component_source,
        script_content_start,
        filename,
        next_arrow_name: None,
        ident_counter: 0,
        class_state_stack: Vec::new(),
        prop_default_exprs,
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
        match &stmt {
            Statement::ImportDeclaration(_) => imports.push(stmt),
            _ => body.push(stmt),
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
    /// Whether the script uses TypeScript (strip type annotations during traverse).
    is_ts: bool,
    /// Stack tracking enclosing functions for $inspect.trace() context.
    function_info_stack: Vec<FunctionInfo>,
    /// Whether any $inspect.trace() was found (dev mode), triggers tracing import.
    has_tracing: bool,
    /// Full component source for line/col computation.
    component_source: &'b str,
    /// Byte offset of script content within the full component source.
    script_content_start: u32,
    /// Filename from CompileOptions (used in trace labels).
    filename: &'b str,
    /// Captured variable name for arrow functions (from VariableDeclarator).
    next_arrow_name: Option<String>,
    /// Counter for generating unique variable names (tmp, $$array_0, etc.).
    ident_counter: u32,
    /// Stack of class state field info for nested classes. Each entry maps
    /// the backing private name (e.g. "#count") to its rune kind.
    class_state_stack: Vec<ClassStateInfo>,
    /// Pre-parsed prop default expressions, indexed by prop position.
    prop_default_exprs: Vec<Option<Expression<'a>>>,
}

struct ClassStateField {
    /// Original public field name (e.g. "count") — None for private fields
    public_name: Option<String>,
    /// Private backing name (e.g. "#count")
    private_name: String,
    /// Whether this is $state (true) or $state.raw (false) — controls the `true` arg in setter
    is_state: bool,
}

struct ClassStateInfo {
    fields: Vec<ClassStateField>,
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

    /// Walk an AssignmentTarget member chain to find root store ref.
    fn extract_assign_member_store_root<'t>(&self, target: &'t oxc_ast::ast::AssignmentTarget<'a>) -> Option<&'t str> {
        match target {
            oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) => {
                let name = svelte_transform::rune_refs::find_expr_root_name(&m.object)?;
                self.component_scoping.is_store_ref(name).then_some(name)
            }
            oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) => {
                let name = svelte_transform::rune_refs::find_expr_root_name(&m.object)?;
                self.component_scoping.is_store_ref(name).then_some(name)
            }
            _ => None,
        }
    }

    /// Walk a SimpleAssignmentTarget member chain to find root store ref.
    fn extract_simple_member_store_root<'t>(&self, target: &'t oxc_ast::ast::SimpleAssignmentTarget<'a>) -> Option<&'t str> {
        match target {
            oxc_ast::ast::SimpleAssignmentTarget::StaticMemberExpression(m) => {
                let name = svelte_transform::rune_refs::find_expr_root_name(&m.object)?;
                self.component_scoping.is_store_ref(name).then_some(name)
            }
            oxc_ast::ast::SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                let name = svelte_transform::rune_refs::find_expr_root_name(&m.object)?;
                self.component_scoping.is_store_ref(name).then_some(name)
            }
            _ => None,
        }
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

    fn gen_props_statements(&mut self) -> Vec<Statement<'a>> {
        let Some(props_gen) = &self.props_gen else {
            return vec![];
        };

        let mut declarators: Vec<(&str, Expression<'a>)> = Vec::new();
        let mut seen_names: Vec<String> = vec![
            "$$slots".to_string(),
            "$$events".to_string(),
            "$$legacy".to_string(),
        ];

        for (i, prop) in props_gen.props.iter().enumerate() {
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

            if prop.default_text.is_some() {
                if prop.is_lazy_default {
                    flags |= PROPS_IS_LAZY_INITIAL;
                }

                args.push(Arg::Num(flags as f64));

                // Use pre-parsed expression when available, fallback to parse for test path
                let default_expr = if let Some(expr) = self.prop_default_exprs.get_mut(i).and_then(|e| e.take()) {
                    expr
                } else {
                    self.b.parse_expression(prop.default_text.as_deref().unwrap())
                };
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

    /// Expand destructured `$state`/`$state.raw` declarations into expanded form.
    /// Called from `exit_statements` after other transformations.
    fn expand_state_destructuring(&mut self, stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>) {
        let mut i = 0;
        while i < stmts.len() {
            let should_expand = if let Statement::VariableDeclaration(decl) = &stmts[i] {
                decl.declarations.len() == 1
                    && !matches!(&decl.declarations[0].id, oxc_ast::ast::BindingPattern::BindingIdentifier(_))
                    && decl.declarations[0].init.as_ref().is_some_and(|init| {
                        Self::detect_state_rune_kind(init).is_some()
                    })
            } else {
                false
            };

            if !should_expand {
                i += 1;
                continue;
            }

            // Take ownership of the statement
            let stmt = stmts.remove(i);
            let Statement::VariableDeclaration(mut decl) = stmt else { unreachable!() };
            let mut declarator = decl.declarations.remove(0);
            let init = declarator.init.take().unwrap();
            let rune_kind = Self::detect_state_rune_kind(&init).unwrap();

            // Extract the rune call argument
            let value = if let Expression::CallExpression(mut call) = init {
                if call.arguments.is_empty() {
                    self.b.ast.expression_object(oxc_span::SPAN, self.b.ast.vec())
                } else {
                    let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                    std::mem::swap(&mut call.arguments[0], &mut dummy);
                    dummy.into_expression()
                }
            } else {
                unreachable!()
            };

            // Generate the expanded declaration
            let replacement = self.gen_state_destructuring(
                &declarator.id,
                value,
                rune_kind,
                decl.kind,
            );

            // Insert replacement statement
            stmts.insert(i, replacement);
            self.ident_counter += 1;
            i += 1;
        }
    }

    /// Detect if an expression is a `$state(...)` or `$state.raw(...)` call.
    fn detect_state_rune_kind(expr: &Expression<'_>) -> Option<RuneKind> {
        if let Expression::CallExpression(call) = expr {
            match &call.callee {
                Expression::Identifier(id) if id.name.as_str() == "$state" => {
                    return Some(RuneKind::State);
                }
                Expression::StaticMemberExpression(member) => {
                    if let Expression::Identifier(obj) = &member.object {
                        if obj.name.as_str() == "$state" && member.property.name.as_str() == "raw" {
                            return Some(RuneKind::StateRaw);
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Generate expanded variable declaration for destructured $state/$state.raw.
    fn gen_state_destructuring(
        &mut self,
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        value: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: oxc_ast::ast::VariableDeclarationKind,
    ) -> Statement<'a> {
        let tmp_name = self.gen_unique_name("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut declarators = Vec::new();

        // First declarator: tmp = value
        let tmp_declarator = self.b.ast.variable_declarator(
            oxc_span::SPAN,
            decl_kind,
            self.b.ast.binding_pattern_binding_identifier(oxc_span::SPAN, self.b.ast.atom(tmp_name_str)),
            NONE,
            Some(value),
            false,
        );
        declarators.push(tmp_declarator);

        // Walk pattern and generate remaining declarators
        let tmp_expr = self.b.rid_expr(tmp_name_str);
        self.gen_destructure_declarators(pattern, tmp_expr, rune_kind, decl_kind, &mut declarators);

        let decl = self.b.ast.variable_declaration(
            oxc_span::SPAN,
            decl_kind,
            self.b.ast.vec_from_iter(declarators),
            false,
        );
        Statement::VariableDeclaration(self.b.alloc(decl))
    }

    /// Recursively generate declarators for destructured patterns.
    fn gen_destructure_declarators(
        &mut self,
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        accessor: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: oxc_ast::ast::VariableDeclarationKind,
        declarators: &mut Vec<oxc_ast::ast::VariableDeclarator<'a>>,
    ) {
        match pattern {
            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                let name = id.name.as_str();
                let sym_id = id.symbol_id.get();
                let is_mutated = sym_id.is_some_and(|s| self.component_scoping.is_mutated(s));

                let final_value = self.wrap_state_value(accessor, rune_kind, is_mutated);

                let declarator = self.b.ast.variable_declarator(
                    oxc_span::SPAN,
                    decl_kind,
                    self.b.ast.binding_pattern_binding_identifier(oxc_span::SPAN, self.b.ast.atom(name)),
                    NONE,
                    Some(final_value),
                    false,
                );
                declarators.push(declarator);
            }
            oxc_ast::ast::BindingPattern::ObjectPattern(obj) => {
                // Collect property key names for rest element
                let mut key_names: Vec<String> = Vec::new();
                for prop in &obj.properties {
                    if let Some(name) = Self::property_key_name(&prop.key) {
                        key_names.push(name);
                    }
                }

                for prop in &obj.properties {
                    let member = self.build_object_member_access(accessor.clone_in(self.b.ast.allocator), &prop.key, prop.computed);
                    self.gen_destructure_declarators(&prop.value, member, rune_kind, decl_kind, declarators);
                }

                if let Some(rest) = &obj.rest {
                    // $.exclude_from_object(accessor, ["key1", "key2"])
                    let keys_array = self.b.array_expr(key_names.iter().map(|k| self.b.str_expr(k)));
                    let exclude_expr = self.b.call_expr("$.exclude_from_object", [
                        Arg::Expr(accessor),
                        Arg::Expr(keys_array),
                    ]);
                    self.gen_destructure_declarators(&rest.argument, exclude_expr, rune_kind, decl_kind, declarators);
                }
            }
            oxc_ast::ast::BindingPattern::ArrayPattern(arr) => {
                // Generate intermediate: $$array_N = $.derived(() => $.to_array(accessor, len))
                let array_name = self.gen_unique_name("$$array");
                let array_name_str: &str = self.b.alloc_str(&array_name);

                let len_arg = if arr.rest.is_some() {
                    vec![Arg::Expr(accessor)]
                } else {
                    vec![Arg::Expr(accessor), Arg::Num(arr.elements.len() as f64)]
                };

                let to_array_call = self.b.call_expr("$.to_array", len_arg);
                let thunk = self.b.arrow_expr(self.b.no_params(), [self.b.expr_stmt(to_array_call)]);
                let derived_call = self.b.call_expr("$.derived", [Arg::Expr(thunk)]);

                let array_declarator = self.b.ast.variable_declarator(
                    oxc_span::SPAN,
                    decl_kind,
                    self.b.ast.binding_pattern_binding_identifier(oxc_span::SPAN, self.b.ast.atom(array_name_str)),
                    NONE,
                    Some(derived_call),
                    false,
                );
                declarators.push(array_declarator);

                // Generate element declarators
                for (idx, elem) in arr.elements.iter().enumerate() {
                    let Some(elem) = elem else { continue };
                    // $.get($$array)[idx]
                    let get_array = self.b.call_expr("$.get", [Arg::Ident(array_name_str)]);
                    let elem_access = self.b.computed_member_expr(get_array, self.b.num_expr(idx as f64));
                    self.gen_destructure_declarators(elem, elem_access, rune_kind, decl_kind, declarators);
                }

                if let Some(rest) = &arr.rest {
                    // $.get($$array).slice(idx)
                    let get_array = self.b.call_expr("$.get", [Arg::Ident(array_name_str)]);
                    let slice = self.b.static_member_expr(get_array, "slice");
                    let slice_call = self.b.ast.expression_call(
                        oxc_span::SPAN,
                        slice,
                        NONE,
                        self.b.ast.vec_from_array([oxc_ast::ast::Argument::from(self.b.num_expr(arr.elements.len() as f64))]),
                        false,
                    );
                    self.gen_destructure_declarators(&rest.argument, slice_call, rune_kind, decl_kind, declarators);
                }
            }
            oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
                // Default value: $.fallback(accessor, default)
                let default_expr = assign.right.clone_in(self.b.ast.allocator);
                let fallback = self.b.call_expr("$.fallback", [
                    Arg::Expr(accessor),
                    Arg::Expr(default_expr),
                ]);
                self.gen_destructure_declarators(&assign.left, fallback, rune_kind, decl_kind, declarators);
            }
        }
    }

    /// Wrap a value based on rune kind and mutation status.
    fn wrap_state_value(
        &self,
        value: Expression<'a>,
        rune_kind: RuneKind,
        is_mutated: bool,
    ) -> Expression<'a> {
        match rune_kind {
            RuneKind::State => {
                let proxied = if Self::should_proxy(&value) {
                    self.b.call_expr("$.proxy", [Arg::Expr(value)])
                } else {
                    value
                };
                if is_mutated {
                    self.b.call_expr("$.state", [Arg::Expr(proxied)])
                } else {
                    proxied
                }
            }
            RuneKind::StateRaw => {
                if is_mutated {
                    self.b.call_expr("$.state", [Arg::Expr(value)])
                } else {
                    value
                }
            }
            _ => value,
        }
    }

    /// Generate a unique name with a given prefix.
    /// Each prefix has its own counter so `tmp` and `$$array` don't conflict.
    fn gen_unique_name(&mut self, prefix: &str) -> String {
        // Use a simple scheme: first call for any prefix gets no suffix,
        // subsequent calls get _1, _2, etc. Track via ident_counter globally
        // but offset per-prefix using a simple convention.
        // For simplicity, just track the count of destructured statements.
        // The first destructuring gets tmp/$$array, second gets tmp_1/$$array_1.
        // We use ident_counter to count destructuring invocations.
        // gen_state_destructuring increments once, both tmp and $$array use same number.
        let n = self.ident_counter;
        if n == 0 {
            prefix.to_string()
        } else {
            format!("{}_{}", prefix, n)
        }
    }

    /// Extract property key name as a string.
    fn property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<String> {
        match key {
            oxc_ast::ast::PropertyKey::StaticIdentifier(id) => Some(id.name.to_string()),
            oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(s.value.to_string()),
            _ => None,
        }
    }

    /// Build a member access expression for an object property key.
    fn build_object_member_access(
        &self,
        object: Expression<'a>,
        key: &oxc_ast::ast::PropertyKey<'a>,
        computed: bool,
    ) -> Expression<'a> {
        if computed {
            if let Some(expr) = Self::property_key_to_expr(self.b, key) {
                self.b.computed_member_expr(object, expr)
            } else {
                object
            }
        } else {
            match key {
                oxc_ast::ast::PropertyKey::StaticIdentifier(id) => {
                    self.b.static_member_expr(object, self.b.alloc_str(id.name.as_str()))
                }
                oxc_ast::ast::PropertyKey::StringLiteral(s) => {
                    self.b.static_member_expr(object, self.b.alloc_str(s.value.as_str()))
                }
                _ => object,
            }
        }
    }

    fn property_key_to_expr<'c>(b: &'c Builder<'a>, key: &oxc_ast::ast::PropertyKey<'a>) -> Option<Expression<'a>> {
        match key {
            oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(b.str_expr(s.value.as_str())),
            oxc_ast::ast::PropertyKey::NumericLiteral(n) => Some(b.num_expr(n.value)),
            _ => None,
        }
    }

    // -----------------------------------------------------------------------
    // Class state field helpers
    // -----------------------------------------------------------------------

    /// Scan a class body for state fields and return info about them.
    fn scan_class_state_fields(&self, body: &oxc_ast::ast::ClassBody<'a>) -> ClassStateInfo {
        let mut fields = Vec::new();

        // Collect existing private names to avoid conflicts when generating backing fields
        let mut existing_private: FxHashSet<String> = FxHashSet::default();
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::PropertyDefinition(prop) = element {
                if let oxc_ast::ast::PropertyKey::PrivateIdentifier(id) = &prop.key {
                    existing_private.insert(id.name.to_string());
                }
            }
        }

        // Scan PropertyDefinitions for $state/$state.raw
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::PropertyDefinition(prop) = element {
                let Some(value) = &prop.value else { continue };
                let Some(rune_kind) = Self::detect_state_rune_kind(value) else { continue };
                let is_state = rune_kind == RuneKind::State;

                match &prop.key {
                    oxc_ast::ast::PropertyKey::PrivateIdentifier(id) => {
                        // Private field: #name = $state(...) → just rewrite callee
                        fields.push(ClassStateField {
                            public_name: None,
                            private_name: id.name.to_string(),
                            is_state,
                        });
                    }
                    oxc_ast::ast::PropertyKey::StaticIdentifier(id) if !prop.computed => {
                        // Public field: name = $state(...) → private backing + getter/setter
                        let name = id.name.to_string();
                        let mut backing = format!("#{}", name);
                        // Deconflict if private name already exists
                        while existing_private.contains(backing.trim_start_matches('#')) {
                            backing = format!("#_{}", backing.trim_start_matches('#'));
                        }
                        existing_private.insert(backing.trim_start_matches('#').to_string());
                        fields.push(ClassStateField {
                            public_name: Some(name),
                            private_name: backing.trim_start_matches('#').to_string(),
                            is_state,
                        });
                    }
                    _ => {}
                }
            }
        }

        // Scan constructor for `this.name = $state(...)` assignments
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::MethodDefinition(method) = element {
                if method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor {
                    if let Some(func_body) = &method.value.body {
                        for stmt in &func_body.statements {
                            if let Statement::ExpressionStatement(es) = stmt {
                                if let Expression::AssignmentExpression(assign) = &es.expression {
                                    if assign.operator == oxc_ast::ast::AssignmentOperator::Assign {
                                        if let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member) = &assign.left {
                                            if let Expression::ThisExpression(_) = &member.object {
                                                if let Some(rune_kind) = Self::detect_state_rune_kind(&assign.right) {
                                                    let name = member.property.name.to_string();
                                                    let is_state = rune_kind == RuneKind::State;
                                                    let mut backing = format!("#{}", name);
                                                    while existing_private.contains(backing.trim_start_matches('#')) {
                                                        backing = format!("#_{}", backing.trim_start_matches('#'));
                                                    }
                                                    existing_private.insert(backing.trim_start_matches('#').to_string());
                                                    fields.push(ClassStateField {
                                                        public_name: Some(name),
                                                        private_name: backing.trim_start_matches('#').to_string(),
                                                        is_state,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        ClassStateInfo { fields }
    }

    /// Rewrite class body: replace state fields with private backing + getter/setter.
    fn rewrite_class_body(
        &self,
        body: &mut oxc_ast::ast::ClassBody<'a>,
        info: &ClassStateInfo,
    ) {
        use oxc_ast::ast::ClassElement;

        // Build a lookup: field name → ClassStateField for quick matching
        let public_fields: std::collections::HashMap<&str, &ClassStateField> = info.fields.iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();
        let private_fields: FxHashSet<&str> = info.fields.iter()
            .filter(|f| f.public_name.is_none())
            .map(|f| f.private_name.as_str())
            .collect();

        let mut new_body: Vec<ClassElement<'a>> = Vec::new();
        // Track which public field names were handled from PropertyDefinition
        let mut handled_public: FxHashSet<String> = FxHashSet::default();

        // Take ownership of old body elements
        let old_elements: Vec<ClassElement<'a>> = {
            let mut temp = self.b.ast.vec();
            std::mem::swap(&mut body.body, &mut temp);
            temp.into_iter().collect()
        };

        for element in old_elements {
            match element {
                ClassElement::PropertyDefinition(mut prop) => {
                    // Check if it's a state field
                    let is_state_prop = prop.value.as_ref().is_some_and(|v| Self::detect_state_rune_kind(v).is_some());
                    if !is_state_prop {
                        new_body.push(ClassElement::PropertyDefinition(prop));
                        continue;
                    }

                    match &prop.key {
                        oxc_ast::ast::PropertyKey::PrivateIdentifier(id) => {
                            let name = id.name.to_string();
                            if private_fields.contains(name.as_str()) {
                                // Private field: just rewrite $state(arg) → $.state(arg)
                                if let Some(Expression::CallExpression(call)) = &mut prop.value {
                                    call.callee = self.b.rid_expr("$.state");
                                }
                                new_body.push(ClassElement::PropertyDefinition(prop));
                            } else {
                                new_body.push(ClassElement::PropertyDefinition(prop));
                            }
                        }
                        oxc_ast::ast::PropertyKey::StaticIdentifier(id) if !prop.computed => {
                            let name = id.name.to_string();
                            if let Some(field_info) = public_fields.get(name.as_str()) {
                                handled_public.insert(name.clone());
                                // Extract the rune argument
                                let arg = if let Some(Expression::CallExpression(mut call)) = prop.value.take() {
                                    if call.arguments.is_empty() {
                                        None
                                    } else {
                                        let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                        std::mem::swap(&mut call.arguments[0], &mut dummy);
                                        Some(dummy.into_expression())
                                    }
                                } else {
                                    None
                                };

                                // Generate: #backing = $.state(arg)
                                let state_call = if let Some(arg) = arg {
                                    self.b.call_expr("$.state", [Arg::Expr(arg)])
                                } else {
                                    self.b.call_expr("$.state", std::iter::empty::<Arg<'a, '_>>())
                                };
                                new_body.push(self.b.class_private_field(
                                    &field_info.private_name,
                                    Some(state_call),
                                ));

                                // Generate getter: get name() { return $.get(this.#backing); }
                                let get_call = self.b.call_expr("$.get", [Arg::Expr(
                                    self.b.this_private_member(&field_info.private_name),
                                )]);
                                let return_stmt = self.b.return_stmt(get_call);
                                new_body.push(self.b.class_getter(
                                    self.b.public_key(&name),
                                    vec![return_stmt],
                                ));

                                // Generate setter: set name(value) { $.set(this.#backing, value, true?); }
                                let mut set_args: Vec<Arg<'a, '_>> = vec![
                                    Arg::Expr(self.b.this_private_member(&field_info.private_name)),
                                    Arg::Ident("value"),
                                ];
                                if field_info.is_state {
                                    set_args.push(Arg::Bool(true));
                                }
                                let set_call = self.b.call_stmt("$.set", set_args);
                                new_body.push(self.b.class_setter(
                                    self.b.public_key(&name),
                                    "value",
                                    vec![set_call],
                                ));
                            } else {
                                new_body.push(ClassElement::PropertyDefinition(prop));
                            }
                        }
                        _ => {
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                    }
                }
                ClassElement::MethodDefinition(mut method) => {
                    if method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor {
                        // Insert #backing; + getter + setter for constructor-originating state fields
                        let ctor_fields: Vec<&ClassStateField> = info.fields.iter()
                            .filter(|f| f.public_name.is_some() && !handled_public.contains(f.public_name.as_deref().unwrap()))
                            .collect();
                        for field_info in &ctor_fields {
                            let name = field_info.public_name.as_deref().unwrap();
                            // #backing; (no init)
                            new_body.push(self.b.class_private_field(&field_info.private_name, None));
                            // getter
                            let get_call = self.b.call_expr("$.get", [Arg::Expr(
                                self.b.this_private_member(&field_info.private_name),
                            )]);
                            let return_stmt = self.b.return_stmt(get_call);
                            new_body.push(self.b.class_getter(self.b.public_key(name), vec![return_stmt]));
                            // setter
                            let mut set_args: Vec<Arg<'a, '_>> = vec![
                                Arg::Expr(self.b.this_private_member(&field_info.private_name)),
                                Arg::Ident("value"),
                            ];
                            if field_info.is_state {
                                set_args.push(Arg::Bool(true));
                            }
                            let set_call = self.b.call_stmt("$.set", set_args);
                            new_body.push(self.b.class_setter(self.b.public_key(name), "value", vec![set_call]));
                        }
                        self.rewrite_constructor(&mut method, info);
                    }
                    new_body.push(ClassElement::MethodDefinition(method));
                }
                other => {
                    new_body.push(other);
                }
            }
        }

        body.body = self.b.ast.vec_from_iter(new_body);
    }

    /// Rewrite constructor: replace `this.name = $state(...)` with `this.#backing = $.state(...)`.
    /// Also insert `#backing;` property definitions and getter/setter before the constructor.
    fn rewrite_constructor(
        &self,
        method: &mut oxc_allocator::Box<'a, oxc_ast::ast::MethodDefinition<'a>>,
        info: &ClassStateInfo,
    ) {
        let Some(func_body) = &mut method.value.body else { return };

        // Build lookup for constructor-originating fields
        let ctor_fields: std::collections::HashMap<&str, &ClassStateField> = info.fields.iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();

        for stmt in func_body.statements.iter_mut() {
            if let Statement::ExpressionStatement(es) = stmt {
                if let Expression::AssignmentExpression(assign) = &mut es.expression {
                    if assign.operator == oxc_ast::ast::AssignmentOperator::Assign {
                        if let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member) = &assign.left {
                            if let Expression::ThisExpression(_) = &member.object {
                                let name = member.property.name.to_string();
                                if let Some(field_info) = ctor_fields.get(name.as_str()) {
                                    // Rewrite: this.name = $state(arg) → this.#backing = $.state(arg)
                                    if let Expression::CallExpression(call) = &mut assign.right {
                                        call.callee = self.b.rid_expr("$.state");
                                    }
                                    // Change left side to this.#backing
                                    let new_left = self.b.this_private_member(&field_info.private_name);
                                    // We need to convert Expression to AssignmentTarget
                                    // For private field: use PrivateFieldExpression
                                    if let Expression::PrivateFieldExpression(pfe) = new_left {
                                        assign.left = oxc_ast::ast::AssignmentTarget::PrivateFieldExpression(pfe);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Check if we're inside a class body that has a private state field with given name.
    fn is_private_state_field(&self, name: &str) -> bool {
        self.class_state_stack.last().is_some_and(|info| {
            info.fields.iter().any(|f| f.public_name.is_none() && f.private_name == name)
        })
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

/// Sanitize a filename for use in trace labels by inserting a zero-width space
/// after each `/` to prevent devtools from treating it as a clickable link.
pub(crate) fn sanitize_location(filename: &str) -> String {
    filename.replace('/', "/\u{200b}")
}

/// Compute 1-based line and column from source text and byte offset.
pub(crate) fn compute_line_col(source: &str, offset: u32) -> (usize, usize) {
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
    fn enter_class_body(
        &mut self,
        node: &mut oxc_ast::ast::ClassBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let info = self.scan_class_state_fields(node);
        self.class_state_stack.push(info);
    }

    fn exit_class_body(
        &mut self,
        node: &mut oxc_ast::ast::ClassBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.body.retain(|member| match member {
                ClassElement::PropertyDefinition(prop) => {
                    !prop.declare
                        && prop.r#type != PropertyDefinitionType::TSAbstractPropertyDefinition
                }
                ClassElement::MethodDefinition(method) => {
                    method.r#type != MethodDefinitionType::TSAbstractMethodDefinition
                }
                ClassElement::TSIndexSignature(_) => false,
                _ => true,
            });
        }

        let Some(info) = self.class_state_stack.pop() else { return };
        if info.fields.is_empty() {
            return;
        }
        self.rewrite_class_body(node, &info);
    }

    fn enter_function(
        &mut self,
        node: &mut oxc_ast::ast::Function<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_parameters = None;
            node.return_type = None;
            node.this_param = None;
        }
        let name = node.id.as_ref()
            .map(|id| id.name.to_string())
            .or_else(|| self.next_arrow_name.take());
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
        if self.is_ts {
            node.type_parameters = None;
            node.return_type = None;
        }
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
            let sanitized = sanitize_location(self.filename);
            let label = format!("{func_name} ({sanitized}:{line}:{col})");
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
        // Strip TypeScript declarations and type-only imports/exports
        if self.is_ts {
            // Remove individual type specifiers from imports/exports
            for stmt in stmts.iter_mut() {
                match stmt {
                    Statement::ImportDeclaration(import) => {
                        if let Some(specs) = &mut import.specifiers {
                            specs.retain(|spec| {
                                !matches!(spec, ImportDeclarationSpecifier::ImportSpecifier(s) if s.import_kind.is_type())
                            });
                        }
                    }
                    Statement::ExportNamedDeclaration(export) if export.declaration.is_none() => {
                        export.specifiers.retain(|spec| !spec.export_kind.is_type());
                    }
                    _ => {}
                }
            }

            // Remove entire TS-only statements
            stmts.retain(|stmt| match stmt {
                Statement::TSTypeAliasDeclaration(_)
                | Statement::TSInterfaceDeclaration(_)
                | Statement::TSModuleDeclaration(_)
                | Statement::TSEnumDeclaration(_) => false,
                Statement::VariableDeclaration(decl) if decl.declare => false,
                Statement::FunctionDeclaration(func) if func.declare => false,
                Statement::ClassDeclaration(class) if class.declare => false,
                Statement::ImportDeclaration(import) if import.import_kind.is_type() => false,
                Statement::ExportNamedDeclaration(export) if export.export_kind.is_type() => false,
                Statement::ExportAllDeclaration(export) if export.export_kind.is_type() => false,
                // Remove imports/exports left with no specifiers after type-specifier filtering
                Statement::ImportDeclaration(import) => {
                    import.specifiers.as_ref().is_none_or(|s| !s.is_empty())
                }
                Statement::ExportNamedDeclaration(export) => {
                    export.declaration.is_some() || !export.specifiers.is_empty()
                }
                _ => true,
            });
        }

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

        // Expand destructured $state/$state.raw declarations
        self.expand_state_destructuring(stmts);

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

    fn enter_formal_parameter(
        &mut self,
        node: &mut oxc_ast::ast::FormalParameter<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.readonly = false;
            node.r#override = false;
        }
    }

    fn enter_catch_parameter(
        &mut self,
        node: &mut oxc_ast::ast::CatchParameter<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
        }
    }

    fn enter_call_expression(
        &mut self,
        node: &mut oxc_ast::ast::CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_arguments = None;
        }
        // Capture callee text for arrow/function arguments (used by $inspect.trace auto-label).
        // e.g. foo(() => { $inspect.trace(); }) → label "foo(...)"
        let has_fn_arg = node.arguments.iter().any(|arg| {
            matches!(arg, oxc_ast::ast::Argument::ArrowFunctionExpression(_)
                | oxc_ast::ast::Argument::FunctionExpression(_))
        });
        if has_fn_arg {
            let start = (self.script_content_start + node.callee.span().start) as usize;
            let end = (self.script_content_start + node.callee.span().end) as usize;
            if end <= self.component_source.len() {
                let callee_text = &self.component_source[start..end];
                self.next_arrow_name = Some(format!("{callee_text}(...)"));
            }
        }
    }

    fn enter_new_expression(
        &mut self,
        node: &mut oxc_ast::ast::NewExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_arguments = None;
        }
    }

    fn enter_tagged_template_expression(
        &mut self,
        node: &mut oxc_ast::ast::TaggedTemplateExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_arguments = None;
        }
    }

    fn enter_class(
        &mut self,
        node: &mut oxc_ast::ast::Class<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_parameters = None;
            node.super_type_arguments = None;
            node.implements.clear();
            node.r#abstract = false;
        }
    }

    fn enter_property_definition(
        &mut self,
        node: &mut oxc_ast::ast::PropertyDefinition<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.readonly = false;
            node.r#override = false;
            node.optional = false;
            node.definite = false;
        }
    }

    fn enter_accessor_property(
        &mut self,
        node: &mut oxc_ast::ast::AccessorProperty<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.r#override = false;
            node.definite = false;
        }
    }

    fn enter_object_property(
        &mut self,
        node: &mut oxc_ast::ast::ObjectProperty<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Capture property key name for arrow/function values (used by $inspect.trace auto-label).
        // e.g. { handler: () => { $inspect.trace(); } } → label "handler"
        if !node.computed {
            let is_fn_value = matches!(&node.value,
                Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_));
            if is_fn_value || node.method {
                if let oxc_ast::ast::PropertyKey::StaticIdentifier(id) = &node.key {
                    self.next_arrow_name = Some(id.name.to_string());
                }
            }
        }
    }

    fn enter_method_definition(
        &mut self,
        node: &mut oxc_ast::ast::MethodDefinition<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.accessibility = None;
            node.r#override = false;
            node.optional = false;
        }
    }

    fn enter_variable_declarator(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.definite = false;
        }
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
                        } else if kind == RuneKind::State {
                            // Wrap proxyable args (arrays/objects) in $.proxy()
                            let needs_proxy = call.arguments[0].as_expression()
                                .is_some_and(|e| Self::should_proxy(e));
                            if needs_proxy {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut call.arguments[0], &mut dummy);
                                let inner = dummy.into_expression();
                                let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                                call.arguments[0] = oxc_ast::ast::Argument::from(proxied);
                            }
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
                RuneKind::StateEager => {
                    // $state.eager(expr) → $.eager(() => expr)
                    let arg = call.arguments.remove(0).into_expression();
                    node.init = Some(self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(arg))]));
                }
                RuneKind::EffectPending => {
                    // $effect.pending() → $.eager($.pending)
                    let pending_call = self.b.call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
                    node.init = Some(self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(pending_call))]));
                }
                _ => {
                    // Other rune kinds — put back the call unchanged
                    node.init = Some(Expression::CallExpression(call));
                }
            }
        }
    }

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        // Strip TS expression wrappers (as, satisfies, non-null, type assertion, instantiation)
        if self.is_ts {
            loop {
                match node {
                    Expression::TSAsExpression(_)
                    | Expression::TSSatisfiesExpression(_)
                    | Expression::TSNonNullExpression(_)
                    | Expression::TSTypeAssertion(_)
                    | Expression::TSInstantiationExpression(_) => {
                        let inner = match self.b.move_expr(node) {
                            Expression::TSAsExpression(ts) => ts.unbox().expression,
                            Expression::TSSatisfiesExpression(ts) => ts.unbox().expression,
                            Expression::TSNonNullExpression(ts) => ts.unbox().expression,
                            Expression::TSTypeAssertion(ts) => ts.unbox().expression,
                            Expression::TSInstantiationExpression(ts) => ts.unbox().expression,
                            _ => unreachable!(),
                        };
                        *node = inner;
                    }
                    _ => break,
                }
            }
        }
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

                // $state.eager(expr) → $.eager(() => expr)
                if let Expression::StaticMemberExpression(member) = &call.callee {
                    if let Expression::Identifier(obj) = &member.object {
                        match (obj.name.as_str(), member.property.name.as_str()) {
                            ("$state", "eager") => {
                                let Expression::CallExpression(mut call) = self.b.move_expr(node) else { unreachable!() };
                                let arg = call.arguments.remove(0).into_expression();
                                *node = self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(arg))]);
                                return;
                            }
                            ("$effect", "pending") => {
                                let pending_call = self.b.call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
                                *node = self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(pending_call))]);
                                return;
                            }
                            _ => {}
                        }
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
            // Private field read wrapping handled in exit_expression to avoid infinite re-entry
            _ => {}
        }
    }

    fn exit_expression(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        // Private state field read: this.#field → $.get(this.#field)
        // Done in exit to avoid infinite re-entry (enter would re-visit the created node).
        if let Expression::PrivateFieldExpression(pfe) = node {
            if matches!(&pfe.object, Expression::ThisExpression(_))
                && self.is_private_state_field(pfe.field.name.as_str())
            {
                let field_expr = self.b.move_expr(node);
                *node = self.b.call_expr("$.get", [Arg::Expr(field_expr)]);
                return;
            }
        }

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

        // Deep store mutation: $store.field = val → $.store_mutate(store, ...)
        if let Some(root_name) = self.extract_assign_member_store_root(&assign.left) {
            let root_name = root_name.to_string();
            let base_name = root_name[1..].to_string();
            let alloc = self.b.ast.allocator;
            // Replace root identifier in the member chain with $.untrack($store)
            svelte_transform::rune_refs::replace_expr_root_in_assign_target(
                &mut assign.left,
                svelte_transform::rune_refs::make_untrack(alloc, &root_name),
            );
            // Take modified assignment as the mutation expression
            let mutation = self.b.move_expr(node);
            let untracked = svelte_transform::rune_refs::make_untrack(alloc, &root_name);
            *node = svelte_transform::rune_refs::make_store_mutate(alloc, &base_name, mutation, untracked);
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

        // Deep store update: $store.count++ → $.store_mutate(store, ...)
        if let Some(root_name) = self.extract_simple_member_store_root(&upd.argument) {
            let root_name = root_name.to_string();
            let base_name = root_name[1..].to_string();
            let alloc = self.b.ast.allocator;
            svelte_transform::rune_refs::replace_expr_root_in_simple_target(
                &mut upd.argument,
                svelte_transform::rune_refs::make_untrack(alloc, &root_name),
            );
            let mutation = self.b.move_expr(node);
            let untracked = svelte_transform::rune_refs::make_untrack(alloc, &root_name);
            *node = svelte_transform::rune_refs::make_store_mutate(alloc, &base_name, mutation, untracked);
        }
    }
}

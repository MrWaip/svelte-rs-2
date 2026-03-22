mod props;
mod state;
mod traverse;

use rustc_hash::FxHashSet;

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Program, Statement};
use oxc_ast::Comment;
use oxc_parser::Parser as OxcParser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::SourceType;
use oxc_traverse::traverse_mut;

use svelte_analyze::{ComponentScoping, PropsAnalysis};
use svelte_ast::ScriptLanguage;
use svelte_analyze::RuneKind;

use crate::builder::Builder;
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Props flag constants (must match svelte/src/constants.js)
// ---------------------------------------------------------------------------

pub(super) const PROPS_IS_IMMUTABLE: u32 = 1;
pub(super) const PROPS_IS_RUNES: u32 = 1 << 1;
pub(super) const PROPS_IS_UPDATED: u32 = 1 << 2;
pub(super) const PROPS_IS_BINDABLE: u32 = 1 << 3;
pub(super) const PROPS_IS_LAZY_INITIAL: u32 = 1 << 4;

/// Script transformation result carrying statements and comment metadata
/// for preserving JSDoc/leading comments in the final output.
pub struct ScriptOutput<'a> {
    pub imports: Vec<Statement<'a>>,
    pub body: Vec<Statement<'a>>,
    pub has_tracing: bool,
    /// Comments from the parsed script program (for OXC Codegen to print).
    pub comments: Vec<Comment>,
    /// Source text the comment spans index into.
    pub source_text: &'a str,
    /// span.end of the original script program (for trailing comment matching).
    pub program_span_end: u32,
}

/// Parse and transform the script block.
pub fn gen_script<'a>(ctx: &mut Ctx<'a>, dev: bool) -> ScriptOutput<'a> {
    if ctx.component.script.is_none() {
        return ScriptOutput {
            imports: vec![], body: vec![], has_tracing: false,
            comments: vec![], source_text: "", program_span_end: 0,
        };
    };

    let allocator = ctx.b.ast.allocator;
    let component_scoping = &ctx.analysis.scoping;
    let props = ctx.analysis.props.as_ref();
    let component_source = &ctx.component.source;
    let script_content_start = ctx.component.script.as_ref().unwrap().content_span.start;

    let filename = ctx.filename;

    // Take pre-parsed Program from analysis (avoids double-parsing)
    if let Some(program) = ctx.parsed.program.take() {
        let b = Builder::new(allocator);
        let prop_defaults: Vec<Option<Expression<'a>>> = props
            .map(|pa| pa.props.iter().map(|p| {
                p.default_text.as_deref().map(|text| b.parse_expression(text))
            }).collect())
            .unwrap_or_default();
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
) -> ScriptOutput<'a> {
    transform_script_text(
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
    )
}

/// Parse the script source and apply rune transformations.
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
) -> ScriptOutput<'a> {
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

    let prop_default_exprs: Vec<Option<Expression<'a>>> = match &props_gen {
        Some(pg) => pg.props.iter().map(|prop| {
            prop.default_text.as_deref().map(|text| b.parse_expression(text))
        }).collect(),
        None => Vec::new(),
    };

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
        prop_default_exprs,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_scoping, ());

    // Post-traverse: wrap $derived arguments in thunks
    if !transformer.derived_pending.is_empty() {
        traverse::wrap_derived_thunks(&b, &mut program, &transformer.derived_pending);
    }

    let has_tracing = transformer.has_tracing;

    let comments: Vec<Comment> = program.comments.iter().copied().collect();
    let source_text = program.source_text;
    let program_span_end = program.span.end;

    let mut imports = vec![];
    let mut body = vec![];

    for stmt in program.body {
        match &stmt {
            Statement::ImportDeclaration(_) => imports.push(stmt),
            _ => body.push(stmt),
        }
    }

    ScriptOutput { imports, body, has_tracing, comments, source_text, program_span_end }
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
) -> ScriptOutput<'a> {
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
        traverse::wrap_derived_thunks(&b, &mut program, &transformer.derived_pending);
    }

    let has_tracing = transformer.has_tracing;

    let comments: Vec<Comment> = program.comments.iter().copied().collect();
    let source_text = program.source_text;
    let program_span_end = program.span.end;

    let mut imports = vec![];
    let mut body = vec![];

    for stmt in program.body {
        match &stmt {
            Statement::ImportDeclaration(_) => imports.push(stmt),
            _ => body.push(stmt),
        }
    }

    ScriptOutput { imports, body, has_tracing, comments, source_text, program_span_end }
}

pub(super) enum PropKind {
    Source,
    NonSource(String),
}

pub(super) struct PropsGenInfo {
    pub(super) props: Vec<PropGenItem>,
}

impl PropsGenInfo {
    pub(super) fn from_analysis(pa: &PropsAnalysis) -> Self {
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

pub(super) struct PropGenItem {
    pub(super) local_name: String,
    pub(super) prop_name: String,
    pub(super) is_prop_source: bool,
    pub(super) is_bindable: bool,
    pub(super) is_rest: bool,
    pub(super) is_mutated: bool,
    pub(super) default_text: Option<String>,
    pub(super) is_lazy_default: bool,
}

pub(super) struct FunctionInfo {
    pub(super) is_async: bool,
    pub(super) name: Option<String>,
    /// Byte offset of the function keyword in the script source (for auto-label location).
    pub(super) span_start: u32,
}

pub(super) struct ClassStateField {
    /// Original public field name (e.g. "count") — None for private fields
    pub(super) public_name: Option<String>,
    /// Private backing name (e.g. "#count")
    pub(super) private_name: String,
    /// Whether this is $state (true) or $state.raw (false) — controls the `true` arg in setter
    pub(super) is_state: bool,
}

pub(super) struct ClassStateInfo {
    pub(super) fields: Vec<ClassStateField>,
}

pub(super) struct ScriptTransformer<'b, 'a> {
    pub(super) b: &'b Builder<'a>,
    /// ComponentScoping — source of truth for rune kind + mutation status.
    pub(super) component_scoping: &'b ComponentScoping,
    /// OXC scoping from SemanticBuilder — used to resolve references to symbols.
    pub(super) scoping: Scoping,
    pub(super) props_gen: Option<PropsGenInfo>,
    /// SymbolIds of $derived/$derived.by runes whose init needs post-traverse wrapping.
    pub(super) derived_pending: FxHashSet<oxc_semantic::SymbolId>,
    /// Whether to strip `export` keywords from declarations. True for component scripts,
    /// false for module compilation where exports must be preserved.
    pub(super) strip_exports: bool,
    /// Whether dev-mode transforms are enabled ($inspect → $.inspect).
    pub(super) dev: bool,
    /// Whether the script uses TypeScript (strip type annotations during traverse).
    pub(super) is_ts: bool,
    /// Stack tracking enclosing functions for $inspect.trace() context.
    pub(super) function_info_stack: Vec<FunctionInfo>,
    /// Whether any $inspect.trace() was found (dev mode), triggers tracing import.
    pub(super) has_tracing: bool,
    /// Full component source for line/col computation.
    pub(super) component_source: &'b str,
    /// Byte offset of script content within the full component source.
    pub(super) script_content_start: u32,
    /// Filename from CompileOptions (used in trace labels).
    pub(super) filename: &'b str,
    /// Captured variable name for arrow functions (from VariableDeclarator).
    pub(super) next_arrow_name: Option<String>,
    /// Counter for generating unique variable names (tmp, $$array_0, etc.).
    pub(super) ident_counter: u32,
    /// Stack of class state field info for nested classes. Each entry maps
    /// the backing private name (e.g. "#count") to its rune kind.
    pub(super) class_state_stack: Vec<ClassStateInfo>,
    /// Pre-parsed prop default expressions, indexed by prop position.
    pub(super) prop_default_exprs: Vec<Option<Expression<'a>>>,
}

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    /// Resolve a binding identifier to its rune kind and mutated status.
    /// Only root-scope symbols are considered runes (skips shadowing parameters).
    pub(super) fn rune_for_binding(
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
    pub(super) fn rune_for_ref(
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
    pub(super) fn prop_kind_for_ref(
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

    pub(super) fn should_proxy(e: &Expression) -> bool {
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
    /// Returns `(dollar_name, base_name)` — e.g. `("$count", "count")`.
    pub(super) fn extract_assign_member_store_root<'t>(&self, target: &'t oxc_ast::ast::AssignmentTarget<'a>) -> Option<(&'t str, &'t str)> {
        match target {
            oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) => {
                let name = svelte_transform::rune_refs::find_expr_root_name(&m.object)?;
                self.component_scoping.store_base_name(name).map(|base| (name, base))
            }
            oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) => {
                let name = svelte_transform::rune_refs::find_expr_root_name(&m.object)?;
                self.component_scoping.store_base_name(name).map(|base| (name, base))
            }
            _ => None,
        }
    }

    /// Walk a SimpleAssignmentTarget member chain to find root store ref.
    /// Returns `(dollar_name, base_name)` — e.g. `("$count", "count")`.
    pub(super) fn extract_simple_member_store_root<'t>(&self, target: &'t oxc_ast::ast::SimpleAssignmentTarget<'a>) -> Option<(&'t str, &'t str)> {
        match target {
            oxc_ast::ast::SimpleAssignmentTarget::StaticMemberExpression(m) => {
                let name = svelte_transform::rune_refs::find_expr_root_name(&m.object)?;
                self.component_scoping.store_base_name(name).map(|base| (name, base))
            }
            oxc_ast::ast::SimpleAssignmentTarget::ComputedMemberExpression(m) => {
                let name = svelte_transform::rune_refs::find_expr_root_name(&m.object)?;
                self.component_scoping.store_base_name(name).map(|base| (name, base))
            }
            _ => None,
        }
    }
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

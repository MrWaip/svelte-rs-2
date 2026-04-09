use compact_str::CompactString;
use svelte_span::Span;

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
    /// `const props = $props()` — identifier pattern, not destructured
    pub is_identifier_pattern: bool,
    /// Full statement spans that introduced this props declaration.
    pub declaration_spans: Vec<Span>,
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
    /// Pre-extracted literal value from init expression (for known_values).
    /// For runes: literal value of first argument (e.g. `$state(42)` → `"42"`).
    /// For non-runes: literal value of init (e.g. `const x = "hello"` → `"hello"`).
    pub init_literal: Option<CompactString>,
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
    EffectPre,
    EffectRoot,
    EffectTracking,
    Props,
    Bindable,
    StateEager,
    EffectPending,
    Inspect,
    InspectWith,
    InspectTrace,
    Host,
    PropsId,
}

impl RuneKind {
    pub fn is_derived(&self) -> bool {
        matches!(self, RuneKind::Derived | RuneKind::DerivedBy)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            RuneKind::State => "$state",
            RuneKind::StateRaw => "$state.raw",
            RuneKind::StateEager => "$state.eager",
            RuneKind::Derived => "$derived",
            RuneKind::DerivedBy => "$derived.by",
            RuneKind::Effect => "$effect",
            RuneKind::EffectPre => "$effect.pre",
            RuneKind::EffectRoot => "$effect.root",
            RuneKind::EffectTracking => "$effect.tracking",
            RuneKind::EffectPending => "$effect.pending",
            RuneKind::Props => "$props",
            RuneKind::PropsId => "$props.id",
            RuneKind::Bindable => "$bindable",
            RuneKind::Inspect => "$inspect",
            RuneKind::InspectWith => "$inspect().with",
            RuneKind::InspectTrace => "$inspect.trace",
            RuneKind::Host => "$host",
        }
    }
}

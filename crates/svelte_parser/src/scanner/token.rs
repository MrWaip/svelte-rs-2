use svelte_span::{GetSpan, Span};
use svelte_diagnostics::Diagnostic;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Text,
    Comment,
    StartTag(StartTag),
    EndTag(EndTag),
    Interpolation(ExpressionTag),
    StartIfTag(StartIfTag),
    ElseTag(ElseTag),
    ScriptTag(ScriptTag),
    EndIfTag,
    StartEachTag(StartEachTag),
    EndEachTag,
    StartSnippetTag(StartSnippetTag),
    EndSnippetTag,
    RenderTag(RenderTagToken),
    HtmlTag(HtmlTagToken),
    ConstTag(ConstTagToken),
    StartKeyTag(StartKeyTag),
    EndKeyTag,
    StartAwaitTag(StartAwaitTag),
    AwaitClauseTag(AwaitClauseTag),
    EndAwaitTag,
    StyleTag(StyleTag),
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptTag {
    pub content_span: Span,
    pub is_typescript: bool,
    pub is_module: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartTag {
    pub attributes: Vec<Attribute>,
    pub name_span: Span,
    pub self_closing: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EndTag {
    pub name_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartEachTag {
    pub collection_span: Span,
    pub context_span: Span,
    pub index_span: Option<Span>,
    pub key_span: Option<Span>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HTMLAttribute {
    pub name_span: Span,
    pub value: AttributeValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    HTMLAttribute(HTMLAttribute),
    ExpressionTag(ExpressionTag),
    ClassDirective(ClassDirective),
    StyleDirective(StyleDirective),
    BindDirective(BindDirective),
    UseDirective(UseDirective),
    /// LEGACY(svelte4): on:directive syntax. Deprecated in Svelte 5, remove in Svelte 6.
    OnDirectiveLegacy(OnDirectiveLegacy),
    TransitionDirective(TransitionDirective),
    AnimateDirective(AnimateDirective),
    /// {@attach expr} — element attachment (Svelte 5.29+)
    AttachTag(AttachTagToken),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AnimateDirective {
    /// Name after "animate:" (e.g., "flip" in `animate:flip`).
    pub name_span: Span,
    /// Expression span if `={expr}` was provided.
    pub expression_span: Span,
    /// Whether an expression was provided.
    pub has_expression: bool,
}

/// {@attach expr} — element attachment (Svelte 5.29+)
#[derive(Debug, PartialEq, Eq)]
pub struct AttachTagToken {
    /// Span of the JS expression inside `{@attach expr}`.
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClassDirective {
    pub shorthand: bool,
    pub name_span: Span,
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StyleDirective {
    pub shorthand: bool,
    pub name_span: Span,
    pub value: AttributeValue,
    pub important: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BindDirective {
    pub shorthand: bool,
    pub name_span: Span,
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UseDirective {
    pub shorthand: bool,
    pub name_span: Span,
    pub expression_span: Span,
}

/// LEGACY(svelte4): on:directive syntax. Deprecated in Svelte 5, remove in Svelte 6.
#[derive(Debug, PartialEq, Eq)]
pub struct OnDirectiveLegacy {
    /// Event name after "on:" (e.g., "click" in `on:click`).
    pub name_span: Span,
    /// Handler expression. Empty if bubble event (no `={...}`).
    pub expression_span: Span,
    /// Modifiers from pipe-separated list (e.g., ["preventDefault", "once"]).
    pub modifiers: Vec<Span>,
    /// Whether an expression was provided (`on:click={handler}` vs `on:click`).
    pub has_expression: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TransitionDirective {
    /// Name after the directive prefix (e.g., "fade" in `transition:fade`).
    pub name_span: Span,
    /// Expression span if `={expr}` was provided.
    pub expression_span: Span,
    /// Modifiers from pipe-separated list (e.g., "local", "global").
    pub modifiers: Vec<Span>,
    /// Whether an expression was provided.
    pub has_expression: bool,
    /// The directive prefix: "transition", "in", or "out".
    pub direction_prefix: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeValue {
    String(Span),
    ExpressionTag(ExpressionTag),
    Concatenation(Concatenation),
    Empty,
}

/// Any expression in curly braces `{ 1 + 1 }` or `{ name }` in the template.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionTag {
    pub span: Span,
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Concatenation {
    pub span: Span,
    pub parts: Vec<ConcatenationPart>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConcatenationPart {
    String(Span),
    Expression(ExpressionTag),
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl GetSpan for Token {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StyleTag {
    pub content_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartSnippetTag {
    pub name_span: Span,
    pub params_span: Option<Span>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RenderTagToken {
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HtmlTagToken {
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConstTagToken {
    pub declaration_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartKeyTag {
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartAwaitTag {
    pub expression_span: Span,
    /// Then binding span if short form `{#await expr then val}`. None for implicit form.
    pub value_span: Option<Span>,
    /// Catch binding span if short form `{#await expr catch err}`. None for implicit form.
    pub error_span: Option<Span>,
    /// Which fragment to start collecting into.
    pub initial_clause: AwaitInitialClause,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AwaitInitialClause {
    /// Implicit form: `{#await expr}` — pending content follows.
    Pending,
    /// Short form: `{#await expr then val}` — then content follows.
    Then,
    /// Short form: `{#await expr catch err}` — catch content follows.
    Catch,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AwaitClauseTag {
    pub clause: AwaitClause,
    pub binding_span: Option<Span>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AwaitClause {
    Then,
    Catch,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StartIfTag {
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElseTag {
    pub elseif: bool,
    pub expression_span: Option<Span>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeIdentifierType<'a> {
    HTMLAttribute(Span, &'a str),
    ClassDirective(Span, &'a str),
    StyleDirective(Span, &'a str),
    BindDirective(Span, &'a str),
    UseDirective(Span, &'a str),
    /// LEGACY(svelte4): on:directive
    OnDirectiveLegacy(Span, &'a str),
    /// transition:, in:, or out: directive
    TransitionDirective(Span, &'a str),
    /// animate: directive
    AnimateDirective(Span, &'a str),
    None,
}

impl<'a> AttributeIdentifierType<'a> {
    pub fn is_class_directive(name: &str) -> bool {
        name == "class"
    }

    pub fn is_style_directive(name: &str) -> bool {
        name == "style"
    }

    pub fn is_bind_directive(name: &str) -> bool {
        name == "bind"
    }

    pub fn is_use_directive(name: &str) -> bool {
        name == "use"
    }

    /// LEGACY(svelte4): on:directive
    pub fn is_on_directive(name: &str) -> bool {
        name == "on"
    }

    pub fn is_transition_directive(name: &str) -> bool {
        name == "transition" || name == "in" || name == "out"
    }

    pub fn is_animate_directive(name: &str) -> bool {
        name == "animate"
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, AttributeIdentifierType::None)
    }

    pub fn as_ok(self) -> Result<AttributeIdentifierType<'a>, Diagnostic> {
        Ok(self)
    }
}

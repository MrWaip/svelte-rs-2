use svelte_ast::TransitionDirection;
use svelte_diagnostics::Diagnostic;
use svelte_span::{GetSpan, Span};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Text,
    Comment { content_span: Span },
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
    DeclarationTag(DeclarationTagToken),
    DebugTag(DebugTagToken),
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

    pub context_deprecated: bool,
    pub invalid_context: Option<Span>,
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
    pub context_span: Option<Span>,
    pub index_span: Option<Span>,
    pub key_span: Option<Span>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HTMLAttribute {
    pub span: Span,
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

    LetDirectiveLegacy(LetDirectiveLegacy),
    UseDirective(UseDirective),

    OnDirectiveLegacy(OnDirectiveLegacy),
    TransitionDirective(TransitionDirective),
    AnimateDirective(AnimateDirective),

    AttachTag(AttachTagToken),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AnimateDirective {
    pub span: Span,

    pub name_span: Span,

    pub expression_span: Span,

    pub has_expression: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AttachTagToken {
    pub span: Span,

    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClassDirective {
    pub span: Span,
    pub shorthand: bool,
    pub name_span: Span,
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StyleDirective {
    pub span: Span,
    pub shorthand: bool,
    pub name_span: Span,
    pub value: AttributeValue,
    pub important: bool,
    pub invalid_modifier: Option<Span>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BindDirective {
    pub span: Span,
    pub shorthand: bool,
    pub name_span: Span,
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LetDirectiveLegacy {
    pub span: Span,
    pub name_span: Span,
    pub expression_span: Span,
    pub has_expression: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UseDirective {
    pub span: Span,
    pub shorthand: bool,
    pub name_span: Span,
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OnDirectiveLegacy {
    pub span: Span,

    pub name_span: Span,

    pub expression_span: Span,

    pub modifiers: Vec<Span>,

    pub has_expression: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TransitionDirective {
    pub span: Span,

    pub name_span: Span,

    pub expression_span: Span,

    pub modifiers: Vec<Span>,

    pub has_expression: bool,

    pub direction: TransitionDirection,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeValue {
    String(Span),
    ExpressionTag(ExpressionTag),
    Concatenation(Concatenation),
    Empty,
}

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

impl Attribute {
    pub fn with_span(mut self, span: Span) -> Self {
        match &mut self {
            Attribute::HTMLAttribute(attr) => attr.span = span,
            Attribute::ExpressionTag(attr) => attr.span = span,
            Attribute::ClassDirective(attr) => attr.span = span,
            Attribute::StyleDirective(attr) => attr.span = span,
            Attribute::BindDirective(attr) => attr.span = span,
            Attribute::LetDirectiveLegacy(attr) => attr.span = span,
            Attribute::UseDirective(attr) => attr.span = span,
            Attribute::OnDirectiveLegacy(attr) => attr.span = span,
            Attribute::TransitionDirective(attr) => attr.span = span,
            Attribute::AnimateDirective(attr) => attr.span = span,
            Attribute::AttachTag(attr) => attr.span = span,
        }
        self
    }
}

impl GetSpan for Attribute {
    fn span(&self) -> Span {
        match self {
            Attribute::HTMLAttribute(attr) => attr.span,
            Attribute::ExpressionTag(attr) => attr.span,
            Attribute::ClassDirective(attr) => attr.span,
            Attribute::StyleDirective(attr) => attr.span,
            Attribute::BindDirective(attr) => attr.span,
            Attribute::LetDirectiveLegacy(attr) => attr.span,
            Attribute::UseDirective(attr) => attr.span,
            Attribute::OnDirectiveLegacy(attr) => attr.span,
            Attribute::TransitionDirective(attr) => attr.span,
            Attribute::AnimateDirective(attr) => attr.span,
            Attribute::AttachTag(attr) => attr.span,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StyleTag {
    pub content_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartSnippetTag {
    pub expression_span: Span,
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
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationTagToken {
    pub statement_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DebugTagToken {
    pub identifiers: Vec<Span>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartKeyTag {
    pub expression_span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartAwaitTag {
    pub expression_span: Span,

    pub value_span: Option<Span>,

    pub error_span: Option<Span>,

    pub initial_clause: AwaitInitialClause,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AwaitInitialClause {
    Pending,

    Then,

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

    LetDirectiveLegacy(Span, &'a str),
    UseDirective(Span, &'a str),

    OnDirectiveLegacy(Span, &'a str),

    TransitionDirective(Span, &'a str),

    AnimateDirective(Span, &'a str),
    None,
}

impl<'a> AttributeIdentifierType<'a> {
    pub fn classify_directive(
        name: &'a str,
        value_span: Span,
        value: &'a str,
    ) -> Option<AttributeIdentifierType<'a>> {
        let directive = match name {
            "class" => Self::ClassDirective(value_span, value),
            "style" => Self::StyleDirective(value_span, value),
            "bind" => Self::BindDirective(value_span, value),
            "let" => Self::LetDirectiveLegacy(value_span, value),
            "use" => Self::UseDirective(value_span, value),
            "on" => Self::OnDirectiveLegacy(value_span, value),
            "transition" | "in" | "out" => Self::TransitionDirective(value_span, name),
            "animate" => Self::AnimateDirective(value_span, value),
            _ => return None,
        };
        Some(directive)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, AttributeIdentifierType::None)
    }

    pub fn as_ok(self) -> Result<AttributeIdentifierType<'a>, Diagnostic> {
        Ok(self)
    }
}

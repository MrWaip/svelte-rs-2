pub mod codes;
pub mod extract_svelte_ignore;

use std::fmt;

use svelte_span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub enum DiagnosticKind {
    // -----------------------------------------------------------------------
    // Parser errors
    // -----------------------------------------------------------------------
    UnexpectedEndOfFile,
    InvalidTagName,
    UnterminatedStartTag,
    InvalidAttributeName,
    UnexpectedToken,
    UnexpectedKeyword,
    NoElementToClose,
    UnclosedNode,
    InvalidExpression,
    NoIfBlockToClose,
    NoIfBlockForElse,
    OnlyOneTopLevelScript,
    OnlyOneTopLevelStyle,
    UnknownDirective,
    NoEachBlockToClose,
    NoKeyBlockToClose,
    VoidElementInvalidContent,
    // svelte:options errors
    SvelteOptionsUnknownAttribute(String),
    SvelteOptionsInvalidAttributeValue(String),
    SvelteOptionsInvalidCustomElementTag,
    SvelteOptionsReservedTagName,
    SvelteOptionsNoChildren,
    SvelteOptionsInvalidAttribute,
    SvelteOptionsDuplicate,
    /// LEGACY(svelte4): `tag` attribute renamed to `customElement`.
    SvelteOptionsDeprecatedTag,
    // Internal compiler errors
    InternalError(String),

    // -----------------------------------------------------------------------
    // Semantic errors (emitted during validate/analyze phase)
    // -----------------------------------------------------------------------

    // --- Options ---
    OptionsInvalidValue {
        details: String,
    },
    OptionsRemoved {
        details: String,
    },
    OptionsUnrecognised {
        keypath: String,
    },

    // --- Runes & script ---
    BindableInvalidLocation,
    ConstantAssignment {
        thing: String,
    },
    ConstantBinding {
        thing: String,
    },
    DeclarationDuplicate {
        name: String,
    },
    DeclarationDuplicateModuleImport,
    DerivedInvalidExport,
    DollarBindingInvalid,
    DollarPrefixInvalid,
    DuplicateClassField {
        name: String,
    },
    EachItemInvalidAssignment,
    EffectInvalidPlacement,
    ExperimentalAsync,
    ExportUndefined {
        name: String,
    },
    GlobalReferenceInvalid {
        name: String,
    },
    HostInvalidPlacement,
    ImportSvelteInternalForbidden,
    InspectTraceGenerator,
    InspectTraceInvalidPlacement,
    InvalidArgumentsUsage,
    LegacyAwaitInvalid,
    LegacyExportInvalid,
    LegacyPropsInvalid,
    LegacyReactiveStatementInvalid,
    LegacyRestPropsInvalid,
    ModuleIllegalDefaultExport,
    PropsDuplicate {
        rune: String,
    },
    PropsIdInvalidPlacement,
    PropsIllegalName,
    PropsInvalidIdentifier,
    PropsInvalidPattern,
    PropsInvalidPlacement,
    ReactiveDeclarationCycle {
        cycle: String,
    },
    RuneInvalidArguments {
        rune: String,
    },
    RuneInvalidArgumentsLength {
        rune: String,
        args: String,
    },
    RuneInvalidComputedProperty,
    RuneInvalidName {
        name: String,
    },
    RuneInvalidSpread {
        rune: String,
    },
    RuneInvalidUsage {
        rune: String,
    },
    RuneMissingParentheses,
    RuneRemoved {
        name: String,
    },
    RuneRenamed {
        name: String,
        replacement: String,
    },
    RunesModeInvalidImport {
        name: String,
    },
    SnippetInvalidExport,
    SnippetParameterAssignment,
    StateFieldDuplicate {
        name: String,
    },
    StateFieldInvalidAssignment,
    StateInvalidExport,
    StateInvalidPlacement {
        rune: String,
    },
    StoreInvalidScopedSubscription,
    StoreInvalidSubscription,
    StoreInvalidSubscriptionModule,
    TypescriptInvalidFeature {
        feature: String,
    },

    // --- CSS errors ---
    CssEmptyDeclaration,
    CssExpectedIdentifier,
    CssExpectedToken {
        token: String,
    },
    CssUnclosedBlock,
    CssGlobalBlockInvalidCombinator {
        name: String,
    },
    CssGlobalBlockInvalidDeclaration,
    CssGlobalBlockInvalidList,
    CssGlobalBlockInvalidModifier,
    CssGlobalBlockInvalidModifierStart,
    CssGlobalBlockInvalidPlacement,
    CssGlobalInvalidPlacement,
    CssGlobalInvalidSelector,
    CssGlobalInvalidSelectorList,
    CssNestingSelectorInvalidPlacement,
    CssSelectorInvalid,
    CssTypeSelectorInvalidPlacement,

    // --- Template / element errors ---
    AnimationDuplicate,
    AnimationInvalidPlacement,
    AnimationMissingKey,
    AttributeContenteditableDynamic,
    AttributeContenteditableMissing,
    AttributeDuplicate,
    AttributeEmptyShorthand,
    AttributeInvalidEventHandler,
    AttributeInvalidMultiple,
    AttributeInvalidName {
        name: String,
    },
    AttributeInvalidSequenceExpression,
    AttributeInvalidType,
    AttributeUnquotedSequence,
    BindGroupInvalidExpression,
    BindGroupInvalidSnippetParameter,
    BindInvalidExpression,
    BindInvalidName {
        name: String,
        explanation: Option<String>,
    },
    BindInvalidParens {
        name: String,
    },
    BindInvalidTarget {
        name: String,
        elements: String,
    },
    BindInvalidValue,
    BlockDuplicateClause {
        name: String,
    },
    BlockInvalidContinuationPlacement,
    BlockInvalidElseif,
    BlockInvalidPlacement {
        name: String,
        location: String,
    },
    BlockUnclosed,
    BlockUnexpectedCharacter {
        character: String,
    },
    BlockUnexpectedClose,
    ComponentInvalidDirective,
    ConstTagCycle {
        cycle: String,
    },
    ConstTagInvalidExpression,
    ConstTagInvalidPlacement,
    ConstTagInvalidReference {
        name: String,
    },
    DebugTagInvalidArguments,
    DirectiveInvalidValue,
    DirectiveMissingName {
        type_: String,
    },
    EachKeyWithoutAs,
    ElementInvalidClosingTag {
        name: String,
    },
    ElementInvalidClosingTagAutoclosed {
        name: String,
        reason: String,
    },
    ElementUnclosed {
        name: String,
    },
    EventHandlerInvalidComponentModifier,
    EventHandlerInvalidModifier {
        list: String,
    },
    EventHandlerInvalidModifierCombination {
        modifier1: String,
        modifier2: String,
    },
    ExpectedAttributeValue,
    ExpectedBlockType,
    ExpectedIdentifier,
    ExpectedPattern,
    ExpectedTag,
    ExpectedToken {
        token: String,
    },
    ExpectedWhitespace,
    IllegalAwaitExpression,
    IllegalElementAttribute {
        name: String,
    },
    JsParseError {
        message: String,
    },
    LetDirectiveInvalidPlacement,
    MixedEventHandlerSyntaxes {
        name: String,
    },
    NodeInvalidPlacement {
        message: String,
    },
    RenderTagInvalidCallExpression,
    RenderTagInvalidExpression,
    RenderTagInvalidSpreadArgument,
    ScriptDuplicate,
    ScriptInvalidAttributeValue {
        name: String,
    },
    ScriptInvalidContext,
    ScriptReservedAttribute {
        name: String,
    },
    SlotAttributeDuplicate {
        name: String,
        component: String,
    },
    SlotAttributeInvalid,
    SlotAttributeInvalidPlacement,
    SlotDefaultDuplicate,
    SlotElementInvalidAttribute,
    SlotElementInvalidName,
    SlotElementInvalidNameDefault,
    SlotSnippetConflict,
    SnippetConflict,
    SnippetInvalidRestParameter,
    SnippetShadowingProp {
        prop: String,
    },
    StyleDirectiveInvalidModifier,
    StyleDuplicate,
    SvelteBodyIllegalAttribute,
    SvelteBoundaryInvalidAttribute,
    SvelteBoundaryInvalidAttributeValue,
    SvelteComponentInvalidThis,
    SvelteComponentMissingThis,
    SvelteElementMissingThis,
    SvelteFragmentInvalidAttribute,
    SvelteFragmentInvalidPlacement,
    SvelteHeadIllegalAttribute,
    SvelteMetaDuplicate {
        name: String,
    },
    SvelteMetaInvalidContent {
        name: String,
    },
    SvelteMetaInvalidPlacement {
        name: String,
    },
    SvelteMetaInvalidTag {
        list: String,
    },
    SvelteSelfInvalidPlacement,
    TagInvalidPlacement {
        name: String,
        location: String,
    },
    TextareaInvalidContent,
    TitleIllegalAttribute,
    TitleInvalidContent,
    TransitionConflict {
        type_: String,
        existing: String,
    },
    TransitionDuplicate {
        type_: String,
    },
    UnterminatedStringConstant,

    // -----------------------------------------------------------------------
    // A11y warnings
    // -----------------------------------------------------------------------
    A11yAccesskey,
    A11yAriaActivedescendantHasTabindex,
    A11yAriaAttributes {
        name: String,
    },
    A11yAutocompleteValid {
        value: String,
        type_: String,
    },
    A11yAutofocus,
    A11yClickEventsHaveKeyEvents,
    A11yConsiderExplicitLabel,
    A11yDistractingElements {
        name: String,
    },
    A11yFigcaptionIndex,
    A11yFigcaptionParent,
    A11yHidden {
        name: String,
    },
    A11yImgRedundantAlt,
    A11yIncorrectAriaAttributeType {
        attribute: String,
        type_: String,
    },
    A11yIncorrectAriaAttributeTypeBoolean {
        attribute: String,
    },
    A11yIncorrectAriaAttributeTypeId {
        attribute: String,
    },
    A11yIncorrectAriaAttributeTypeIdlist {
        attribute: String,
    },
    A11yIncorrectAriaAttributeTypeInteger {
        attribute: String,
    },
    A11yIncorrectAriaAttributeTypeToken {
        attribute: String,
        values: String,
    },
    A11yIncorrectAriaAttributeTypeTokenlist {
        attribute: String,
        values: String,
    },
    A11yIncorrectAriaAttributeTypeTristate {
        attribute: String,
    },
    A11yInteractiveSupportsFocus {
        role: String,
    },
    A11yInvalidAttribute {
        href_value: String,
        href_attribute: String,
    },
    A11yLabelHasAssociatedControl,
    A11yMediaHasCaption,
    A11yMisplacedRole {
        name: String,
    },
    A11yMisplacedScope,
    A11yMissingAttribute {
        name: String,
        article: String,
        sequence: String,
    },
    A11yMissingContent {
        name: String,
    },
    A11yMouseEventsHaveKeyEvents {
        event: String,
        accompanied_by: String,
    },
    A11yNoAbstractRole {
        role: String,
    },
    A11yNoInteractiveElementToNoninteractiveRole {
        element: String,
        role: String,
    },
    A11yNoNoninteractiveElementInteractions {
        element: String,
    },
    A11yNoNoninteractiveElementToInteractiveRole {
        element: String,
        role: String,
    },
    A11yNoNoninteractiveTabindex,
    A11yNoRedundantRoles {
        role: String,
    },
    A11yNoStaticElementInteractions {
        element: String,
        handler: String,
    },
    A11yPositiveTabindex,
    A11yRoleHasRequiredAriaProps {
        role: String,
        props: String,
    },
    A11yRoleSupportsAriaProps {
        attribute: String,
        role: String,
    },
    A11yRoleSupportsAriaPropsImplicit {
        attribute: String,
        role: String,
        name: String,
    },
    A11yUnknownAriaAttribute {
        attribute: String,
        suggestion: Option<String>,
    },
    A11yUnknownRole {
        role: String,
        suggestion: Option<String>,
    },

    // -----------------------------------------------------------------------
    // Metadata warnings
    // -----------------------------------------------------------------------
    BidirectionalControlCharacters,
    LegacyCode {
        code: String,
        suggestion: String,
    },
    UnknownCode {
        code: String,
        suggestion: Option<String>,
    },

    // -----------------------------------------------------------------------
    // Options warnings
    // -----------------------------------------------------------------------
    OptionsDeprecatedAccessors,
    OptionsDeprecatedImmutable,
    OptionsMissingCustomElement,
    OptionsRemovedEnableSourcemap,
    OptionsRemovedHydratable,
    OptionsRemovedLoopGuardTimeout,
    OptionsRenamedSsrDom,

    // -----------------------------------------------------------------------
    // Component warnings
    // -----------------------------------------------------------------------
    CustomElementPropsIdentifier,
    ExportLetUnused {
        name: String,
    },
    LegacyComponentCreation,
    NonReactiveUpdate {
        name: String,
    },
    PerfAvoidInlineClass,
    PerfAvoidNestedClass,
    ReactiveDeclarationInvalidPlacement,
    ReactiveDeclarationModuleScriptDependency,
    StateReferencedLocally {
        name: String,
        type_: String,
    },
    StoreRuneConflict {
        name: String,
    },

    // -----------------------------------------------------------------------
    // CSS warnings
    // -----------------------------------------------------------------------
    CssUnusedSelector {
        name: String,
    },

    // -----------------------------------------------------------------------
    // Attribute / element warnings
    // -----------------------------------------------------------------------
    AttributeAvoidIs,
    AttributeGlobalEventReference {
        name: String,
    },
    AttributeIllegalColon,
    AttributeInvalidPropertyName {
        wrong: String,
        right: String,
    },
    AttributeQuoted,
    BindInvalidEachRest {
        name: String,
    },
    BlockEmpty,
    ComponentNameLowercase {
        name: String,
    },
    ElementImplicitlyClosed {
        tag: String,
        closing: String,
    },
    ElementInvalidSelfClosingTag {
        name: String,
    },
    EventDirectiveDeprecated {
        name: String,
    },
    NodeInvalidPlacementSsr {
        message: String,
    },
    ScriptContextDeprecated,
    ScriptUnknownAttribute,
    SlotElementDeprecated,
    SvelteComponentDeprecated,
    SvelteElementInvalidThis,
    SvelteSelfDeprecated {
        name: String,
        basename: String,
    },
}

impl DiagnosticKind {
    /// Returns the snake_case error/warning code for this diagnostic.
    /// Matches official Svelte codes.
    pub fn code(&self) -> &'static str {
        match self {
            // Parser errors
            Self::UnexpectedEndOfFile => "unexpected_eof",
            Self::InvalidTagName => "tag_invalid_name",
            Self::UnterminatedStartTag => "unterminated_start_tag",
            Self::InvalidAttributeName => "attribute_invalid_name",
            Self::UnexpectedToken => "unexpected_token",
            Self::UnexpectedKeyword => "unexpected_reserved_word",
            Self::NoElementToClose => "element_invalid_closing_tag",
            Self::UnclosedNode => "element_unclosed",
            Self::InvalidExpression => "invalid_expression",
            Self::NoIfBlockToClose => "block_unexpected_close",
            Self::NoIfBlockForElse => "block_unexpected_close",
            Self::OnlyOneTopLevelScript => "script_duplicate",
            Self::OnlyOneTopLevelStyle => "style_duplicate",
            Self::UnknownDirective => "unknown_directive",
            Self::NoEachBlockToClose => "block_unexpected_close",
            Self::NoKeyBlockToClose => "block_unexpected_close",
            Self::VoidElementInvalidContent => "void_element_invalid_content",
            Self::SvelteOptionsUnknownAttribute(_) => "svelte_options_unknown_attribute",
            Self::SvelteOptionsInvalidAttributeValue(_) => "svelte_options_invalid_attribute_value",
            Self::SvelteOptionsInvalidCustomElementTag => "svelte_options_invalid_customelement",
            Self::SvelteOptionsReservedTagName => "svelte_options_reserved_tagname",
            Self::SvelteOptionsNoChildren => "svelte_options_children_forbidden",
            Self::SvelteOptionsInvalidAttribute => "svelte_options_invalid_attribute",
            Self::SvelteOptionsDuplicate => "svelte_options_duplicate",
            Self::SvelteOptionsDeprecatedTag => "svelte_options_deprecated_tag",
            Self::InternalError(_) => "internal_error",

            // Semantic errors
            Self::OptionsInvalidValue { .. } => "options_invalid_value",
            Self::OptionsRemoved { .. } => "options_removed",
            Self::OptionsUnrecognised { .. } => "options_unrecognised",
            Self::BindableInvalidLocation => "bindable_invalid_location",
            Self::ConstantAssignment { .. } => "constant_assignment",
            Self::ConstantBinding { .. } => "constant_binding",
            Self::DeclarationDuplicate { .. } => "declaration_duplicate",
            Self::DeclarationDuplicateModuleImport => "declaration_duplicate_module_import",
            Self::DerivedInvalidExport => "derived_invalid_export",
            Self::DollarBindingInvalid => "dollar_binding_invalid",
            Self::DollarPrefixInvalid => "dollar_prefix_invalid",
            Self::DuplicateClassField { .. } => "duplicate_class_field",
            Self::EachItemInvalidAssignment => "each_item_invalid_assignment",
            Self::EffectInvalidPlacement => "effect_invalid_placement",
            Self::ExperimentalAsync => "experimental_async",
            Self::ExportUndefined { .. } => "export_undefined",
            Self::GlobalReferenceInvalid { .. } => "global_reference_invalid",
            Self::HostInvalidPlacement => "host_invalid_placement",
            Self::ImportSvelteInternalForbidden => "import_svelte_internal_forbidden",
            Self::InspectTraceGenerator => "inspect_trace_generator",
            Self::InspectTraceInvalidPlacement => "inspect_trace_invalid_placement",
            Self::InvalidArgumentsUsage => "invalid_arguments_usage",
            Self::LegacyAwaitInvalid => "legacy_await_invalid",
            Self::LegacyExportInvalid => "legacy_export_invalid",
            Self::LegacyPropsInvalid => "legacy_props_invalid",
            Self::LegacyReactiveStatementInvalid => "legacy_reactive_statement_invalid",
            Self::LegacyRestPropsInvalid => "legacy_rest_props_invalid",
            Self::ModuleIllegalDefaultExport => "module_illegal_default_export",
            Self::PropsDuplicate { .. } => "props_duplicate",
            Self::PropsIdInvalidPlacement => "props_id_invalid_placement",
            Self::PropsIllegalName => "props_illegal_name",
            Self::PropsInvalidIdentifier => "props_invalid_identifier",
            Self::PropsInvalidPattern => "props_invalid_pattern",
            Self::PropsInvalidPlacement => "props_invalid_placement",
            Self::ReactiveDeclarationCycle { .. } => "reactive_declaration_cycle",
            Self::RuneInvalidArguments { .. } => "rune_invalid_arguments",
            Self::RuneInvalidArgumentsLength { .. } => "rune_invalid_arguments_length",
            Self::RuneInvalidComputedProperty => "rune_invalid_computed_property",
            Self::RuneInvalidName { .. } => "rune_invalid_name",
            Self::RuneInvalidSpread { .. } => "rune_invalid_spread",
            Self::RuneInvalidUsage { .. } => "rune_invalid_usage",
            Self::RuneMissingParentheses => "rune_missing_parentheses",
            Self::RuneRemoved { .. } => "rune_removed",
            Self::RuneRenamed { .. } => "rune_renamed",
            Self::RunesModeInvalidImport { .. } => "runes_mode_invalid_import",
            Self::SnippetInvalidExport => "snippet_invalid_export",
            Self::SnippetParameterAssignment => "snippet_parameter_assignment",
            Self::StateFieldDuplicate { .. } => "state_field_duplicate",
            Self::StateFieldInvalidAssignment => "state_field_invalid_assignment",
            Self::StateInvalidExport => "state_invalid_export",
            Self::StateInvalidPlacement { .. } => "state_invalid_placement",
            Self::StoreInvalidScopedSubscription => "store_invalid_scoped_subscription",
            Self::StoreInvalidSubscription => "store_invalid_subscription",
            Self::StoreInvalidSubscriptionModule => "store_invalid_subscription_module",
            Self::TypescriptInvalidFeature { .. } => "typescript_invalid_feature",
            Self::CssEmptyDeclaration => "css_empty_declaration",
            Self::CssExpectedIdentifier => "css_expected_identifier",
            Self::CssExpectedToken { .. } => "css_expected_token",
            Self::CssUnclosedBlock => "css_unclosed_block",
            Self::CssGlobalBlockInvalidCombinator { .. } => "css_global_block_invalid_combinator",
            Self::CssGlobalBlockInvalidDeclaration => "css_global_block_invalid_declaration",
            Self::CssGlobalBlockInvalidList => "css_global_block_invalid_list",
            Self::CssGlobalBlockInvalidModifier => "css_global_block_invalid_modifier",
            Self::CssGlobalBlockInvalidModifierStart => "css_global_block_invalid_modifier_start",
            Self::CssGlobalBlockInvalidPlacement => "css_global_block_invalid_placement",
            Self::CssGlobalInvalidPlacement => "css_global_invalid_placement",
            Self::CssGlobalInvalidSelector => "css_global_invalid_selector",
            Self::CssGlobalInvalidSelectorList => "css_global_invalid_selector_list",
            Self::CssNestingSelectorInvalidPlacement => "css_nesting_selector_invalid_placement",
            Self::CssSelectorInvalid => "css_selector_invalid",
            Self::CssTypeSelectorInvalidPlacement => "css_type_selector_invalid_placement",
            Self::AnimationDuplicate => "animation_duplicate",
            Self::AnimationInvalidPlacement => "animation_invalid_placement",
            Self::AnimationMissingKey => "animation_missing_key",
            Self::AttributeContenteditableDynamic => "attribute_contenteditable_dynamic",
            Self::AttributeContenteditableMissing => "attribute_contenteditable_missing",
            Self::AttributeDuplicate => "attribute_duplicate",
            Self::AttributeEmptyShorthand => "attribute_empty_shorthand",
            Self::AttributeInvalidEventHandler => "attribute_invalid_event_handler",
            Self::AttributeInvalidMultiple => "attribute_invalid_multiple",
            Self::AttributeInvalidName { .. } => "attribute_invalid_name",
            Self::AttributeInvalidSequenceExpression => "attribute_invalid_sequence_expression",
            Self::AttributeInvalidType => "attribute_invalid_type",
            Self::AttributeUnquotedSequence => "attribute_unquoted_sequence",
            Self::BindGroupInvalidExpression => "bind_group_invalid_expression",
            Self::BindGroupInvalidSnippetParameter => "bind_group_invalid_snippet_parameter",
            Self::BindInvalidExpression => "bind_invalid_expression",
            Self::BindInvalidName { .. } => "bind_invalid_name",
            Self::BindInvalidParens { .. } => "bind_invalid_parens",
            Self::BindInvalidTarget { .. } => "bind_invalid_target",
            Self::BindInvalidValue => "bind_invalid_value",
            Self::BlockDuplicateClause { .. } => "block_duplicate_clause",
            Self::BlockInvalidContinuationPlacement => "block_invalid_continuation_placement",
            Self::BlockInvalidElseif => "block_invalid_elseif",
            Self::BlockInvalidPlacement { .. } => "block_invalid_placement",
            Self::BlockUnclosed => "block_unclosed",
            Self::BlockUnexpectedCharacter { .. } => "block_unexpected_character",
            Self::BlockUnexpectedClose => "block_unexpected_close",
            Self::ComponentInvalidDirective => "component_invalid_directive",
            Self::ConstTagCycle { .. } => "const_tag_cycle",
            Self::ConstTagInvalidExpression => "const_tag_invalid_expression",
            Self::ConstTagInvalidPlacement => "const_tag_invalid_placement",
            Self::ConstTagInvalidReference { .. } => "const_tag_invalid_reference",
            Self::DebugTagInvalidArguments => "debug_tag_invalid_arguments",
            Self::DirectiveInvalidValue => "directive_invalid_value",
            Self::DirectiveMissingName { .. } => "directive_missing_name",
            Self::EachKeyWithoutAs => "each_key_without_as",
            Self::ElementInvalidClosingTag { .. } => "element_invalid_closing_tag",
            Self::ElementInvalidClosingTagAutoclosed { .. } => {
                "element_invalid_closing_tag_autoclosed"
            }
            Self::ElementUnclosed { .. } => "element_unclosed",
            Self::EventHandlerInvalidComponentModifier => {
                "event_handler_invalid_component_modifier"
            }
            Self::EventHandlerInvalidModifier { .. } => "event_handler_invalid_modifier",
            Self::EventHandlerInvalidModifierCombination { .. } => {
                "event_handler_invalid_modifier_combination"
            }
            Self::ExpectedAttributeValue => "expected_attribute_value",
            Self::ExpectedBlockType => "expected_block_type",
            Self::ExpectedIdentifier => "expected_identifier",
            Self::ExpectedPattern => "expected_pattern",
            Self::ExpectedTag => "expected_tag",
            Self::ExpectedToken { .. } => "expected_token",
            Self::ExpectedWhitespace => "expected_whitespace",
            Self::IllegalAwaitExpression => "illegal_await_expression",
            Self::IllegalElementAttribute { .. } => "illegal_element_attribute",
            Self::JsParseError { .. } => "js_parse_error",
            Self::LetDirectiveInvalidPlacement => "let_directive_invalid_placement",
            Self::MixedEventHandlerSyntaxes { .. } => "mixed_event_handler_syntaxes",
            Self::NodeInvalidPlacement { .. } => "node_invalid_placement",
            Self::RenderTagInvalidCallExpression => "render_tag_invalid_call_expression",
            Self::RenderTagInvalidExpression => "render_tag_invalid_expression",
            Self::RenderTagInvalidSpreadArgument => "render_tag_invalid_spread_argument",
            Self::ScriptDuplicate => "script_duplicate",
            Self::ScriptInvalidAttributeValue { .. } => "script_invalid_attribute_value",
            Self::ScriptInvalidContext => "script_invalid_context",
            Self::ScriptReservedAttribute { .. } => "script_reserved_attribute",
            Self::SlotAttributeDuplicate { .. } => "slot_attribute_duplicate",
            Self::SlotAttributeInvalid => "slot_attribute_invalid",
            Self::SlotAttributeInvalidPlacement => "slot_attribute_invalid_placement",
            Self::SlotDefaultDuplicate => "slot_default_duplicate",
            Self::SlotElementInvalidAttribute => "slot_element_invalid_attribute",
            Self::SlotElementInvalidName => "slot_element_invalid_name",
            Self::SlotElementInvalidNameDefault => "slot_element_invalid_name_default",
            Self::SlotSnippetConflict => "slot_snippet_conflict",
            Self::SnippetConflict => "snippet_conflict",
            Self::SnippetInvalidRestParameter => "snippet_invalid_rest_parameter",
            Self::SnippetShadowingProp { .. } => "snippet_shadowing_prop",
            Self::StyleDirectiveInvalidModifier => "style_directive_invalid_modifier",
            Self::StyleDuplicate => "style_duplicate",
            Self::SvelteBodyIllegalAttribute => "svelte_body_illegal_attribute",
            Self::SvelteBoundaryInvalidAttribute => "svelte_boundary_invalid_attribute",
            Self::SvelteBoundaryInvalidAttributeValue => "svelte_boundary_invalid_attribute_value",
            Self::SvelteComponentInvalidThis => "svelte_component_invalid_this",
            Self::SvelteComponentMissingThis => "svelte_component_missing_this",
            Self::SvelteElementMissingThis => "svelte_element_missing_this",
            Self::SvelteFragmentInvalidAttribute => "svelte_fragment_invalid_attribute",
            Self::SvelteFragmentInvalidPlacement => "svelte_fragment_invalid_placement",
            Self::SvelteHeadIllegalAttribute => "svelte_head_illegal_attribute",
            Self::SvelteMetaDuplicate { .. } => "svelte_meta_duplicate",
            Self::SvelteMetaInvalidContent { .. } => "svelte_meta_invalid_content",
            Self::SvelteMetaInvalidPlacement { .. } => "svelte_meta_invalid_placement",
            Self::SvelteMetaInvalidTag { .. } => "svelte_meta_invalid_tag",
            Self::SvelteSelfInvalidPlacement => "svelte_self_invalid_placement",
            Self::TagInvalidPlacement { .. } => "tag_invalid_placement",
            Self::TextareaInvalidContent => "textarea_invalid_content",
            Self::TitleIllegalAttribute => "title_illegal_attribute",
            Self::TitleInvalidContent => "title_invalid_content",
            Self::TransitionConflict { .. } => "transition_conflict",
            Self::TransitionDuplicate { .. } => "transition_duplicate",
            Self::UnterminatedStringConstant => "unterminated_string_constant",

            // A11y warnings
            Self::A11yAccesskey => "a11y_accesskey",
            Self::A11yAriaActivedescendantHasTabindex => "a11y_aria_activedescendant_has_tabindex",
            Self::A11yAriaAttributes { .. } => "a11y_aria_attributes",
            Self::A11yAutocompleteValid { .. } => "a11y_autocomplete_valid",
            Self::A11yAutofocus => "a11y_autofocus",
            Self::A11yClickEventsHaveKeyEvents => "a11y_click_events_have_key_events",
            Self::A11yConsiderExplicitLabel => "a11y_consider_explicit_label",
            Self::A11yDistractingElements { .. } => "a11y_distracting_elements",
            Self::A11yFigcaptionIndex => "a11y_figcaption_index",
            Self::A11yFigcaptionParent => "a11y_figcaption_parent",
            Self::A11yHidden { .. } => "a11y_hidden",
            Self::A11yImgRedundantAlt => "a11y_img_redundant_alt",
            Self::A11yIncorrectAriaAttributeType { .. } => "a11y_incorrect_aria_attribute_type",
            Self::A11yIncorrectAriaAttributeTypeBoolean { .. } => {
                "a11y_incorrect_aria_attribute_type_boolean"
            }
            Self::A11yIncorrectAriaAttributeTypeId { .. } => {
                "a11y_incorrect_aria_attribute_type_id"
            }
            Self::A11yIncorrectAriaAttributeTypeIdlist { .. } => {
                "a11y_incorrect_aria_attribute_type_idlist"
            }
            Self::A11yIncorrectAriaAttributeTypeInteger { .. } => {
                "a11y_incorrect_aria_attribute_type_integer"
            }
            Self::A11yIncorrectAriaAttributeTypeToken { .. } => {
                "a11y_incorrect_aria_attribute_type_token"
            }
            Self::A11yIncorrectAriaAttributeTypeTokenlist { .. } => {
                "a11y_incorrect_aria_attribute_type_tokenlist"
            }
            Self::A11yIncorrectAriaAttributeTypeTristate { .. } => {
                "a11y_incorrect_aria_attribute_type_tristate"
            }
            Self::A11yInteractiveSupportsFocus { .. } => "a11y_interactive_supports_focus",
            Self::A11yInvalidAttribute { .. } => "a11y_invalid_attribute",
            Self::A11yLabelHasAssociatedControl => "a11y_label_has_associated_control",
            Self::A11yMediaHasCaption => "a11y_media_has_caption",
            Self::A11yMisplacedRole { .. } => "a11y_misplaced_role",
            Self::A11yMisplacedScope => "a11y_misplaced_scope",
            Self::A11yMissingAttribute { .. } => "a11y_missing_attribute",
            Self::A11yMissingContent { .. } => "a11y_missing_content",
            Self::A11yMouseEventsHaveKeyEvents { .. } => "a11y_mouse_events_have_key_events",
            Self::A11yNoAbstractRole { .. } => "a11y_no_abstract_role",
            Self::A11yNoInteractiveElementToNoninteractiveRole { .. } => {
                "a11y_no_interactive_element_to_noninteractive_role"
            }
            Self::A11yNoNoninteractiveElementInteractions { .. } => {
                "a11y_no_noninteractive_element_interactions"
            }
            Self::A11yNoNoninteractiveElementToInteractiveRole { .. } => {
                "a11y_no_noninteractive_element_to_interactive_role"
            }
            Self::A11yNoNoninteractiveTabindex => "a11y_no_noninteractive_tabindex",
            Self::A11yNoRedundantRoles { .. } => "a11y_no_redundant_roles",
            Self::A11yNoStaticElementInteractions { .. } => "a11y_no_static_element_interactions",
            Self::A11yPositiveTabindex => "a11y_positive_tabindex",
            Self::A11yRoleHasRequiredAriaProps { .. } => "a11y_role_has_required_aria_props",
            Self::A11yRoleSupportsAriaProps { .. } => "a11y_role_supports_aria_props",
            Self::A11yRoleSupportsAriaPropsImplicit { .. } => {
                "a11y_role_supports_aria_props_implicit"
            }
            Self::A11yUnknownAriaAttribute { .. } => "a11y_unknown_aria_attribute",
            Self::A11yUnknownRole { .. } => "a11y_unknown_role",

            // Metadata warnings
            Self::BidirectionalControlCharacters => "bidirectional_control_characters",
            Self::LegacyCode { .. } => "legacy_code",
            Self::UnknownCode { .. } => "unknown_code",

            // Options warnings
            Self::OptionsDeprecatedAccessors => "options_deprecated_accessors",
            Self::OptionsDeprecatedImmutable => "options_deprecated_immutable",
            Self::OptionsMissingCustomElement => "options_missing_custom_element",
            Self::OptionsRemovedEnableSourcemap => "options_removed_enable_sourcemap",
            Self::OptionsRemovedHydratable => "options_removed_hydratable",
            Self::OptionsRemovedLoopGuardTimeout => "options_removed_loop_guard_timeout",
            Self::OptionsRenamedSsrDom => "options_renamed_ssr_dom",

            // Component warnings
            Self::CustomElementPropsIdentifier => "custom_element_props_identifier",
            Self::ExportLetUnused { .. } => "export_let_unused",
            Self::LegacyComponentCreation => "legacy_component_creation",
            Self::NonReactiveUpdate { .. } => "non_reactive_update",
            Self::PerfAvoidInlineClass => "perf_avoid_inline_class",
            Self::PerfAvoidNestedClass => "perf_avoid_nested_class",
            Self::ReactiveDeclarationInvalidPlacement => "reactive_declaration_invalid_placement",
            Self::ReactiveDeclarationModuleScriptDependency => {
                "reactive_declaration_module_script_dependency"
            }
            Self::StateReferencedLocally { .. } => "state_referenced_locally",
            Self::StoreRuneConflict { .. } => "store_rune_conflict",

            // CSS warnings
            Self::CssUnusedSelector { .. } => "css_unused_selector",

            // Attribute / element warnings
            Self::AttributeAvoidIs => "attribute_avoid_is",
            Self::AttributeGlobalEventReference { .. } => "attribute_global_event_reference",
            Self::AttributeIllegalColon => "attribute_illegal_colon",
            Self::AttributeInvalidPropertyName { .. } => "attribute_invalid_property_name",
            Self::AttributeQuoted => "attribute_quoted",
            Self::BindInvalidEachRest { .. } => "bind_invalid_each_rest",
            Self::BlockEmpty => "block_empty",
            Self::ComponentNameLowercase { .. } => "component_name_lowercase",
            Self::ElementImplicitlyClosed { .. } => "element_implicitly_closed",
            Self::ElementInvalidSelfClosingTag { .. } => "element_invalid_self_closing_tag",
            Self::EventDirectiveDeprecated { .. } => "event_directive_deprecated",
            Self::NodeInvalidPlacementSsr { .. } => "node_invalid_placement_ssr",
            Self::ScriptContextDeprecated => "script_context_deprecated",
            Self::ScriptUnknownAttribute => "script_unknown_attribute",
            Self::SlotElementDeprecated => "slot_element_deprecated",
            Self::SvelteComponentDeprecated => "svelte_component_deprecated",
            Self::SvelteElementInvalidThis => "svelte_element_invalid_this",
            Self::SvelteSelfDeprecated { .. } => "svelte_self_deprecated",
        }
    }

    /// Returns a human-readable message.
    pub fn message(&self) -> String {
        match self {
            // Parser errors
            Self::UnexpectedEndOfFile => "Unexpected end of input".into(),
            Self::InvalidTagName => "Expected a valid element or component name".into(),
            Self::UnterminatedStartTag => "Start tag is not terminated".into(),
            Self::InvalidAttributeName => "Invalid attribute name".into(),
            Self::UnexpectedToken => "Unexpected token".into(),
            Self::UnexpectedKeyword => "Unexpected reserved word".into(),
            Self::NoElementToClose => "Attempted to close an element that was not open".into(),
            Self::UnclosedNode => "Element was left open".into(),
            Self::InvalidExpression => "Invalid expression".into(),
            Self::NoIfBlockToClose => "Unexpected {/if} \u{2014} there is no matching {#if}".into(),
            Self::NoIfBlockForElse => "Unexpected {:else} \u{2014} there is no matching {#if}".into(),
            Self::OnlyOneTopLevelScript => "A component can have a single top-level <script> element".into(),
            Self::OnlyOneTopLevelStyle => "A component can have a single top-level <style> element".into(),
            Self::UnknownDirective => "Unknown directive".into(),
            Self::NoEachBlockToClose => "Unexpected {/each} \u{2014} there is no matching {#each}".into(),
            Self::NoKeyBlockToClose => "Unexpected {/key} \u{2014} there is no matching {#key}".into(),
            Self::VoidElementInvalidContent => "Void elements cannot have children or closing tags".into(),
            Self::SvelteOptionsUnknownAttribute(name) => format!("<svelte:options> unknown attribute '{name}'"),
            Self::SvelteOptionsInvalidAttributeValue(expected) => format!("Value must be {expected}"),
            Self::SvelteOptionsInvalidCustomElementTag => "\"tag\" must be a valid custom element name".into(),
            Self::SvelteOptionsReservedTagName => "\"tag\" cannot be a reserved custom element name".into(),
            Self::SvelteOptionsNoChildren => "<svelte:options> cannot have children".into(),
            Self::SvelteOptionsInvalidAttribute => "<svelte:options> can only have static attributes".into(),
            Self::SvelteOptionsDuplicate => "A component can have a single <svelte:options> element".into(),
            Self::SvelteOptionsDeprecatedTag => "\"tag\" option is deprecated \u{2014} use \"customElement\" instead".into(),
            Self::InternalError(msg) => format!("Internal compiler error: {msg}"),

            // Semantic errors
            Self::OptionsInvalidValue { details } => format!("Invalid compiler option: {details}"),
            Self::OptionsRemoved { details } => format!("Invalid compiler option: {details}"),
            Self::OptionsUnrecognised { keypath } => format!("Unrecognised compiler option {keypath}"),
            Self::BindableInvalidLocation => "`$bindable()` can only be used inside a `$props()` declaration".into(),
            Self::ConstantAssignment { thing } => format!("Cannot assign to {thing}"),
            Self::ConstantBinding { thing } => format!("Cannot bind to {thing}"),
            Self::DeclarationDuplicate { name } => format!("`{name}` has already been declared"),
            Self::DeclarationDuplicateModuleImport => "Cannot declare a variable with the same name as an import inside `<script module>`".into(),
            Self::DerivedInvalidExport => "Cannot export derived state from a module. To expose the current derived value, export a function returning its value".into(),
            Self::DollarBindingInvalid => "The $ name is reserved, and cannot be used for variables and imports".into(),
            Self::DollarPrefixInvalid => "The $ prefix is reserved, and cannot be used for variables and imports".into(),
            Self::DuplicateClassField { name } => format!("`{name}` has already been declared"),
            Self::EachItemInvalidAssignment => "Cannot reassign or bind to each block argument in runes mode. Use the array and index variables instead (e.g. `array[i] = value` instead of `entry = value`, or `bind:value={array[i]}` instead of `bind:value={entry}`)".into(),
            Self::EffectInvalidPlacement => "`$effect()` can only be used as an expression statement".into(),
            Self::ExperimentalAsync => "Cannot use `await` in deriveds and template expressions, or at the top level of a component, unless the `experimental.async` compiler option is `true`".into(),
            Self::ExportUndefined { name } => format!("`{name}` is not defined"),
            Self::GlobalReferenceInvalid { name } => format!("`{name}` is an illegal variable name. To reference a global variable called `{name}`, use `globalThis.{name}`"),
            Self::HostInvalidPlacement => "`$host()` can only be used inside custom element component instances".into(),
            Self::ImportSvelteInternalForbidden => "Imports of `svelte/internal/*` are forbidden. It contains private runtime code which is subject to change without notice. If you're importing from `svelte/internal/*` to work around a limitation of Svelte, please open an issue at https://github.com/sveltejs/svelte and explain your use case".into(),
            Self::InspectTraceGenerator => "`$inspect.trace(...)` cannot be used inside a generator function".into(),
            Self::InspectTraceInvalidPlacement => "`$inspect.trace(...)` must be the first statement of a function body".into(),
            Self::InvalidArgumentsUsage => "The arguments keyword cannot be used within the template or at the top level of a component".into(),
            Self::LegacyAwaitInvalid => "Cannot use `await` in deriveds and template expressions, or at the top level of a component, unless in runes mode".into(),
            Self::LegacyExportInvalid => "Cannot use `export let` in runes mode \u{2014} use `$props()` instead".into(),
            Self::LegacyPropsInvalid => "Cannot use `$$props` in runes mode".into(),
            Self::LegacyReactiveStatementInvalid => "`$:` is not allowed in runes mode, use `$derived` or `$effect` instead".into(),
            Self::LegacyRestPropsInvalid => "Cannot use `$$restProps` in runes mode".into(),
            Self::ModuleIllegalDefaultExport => "A component cannot have a default export".into(),
            Self::PropsDuplicate { rune } => format!("Cannot use `{rune}()` more than once"),
            Self::PropsIdInvalidPlacement => "`$props.id()` can only be used at the top level of components as a variable declaration initializer".into(),
            Self::PropsIllegalName => "Declaring or accessing a prop starting with `$$` is illegal (they are reserved for Svelte internals)".into(),
            Self::PropsInvalidIdentifier => "`$props()` can only be used with an object destructuring pattern".into(),
            Self::PropsInvalidPattern => "`$props()` assignment must not contain nested properties or computed keys".into(),
            Self::PropsInvalidPlacement => "`$props()` can only be used at the top level of components as a variable declaration initializer".into(),
            Self::ReactiveDeclarationCycle { cycle } => format!("Cyclical dependency detected: {cycle}"),
            Self::RuneInvalidArguments { rune } => format!("`{rune}` cannot be called with arguments"),
            Self::RuneInvalidArgumentsLength { rune, args } => format!("`{rune}` must be called with {args}"),
            Self::RuneInvalidComputedProperty => "Cannot access a computed property of a rune".into(),
            Self::RuneInvalidName { name } => format!("`{name}` is not a valid rune"),
            Self::RuneInvalidSpread { rune } => format!("`{rune}` cannot be called with a spread argument"),
            Self::RuneInvalidUsage { rune } => format!("Cannot use `{rune}` rune in non-runes mode"),
            Self::RuneMissingParentheses => "Cannot use rune without parentheses".into(),
            Self::RuneRemoved { name } => format!("The `{name}` rune has been removed"),
            Self::RuneRenamed { name, replacement } => format!("`{name}` is now `{replacement}`"),
            Self::RunesModeInvalidImport { name } => format!("{name} cannot be used in runes mode"),
            Self::SnippetInvalidExport => "An exported snippet can only reference things declared in a `<script module>`, or other exportable snippets".into(),
            Self::SnippetParameterAssignment => "Cannot reassign or bind to snippet parameter".into(),
            Self::StateFieldDuplicate { name } => format!("`{name}` has already been declared on this class"),
            Self::StateFieldInvalidAssignment => "Cannot assign to a state field before its declaration".into(),
            Self::StateInvalidExport => "Cannot export state from a module if it is reassigned. Either export a function returning the state value or only mutate the state value's properties".into(),
            Self::StateInvalidPlacement { rune } => format!("`{rune}(...)` can only be used as a variable declaration initializer, a class field declaration, or the first assignment to a class field at the top level of the constructor."),
            Self::StoreInvalidScopedSubscription => "Cannot subscribe to stores that are not declared at the top level of the component".into(),
            Self::StoreInvalidSubscription => "Cannot reference store value inside `<script module>`".into(),
            Self::StoreInvalidSubscriptionModule => "Cannot reference store value outside a `.svelte` file".into(),
            Self::TypescriptInvalidFeature { feature } => format!("TypeScript language features like {feature} are not natively supported, and their use is generally discouraged. Outside of `<script>` tags, these features are not supported. For use within `<script>` tags, you will need to use a preprocessor to convert it to JavaScript before it gets passed to the Svelte compiler. If you are using `vitePreprocess`, make sure to specifically enable preprocessing script tags (`vitePreprocess({{ script: true }})`)"),
            Self::CssEmptyDeclaration => "Declaration cannot be empty".into(),
            Self::CssExpectedIdentifier => "Expected a valid CSS identifier".into(),
            Self::CssExpectedToken { token } => format!("Expected `{token}`"),
            Self::CssUnclosedBlock => "Unclosed block".into(),
            Self::CssGlobalBlockInvalidCombinator { name } => format!("A `:global` selector cannot follow a `{name}` combinator"),
            Self::CssGlobalBlockInvalidDeclaration => "A top-level `:global {{...}}` block can only contain rules, not declarations".into(),
            Self::CssGlobalBlockInvalidList => "A `:global` selector cannot be part of a selector list with entries that don't contain `:global`".into(),
            Self::CssGlobalBlockInvalidModifier => "A `:global` selector cannot modify an existing selector".into(),
            Self::CssGlobalBlockInvalidModifierStart => "A `:global` selector can only be modified if it is a descendant of other selectors".into(),
            Self::CssGlobalBlockInvalidPlacement => "A `:global` selector cannot be inside a pseudoclass".into(),
            Self::CssGlobalInvalidPlacement => "`:global(...)` can be at the start or end of a selector sequence, but not in the middle".into(),
            Self::CssGlobalInvalidSelector => "`:global(...)` must contain exactly one selector".into(),
            Self::CssGlobalInvalidSelectorList => "`:global(...)` must not contain type or universal selectors when used in a compound selector".into(),
            Self::CssNestingSelectorInvalidPlacement => "Nesting selectors can only be used inside a rule or as the first selector inside a lone `:global(...)`".into(),
            Self::CssSelectorInvalid => "Invalid selector".into(),
            Self::CssTypeSelectorInvalidPlacement => "`:global(...)` must not be followed by a type selector".into(),
            Self::AnimationDuplicate => "An element can only have one 'animate' directive".into(),
            Self::AnimationInvalidPlacement => "An element that uses the `animate:` directive must be the only child of a keyed `{#each ...}` block".into(),
            Self::AnimationMissingKey => "An element that uses the `animate:` directive must be the only child of a keyed `{#each ...}` block. Did you forget to add a key to your each block?".into(),
            Self::AttributeContenteditableDynamic => "'contenteditable' attribute cannot be dynamic if element uses two-way binding".into(),
            Self::AttributeContenteditableMissing => "'contenteditable' attribute is required for textContent, innerHTML and innerText two-way bindings".into(),
            Self::AttributeDuplicate => "Attributes need to be unique".into(),
            Self::AttributeEmptyShorthand => "Attribute shorthand cannot be empty".into(),
            Self::AttributeInvalidEventHandler => "Event attribute must be a JavaScript expression, not a string".into(),
            Self::AttributeInvalidMultiple => "'multiple' attribute must be static if select uses two-way binding".into(),
            Self::AttributeInvalidName { name } => format!("'{name}' is not a valid attribute name"),
            Self::AttributeInvalidSequenceExpression => "Comma-separated expressions are not allowed as attribute/directive values in runes mode, unless wrapped in parentheses".into(),
            Self::AttributeInvalidType => "'type' attribute must be a static text value if input uses two-way binding".into(),
            Self::AttributeUnquotedSequence => "Attribute values containing `{...}` must be enclosed in quote marks, unless the value only contains the expression".into(),
            Self::BindGroupInvalidExpression => "`bind:group` can only bind to an Identifier or MemberExpression".into(),
            Self::BindGroupInvalidSnippetParameter => "Cannot `bind:group` to a snippet parameter".into(),
            Self::BindInvalidExpression => "Can only bind to an Identifier or MemberExpression or a `{get, set}` pair".into(),
            Self::BindInvalidName { name, explanation } => match explanation {
                Some(e) => format!("`bind:{name}` is not a valid binding. {e}"),
                None => format!("`bind:{name}` is not a valid binding"),
            },
            Self::BindInvalidParens { name } => format!("`bind:{name}={{get, set}}` must not have surrounding parentheses"),
            Self::BindInvalidTarget { name, elements } => format!("`bind:{name}` can only be used with {elements}"),
            Self::BindInvalidValue => "Can only bind to state or props".into(),
            Self::BlockDuplicateClause { name } => format!("{name} cannot appear more than once within a block"),
            Self::BlockInvalidContinuationPlacement => "{{:...}} block is invalid at this position (did you forget to close the preceding element or block?)".into(),
            Self::BlockInvalidElseif => "'elseif' should be 'else if'".into(),
            Self::BlockInvalidPlacement { name, location } => format!("{{#{name} ...}} block cannot be {location}"),
            Self::BlockUnclosed => "Block was left open".into(),
            Self::BlockUnexpectedCharacter { character } => format!("Expected a `{character}` character immediately following the opening bracket"),
            Self::BlockUnexpectedClose => "Unexpected block closing tag".into(),
            Self::ComponentInvalidDirective => "This type of directive is not valid on components".into(),
            Self::ConstTagCycle { cycle } => format!("Cyclical dependency detected: {cycle}"),
            Self::ConstTagInvalidExpression => "{{@const ...}} must consist of a single variable declaration".into(),
            Self::ConstTagInvalidPlacement => "`{@const}` must be the immediate child of `{#snippet}`, `{#if}`, `{:else if}`, `{:else}`, `{#each}`, `{:then}`, `{:catch}`, `<svelte:fragment>`, `<svelte:boundary>` or `<Component>`".into(),
            Self::ConstTagInvalidReference { name } => format!("The `{{@const {name} = ...}}` declaration is not available in this snippet"),
            Self::DebugTagInvalidArguments => "{{@debug ...}} arguments must be identifiers, not arbitrary expressions".into(),
            Self::DirectiveInvalidValue => "Directive value must be a JavaScript expression enclosed in curly braces".into(),
            Self::DirectiveMissingName { type_ } => format!("`{type_}` name cannot be empty"),
            Self::EachKeyWithoutAs => "An `{#each ...}` block without an `as` clause cannot have a key".into(),
            Self::ElementInvalidClosingTag { name } => format!("`</{name}>` attempted to close an element that was not open"),
            Self::ElementInvalidClosingTagAutoclosed { name, reason } => format!("`</{name}>` attempted to close element that was already automatically closed by `<{reason}>` (cannot nest `<{reason}>` inside `<{name}>`)"),
            Self::ElementUnclosed { name } => format!("`<{name}>` was left open"),
            Self::EventHandlerInvalidComponentModifier => "Event modifiers other than 'once' can only be used on DOM elements".into(),
            Self::EventHandlerInvalidModifier { list } => format!("Valid event modifiers are {list}"),
            Self::EventHandlerInvalidModifierCombination { modifier1, modifier2 } => format!("The '{modifier1}' and '{modifier2}' modifiers cannot be used together"),
            Self::ExpectedAttributeValue => "Expected attribute value".into(),
            Self::ExpectedBlockType => "Expected 'if', 'each', 'await', 'key' or 'snippet'".into(),
            Self::ExpectedIdentifier => "Expected an identifier".into(),
            Self::ExpectedPattern => "Expected identifier or destructure pattern".into(),
            Self::ExpectedTag => "Expected 'html', 'render', 'attach', 'const', or 'debug'".into(),
            Self::ExpectedToken { token } => format!("Expected token {token}"),
            Self::ExpectedWhitespace => "Expected whitespace".into(),
            Self::IllegalAwaitExpression => "`use:`, `transition:` and `animate:` directives, attachments and bindings do not support await expressions".into(),
            Self::IllegalElementAttribute { name } => format!("`<{name}>` does not support non-event attributes or spread attributes"),
            Self::JsParseError { message } => message.clone(),
            Self::LetDirectiveInvalidPlacement => "`let:` directive at invalid position".into(),
            Self::MixedEventHandlerSyntaxes { name } => format!("Mixing old (on:{name}) and new syntaxes for event handling is not allowed. Use only the on{name} syntax"),
            Self::NodeInvalidPlacement { message } => format!("{message}. The browser will 'repair' the HTML (by moving, removing, or inserting elements) which breaks Svelte's assumptions about the structure of your components."),
            Self::RenderTagInvalidCallExpression => "Calling a snippet function using apply, bind or call is not allowed".into(),
            Self::RenderTagInvalidExpression => "`{@render ...}` tags can only contain call expressions".into(),
            Self::RenderTagInvalidSpreadArgument => "cannot use spread arguments in `{@render ...}` tags".into(),
            Self::ScriptDuplicate => "A component can have a single top-level `<script>` element and/or a single top-level `<script module>` element".into(),
            Self::ScriptInvalidAttributeValue { name } => format!("If the `{name}` attribute is supplied, it must be a boolean attribute"),
            Self::ScriptInvalidContext => "If the context attribute is supplied, its value must be \"module\"".into(),
            Self::ScriptReservedAttribute { name } => format!("The `{name}` attribute is reserved and cannot be used"),
            Self::SlotAttributeDuplicate { name, component } => format!("Duplicate slot name '{name}' in <{component}>"),
            Self::SlotAttributeInvalid => "slot attribute must be a static value".into(),
            Self::SlotAttributeInvalidPlacement => "Element with a slot='...' attribute must be a child of a component or a descendant of a custom element".into(),
            Self::SlotDefaultDuplicate => "Found default slot content alongside an explicit slot=\"default\"".into(),
            Self::SlotElementInvalidAttribute => "`<slot>` can only receive attributes and (optionally) let directives".into(),
            Self::SlotElementInvalidName => "slot attribute must be a static value".into(),
            Self::SlotElementInvalidNameDefault => "`default` is a reserved word \u{2014} it cannot be used as a slot name".into(),
            Self::SlotSnippetConflict => "Cannot use `<slot>` syntax and `{@render ...}` tags in the same component. Migrate towards `{@render ...}` tags completely".into(),
            Self::SnippetConflict => "Cannot use explicit children snippet at the same time as implicit children content. Remove either the non-whitespace content or the children snippet block".into(),
            Self::SnippetInvalidRestParameter => "Snippets do not support rest parameters; use an array instead".into(),
            Self::SnippetShadowingProp { prop } => format!("This snippet is shadowing the prop `{prop}` with the same name"),
            Self::StyleDirectiveInvalidModifier => "`style:` directive can only use the `important` modifier".into(),
            Self::StyleDuplicate => "A component can have a single top-level `<style>` element".into(),
            Self::SvelteBodyIllegalAttribute => "`<svelte:body>` does not support non-event attributes or spread attributes".into(),
            Self::SvelteBoundaryInvalidAttribute => "Valid attributes on `<svelte:boundary>` are `onerror` and `failed`".into(),
            Self::SvelteBoundaryInvalidAttributeValue => "Attribute value must be a non-string expression".into(),
            Self::SvelteComponentInvalidThis => "Invalid component definition \u{2014} must be an `{expression}`".into(),
            Self::SvelteComponentMissingThis => "`<svelte:component>` must have a 'this' attribute".into(),
            Self::SvelteElementMissingThis => "`<svelte:element>` must have a 'this' attribute with a value".into(),
            Self::SvelteFragmentInvalidAttribute => "`<svelte:fragment>` can only have a slot attribute and (optionally) a let: directive".into(),
            Self::SvelteFragmentInvalidPlacement => "`<svelte:fragment>` must be the direct child of a component".into(),
            Self::SvelteHeadIllegalAttribute => "`<svelte:head>` cannot have attributes nor directives".into(),
            Self::SvelteMetaDuplicate { name } => format!("A component can only have one `<{name}>` element"),
            Self::SvelteMetaInvalidContent { name } => format!("<{name}> cannot have children"),
            Self::SvelteMetaInvalidPlacement { name } => format!("`<{name}>` tags cannot be inside elements or blocks"),
            Self::SvelteMetaInvalidTag { list } => format!("Valid `<svelte:...>` tag names are {list}"),
            Self::SvelteSelfInvalidPlacement => "`<svelte:self>` components can only exist inside `{#if}` blocks, `{#each}` blocks, `{#snippet}` blocks or slots passed to components".into(),
            Self::TagInvalidPlacement { name, location } => format!("{{@{name} ...}} tag cannot be {location}"),
            Self::TextareaInvalidContent => "A `<textarea>` can have either a value attribute or (equivalently) child content, but not both".into(),
            Self::TitleIllegalAttribute => "`<title>` cannot have attributes nor directives".into(),
            Self::TitleInvalidContent => "`<title>` can only contain text and {{tags}}".into(),
            Self::TransitionConflict { type_, existing } => format!("Cannot use `{type_}:` alongside existing `{existing}:` directive"),
            Self::TransitionDuplicate { type_ } => format!("Cannot use multiple `{type_}:` directives on a single element"),
            Self::UnterminatedStringConstant => "Unterminated string constant".into(),

            // A11y warnings
            Self::A11yAccesskey => "Avoid using accesskey".into(),
            Self::A11yAriaActivedescendantHasTabindex => "An element with an aria-activedescendant attribute should have a tabindex value".into(),
            Self::A11yAriaAttributes { name } => format!("`<{name}>` should not have aria-* attributes"),
            Self::A11yAutocompleteValid { value, type_ } => format!("'{value}' is an invalid value for 'autocomplete' on `<input type=\"{type_}\">`"),
            Self::A11yAutofocus => "Avoid using autofocus".into(),
            Self::A11yClickEventsHaveKeyEvents => "Visible, non-interactive elements with a click event must be accompanied by a keyboard event handler. Consider whether an interactive element such as `<button type=\"button\">` or `<a>` might be more appropriate".into(),
            Self::A11yConsiderExplicitLabel => "Buttons and links should either contain text or have an `aria-label`, `aria-labelledby` or `title` attribute".into(),
            Self::A11yDistractingElements { name } => format!("Avoid `<{name}>` elements"),
            Self::A11yFigcaptionIndex => "`<figcaption>` must be first or last child of `<figure>`".into(),
            Self::A11yFigcaptionParent => "`<figcaption>` must be an immediate child of `<figure>`".into(),
            Self::A11yHidden { name } => format!("`<{name}>` element should not be hidden"),
            Self::A11yImgRedundantAlt => "Screenreaders already announce `<img>` elements as an image".into(),
            Self::A11yIncorrectAriaAttributeType { attribute, type_ } => format!("The value of '{attribute}' must be a {type_}"),
            Self::A11yIncorrectAriaAttributeTypeBoolean { attribute } => format!("The value of '{attribute}' must be either 'true' or 'false'. It cannot be empty"),
            Self::A11yIncorrectAriaAttributeTypeId { attribute } => format!("The value of '{attribute}' must be a string that represents a DOM element ID"),
            Self::A11yIncorrectAriaAttributeTypeIdlist { attribute } => format!("The value of '{attribute}' must be a space-separated list of strings that represent DOM element IDs"),
            Self::A11yIncorrectAriaAttributeTypeInteger { attribute } => format!("The value of '{attribute}' must be an integer"),
            Self::A11yIncorrectAriaAttributeTypeToken { attribute, values } => format!("The value of '{attribute}' must be exactly one of {values}"),
            Self::A11yIncorrectAriaAttributeTypeTokenlist { attribute, values } => format!("The value of '{attribute}' must be a space-separated list of one or more of {values}"),
            Self::A11yIncorrectAriaAttributeTypeTristate { attribute } => format!("The value of '{attribute}' must be exactly one of true, false, or mixed"),
            Self::A11yInteractiveSupportsFocus { role } => format!("Elements with the '{role}' interactive role must have a tabindex value"),
            Self::A11yInvalidAttribute { href_value, href_attribute } => format!("'{href_value}' is not a valid {href_attribute} attribute"),
            Self::A11yLabelHasAssociatedControl => "A form label must be associated with a control".into(),
            Self::A11yMediaHasCaption => "`<video>` elements must have a `<track kind=\"captions\">`".into(),
            Self::A11yMisplacedRole { name } => format!("`<{name}>` should not have role attribute"),
            Self::A11yMisplacedScope => "The scope attribute should only be used with `<th>` elements".into(),
            Self::A11yMissingAttribute { name, article, sequence } => format!("`<{name}>` element should have {article} {sequence} attribute"),
            Self::A11yMissingContent { name } => format!("`<{name}>` element should contain text"),
            Self::A11yMouseEventsHaveKeyEvents { event, accompanied_by } => format!("'{event}' event must be accompanied by '{accompanied_by}' event"),
            Self::A11yNoAbstractRole { role } => format!("Abstract role '{role}' is forbidden"),
            Self::A11yNoInteractiveElementToNoninteractiveRole { element, role } => format!("`<{element}>` cannot have role '{role}'"),
            Self::A11yNoNoninteractiveElementInteractions { element } => format!("Non-interactive element `<{element}>` should not be assigned mouse or keyboard event listeners"),
            Self::A11yNoNoninteractiveElementToInteractiveRole { element, role } => format!("Non-interactive element `<{element}>` cannot have interactive role '{role}'"),
            Self::A11yNoNoninteractiveTabindex => "noninteractive element cannot have nonnegative tabIndex value".into(),
            Self::A11yNoRedundantRoles { role } => format!("Redundant role '{role}'"),
            Self::A11yNoStaticElementInteractions { element, handler } => format!("`<{element}>` with a {handler} handler must have an ARIA role"),
            Self::A11yPositiveTabindex => "Avoid tabindex values above zero".into(),
            Self::A11yRoleHasRequiredAriaProps { role, props } => format!("Elements with the ARIA role \"{role}\" must have the following attributes defined: {props}"),
            Self::A11yRoleSupportsAriaProps { attribute, role } => format!("The attribute '{attribute}' is not supported by the role '{role}'"),
            Self::A11yRoleSupportsAriaPropsImplicit { attribute, role, name } => format!("The attribute '{attribute}' is not supported by the role '{role}'. This role is implicit on the element `<{name}>`"),
            Self::A11yUnknownAriaAttribute { attribute, suggestion } => match suggestion {
                Some(s) => format!("Unknown aria attribute 'aria-{attribute}'. Did you mean '{s}'?"),
                None => format!("Unknown aria attribute 'aria-{attribute}'"),
            },
            Self::A11yUnknownRole { role, suggestion } => match suggestion {
                Some(s) => format!("Unknown role '{role}'. Did you mean '{s}'?"),
                None => format!("Unknown role '{role}'"),
            },

            // Metadata warnings
            Self::BidirectionalControlCharacters => "A bidirectional control character was detected in your code. These characters can be used to alter the visual direction of your code and could have unintended consequences".into(),
            Self::LegacyCode { code, suggestion } => format!("`{code}` is no longer valid \u{2014} please use `{suggestion}` instead"),
            Self::UnknownCode { code, suggestion } => match suggestion {
                Some(s) => format!("`{code}` is not a recognised code (did you mean `{s}`?)"),
                None => format!("`{code}` is not a recognised code"),
            },

            // Options warnings
            Self::OptionsDeprecatedAccessors => "The `accessors` option has been deprecated. It will have no effect in runes mode".into(),
            Self::OptionsDeprecatedImmutable => "The `immutable` option has been deprecated. It will have no effect in runes mode".into(),
            Self::OptionsMissingCustomElement => "The `customElement` option is used when generating a custom element. Did you forget the `customElement: true` compile option?".into(),
            Self::OptionsRemovedEnableSourcemap => "The `enableSourcemap` option has been removed. Source maps are always generated now, and tooling can choose to ignore them".into(),
            Self::OptionsRemovedHydratable => "The `hydratable` option has been removed. Svelte components are always hydratable now".into(),
            Self::OptionsRemovedLoopGuardTimeout => "The `loopGuardTimeout` option has been removed".into(),
            Self::OptionsRenamedSsrDom => "`generate: \"dom\"` and `generate: \"ssr\"` options have been renamed to \"client\" and \"server\" respectively".into(),

            // Component warnings
            Self::CustomElementPropsIdentifier => "Using a rest element or a non-destructured declaration with `$props()` means that Svelte can't infer what properties to expose when creating a custom element. Consider destructuring all the props or explicitly specifying the `customElement.props` option.".into(),
            Self::ExportLetUnused { name } => format!("Component has unused export property '{name}'. If it is for external reference only, please consider using `export const {name}`"),
            Self::LegacyComponentCreation => "Svelte 5 components are no longer classes. Instantiate them using `mount` or `hydrate` (imported from 'svelte') instead.".into(),
            Self::NonReactiveUpdate { name } => format!("`{name}` is updated, but is not declared with `$state(...)`. Changing its value will not correctly trigger updates"),
            Self::PerfAvoidInlineClass => "Avoid 'new class' \u{2014} instead, declare the class at the top level scope".into(),
            Self::PerfAvoidNestedClass => "Avoid declaring classes below the top level scope".into(),
            Self::ReactiveDeclarationInvalidPlacement => "Reactive declarations only exist at the top level of the instance script".into(),
            Self::ReactiveDeclarationModuleScriptDependency => "Reassignments of module-level declarations will not cause reactive statements to update".into(),
            Self::StateReferencedLocally { name, type_ } => format!("This reference only captures the initial value of `{name}`. Did you mean to reference it inside a {type_} instead?"),
            Self::StoreRuneConflict { name } => format!("It looks like you're using the `${name}` rune, but there is a local binding called `{name}`. Referencing a local variable with a `$` prefix will create a store subscription. Please rename `{name}` to avoid the ambiguity"),

            // CSS warnings
            Self::CssUnusedSelector { name } => format!("Unused CSS selector \"{name}\""),

            // Attribute / element warnings
            Self::AttributeAvoidIs => "The \"is\" attribute is not supported cross-browser and should be avoided".into(),
            Self::AttributeGlobalEventReference { name } => format!("You are referencing `globalThis.{name}`. Did you forget to declare a variable with that name?"),
            Self::AttributeIllegalColon => "Attributes should not contain ':' characters to prevent ambiguity with Svelte directives".into(),
            Self::AttributeInvalidPropertyName { wrong, right } => format!("'{wrong}' is not a valid HTML attribute. Did you mean '{right}'?"),
            Self::AttributeQuoted => "Quoted attributes on components and custom elements will be stringified in a future version of Svelte. If this isn't what you want, remove the quotes".into(),
            Self::BindInvalidEachRest { name } => format!("The rest operator (...) will create a new object and binding '{name}' with the original object will not work"),
            Self::BlockEmpty => "Empty block".into(),
            Self::ComponentNameLowercase { name } => format!("`<{name}>` will be treated as an HTML element unless it begins with a capital letter"),
            Self::ElementImplicitlyClosed { tag, closing } => format!("This element is implicitly closed by the following `{tag}`, which can cause an unexpected DOM structure. Add an explicit `{closing}` to avoid surprises."),
            Self::ElementInvalidSelfClosingTag { name } => format!("Self-closing HTML tags for non-void elements are ambiguous \u{2014} use `<{name} ...></{name}>` rather than `<{name} ... />`"),
            Self::EventDirectiveDeprecated { name } => format!("Using `on:{name}` to listen to the {name} event is deprecated. Use the event attribute `on{name}` instead"),
            Self::NodeInvalidPlacementSsr { message } => format!("{message}. When rendering this component on the server, the resulting HTML will be modified by the browser (by moving, removing, or inserting elements), likely resulting in a `hydration_mismatch` warning"),
            Self::ScriptContextDeprecated => "`context=\"module\"` is deprecated, use the `module` attribute instead".into(),
            Self::ScriptUnknownAttribute => "Unrecognized attribute \u{2014} should be one of `generics`, `lang` or `module`. If this exists for a preprocessor, ensure that the preprocessor removes it".into(),
            Self::SlotElementDeprecated => "Using `<slot>` to render parent content is deprecated. Use `{@render ...}` tags instead".into(),
            Self::SvelteComponentDeprecated => "`<svelte:component>` is deprecated in runes mode \u{2014} components are dynamic by default".into(),
            Self::SvelteElementInvalidThis => "`this` should be an `{expression}`. Using a string attribute value will cause an error in future versions of Svelte".into(),
            Self::SvelteSelfDeprecated { name, basename } => format!("`<svelte:self>` is deprecated \u{2014} use self-imports (e.g. `import {name} from './{basename}'`) instead"),
        }
    }

    /// Returns the severity for this diagnostic kind.
    pub fn severity(&self) -> Severity {
        match self {
            // All warning variants
            Self::A11yAccesskey
            | Self::A11yAriaActivedescendantHasTabindex
            | Self::A11yAriaAttributes { .. }
            | Self::A11yAutocompleteValid { .. }
            | Self::A11yAutofocus
            | Self::A11yClickEventsHaveKeyEvents
            | Self::A11yConsiderExplicitLabel
            | Self::A11yDistractingElements { .. }
            | Self::A11yFigcaptionIndex
            | Self::A11yFigcaptionParent
            | Self::A11yHidden { .. }
            | Self::A11yImgRedundantAlt
            | Self::A11yIncorrectAriaAttributeType { .. }
            | Self::A11yIncorrectAriaAttributeTypeBoolean { .. }
            | Self::A11yIncorrectAriaAttributeTypeId { .. }
            | Self::A11yIncorrectAriaAttributeTypeIdlist { .. }
            | Self::A11yIncorrectAriaAttributeTypeInteger { .. }
            | Self::A11yIncorrectAriaAttributeTypeToken { .. }
            | Self::A11yIncorrectAriaAttributeTypeTokenlist { .. }
            | Self::A11yIncorrectAriaAttributeTypeTristate { .. }
            | Self::A11yInteractiveSupportsFocus { .. }
            | Self::A11yInvalidAttribute { .. }
            | Self::A11yLabelHasAssociatedControl
            | Self::A11yMediaHasCaption
            | Self::A11yMisplacedRole { .. }
            | Self::A11yMisplacedScope
            | Self::A11yMissingAttribute { .. }
            | Self::A11yMissingContent { .. }
            | Self::A11yMouseEventsHaveKeyEvents { .. }
            | Self::A11yNoAbstractRole { .. }
            | Self::A11yNoInteractiveElementToNoninteractiveRole { .. }
            | Self::A11yNoNoninteractiveElementInteractions { .. }
            | Self::A11yNoNoninteractiveElementToInteractiveRole { .. }
            | Self::A11yNoNoninteractiveTabindex
            | Self::A11yNoRedundantRoles { .. }
            | Self::A11yNoStaticElementInteractions { .. }
            | Self::A11yPositiveTabindex
            | Self::A11yRoleHasRequiredAriaProps { .. }
            | Self::A11yRoleSupportsAriaProps { .. }
            | Self::A11yRoleSupportsAriaPropsImplicit { .. }
            | Self::A11yUnknownAriaAttribute { .. }
            | Self::A11yUnknownRole { .. }
            | Self::BidirectionalControlCharacters
            | Self::LegacyCode { .. }
            | Self::UnknownCode { .. }
            | Self::OptionsDeprecatedAccessors
            | Self::OptionsDeprecatedImmutable
            | Self::OptionsMissingCustomElement
            | Self::OptionsRemovedEnableSourcemap
            | Self::OptionsRemovedHydratable
            | Self::OptionsRemovedLoopGuardTimeout
            | Self::OptionsRenamedSsrDom
            | Self::CustomElementPropsIdentifier
            | Self::ExportLetUnused { .. }
            | Self::LegacyComponentCreation
            | Self::NonReactiveUpdate { .. }
            | Self::PerfAvoidInlineClass
            | Self::PerfAvoidNestedClass
            | Self::ReactiveDeclarationInvalidPlacement
            | Self::ReactiveDeclarationModuleScriptDependency
            | Self::StateReferencedLocally { .. }
            | Self::StoreRuneConflict { .. }
            | Self::CssUnusedSelector { .. }
            | Self::AttributeAvoidIs
            | Self::AttributeGlobalEventReference { .. }
            | Self::AttributeIllegalColon
            | Self::AttributeInvalidPropertyName { .. }
            | Self::AttributeQuoted
            | Self::BindInvalidEachRest { .. }
            | Self::BlockEmpty
            | Self::ComponentNameLowercase { .. }
            | Self::ElementImplicitlyClosed { .. }
            | Self::ElementInvalidSelfClosingTag { .. }
            | Self::EventDirectiveDeprecated { .. }
            | Self::NodeInvalidPlacementSsr { .. }
            | Self::ScriptContextDeprecated
            | Self::ScriptUnknownAttribute
            | Self::SlotElementDeprecated
            | Self::SvelteComponentDeprecated
            | Self::SvelteElementInvalidThis
            | Self::SvelteSelfDeprecated { .. }
            | Self::SvelteOptionsDeprecatedTag => Severity::Warning,

            // Everything else is an error
            _ => Severity::Error,
        }
    }

    /// Returns a link to the Svelte documentation for this diagnostic, if one exists.
    pub fn svelte_doc_url(&self) -> Option<String> {
        let code = self.code();
        match self {
            Self::UnexpectedToken | Self::UnknownDirective | Self::InternalError(_) => None,
            _ => Some(format!("https://svelte.dev/e/{code}")),
        }
    }

    /// Returns the complete list of valid warning codes.
    pub fn all_warning_codes() -> &'static [&'static str] {
        &[
            "a11y_accesskey",
            "a11y_aria_activedescendant_has_tabindex",
            "a11y_aria_attributes",
            "a11y_autocomplete_valid",
            "a11y_autofocus",
            "a11y_click_events_have_key_events",
            "a11y_consider_explicit_label",
            "a11y_distracting_elements",
            "a11y_figcaption_index",
            "a11y_figcaption_parent",
            "a11y_hidden",
            "a11y_img_redundant_alt",
            "a11y_incorrect_aria_attribute_type",
            "a11y_incorrect_aria_attribute_type_boolean",
            "a11y_incorrect_aria_attribute_type_id",
            "a11y_incorrect_aria_attribute_type_idlist",
            "a11y_incorrect_aria_attribute_type_integer",
            "a11y_incorrect_aria_attribute_type_token",
            "a11y_incorrect_aria_attribute_type_tokenlist",
            "a11y_incorrect_aria_attribute_type_tristate",
            "a11y_interactive_supports_focus",
            "a11y_invalid_attribute",
            "a11y_label_has_associated_control",
            "a11y_media_has_caption",
            "a11y_misplaced_role",
            "a11y_misplaced_scope",
            "a11y_missing_attribute",
            "a11y_missing_content",
            "a11y_mouse_events_have_key_events",
            "a11y_no_abstract_role",
            "a11y_no_interactive_element_to_noninteractive_role",
            "a11y_no_noninteractive_element_interactions",
            "a11y_no_noninteractive_element_to_interactive_role",
            "a11y_no_noninteractive_tabindex",
            "a11y_no_redundant_roles",
            "a11y_no_static_element_interactions",
            "a11y_positive_tabindex",
            "a11y_role_has_required_aria_props",
            "a11y_role_supports_aria_props",
            "a11y_role_supports_aria_props_implicit",
            "a11y_unknown_aria_attribute",
            "a11y_unknown_role",
            "bidirectional_control_characters",
            "legacy_code",
            "unknown_code",
            "options_deprecated_accessors",
            "options_deprecated_immutable",
            "options_missing_custom_element",
            "options_removed_enable_sourcemap",
            "options_removed_hydratable",
            "options_removed_loop_guard_timeout",
            "options_renamed_ssr_dom",
            "custom_element_props_identifier",
            "export_let_unused",
            "legacy_component_creation",
            "non_reactive_update",
            "perf_avoid_inline_class",
            "perf_avoid_nested_class",
            "reactive_declaration_invalid_placement",
            "reactive_declaration_module_script_dependency",
            "state_referenced_locally",
            "store_rune_conflict",
            "css_unused_selector",
            "attribute_avoid_is",
            "attribute_global_event_reference",
            "attribute_illegal_colon",
            "attribute_invalid_property_name",
            "attribute_quoted",
            "bind_invalid_each_rest",
            "block_empty",
            "component_name_lowercase",
            "element_implicitly_closed",
            "element_invalid_self_closing_tag",
            "event_directive_deprecated",
            "node_invalid_placement_ssr",
            "script_context_deprecated",
            "script_unknown_attribute",
            "slot_element_deprecated",
            "svelte_component_deprecated",
            "svelte_element_invalid_this",
            "svelte_self_deprecated",
        ]
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub span: Span,
    pub severity: Severity,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.message())?;
        if let Some(url) = self.kind.svelte_doc_url() {
            write!(f, "\n{url}")?;
        }
        Ok(())
    }
}

impl Diagnostic {
    pub fn error(kind: DiagnosticKind, span: Span) -> Self {
        Diagnostic {
            kind,
            span,
            severity: Severity::Error,
        }
    }

    pub fn warning(kind: DiagnosticKind, span: Span) -> Self {
        Diagnostic {
            kind,
            span,
            severity: Severity::Warning,
        }
    }

    pub fn unexpected_end_of_file(span: Span) -> Self {
        Self::error(DiagnosticKind::UnexpectedEndOfFile, span)
    }

    pub fn invalid_tag_name(span: Span) -> Self {
        Self::error(DiagnosticKind::InvalidTagName, span)
    }

    pub fn unterminated_start_tag(span: Span) -> Self {
        Self::error(DiagnosticKind::UnterminatedStartTag, span)
    }

    pub fn invalid_attribute_name(span: Span) -> Self {
        Self::error(DiagnosticKind::InvalidAttributeName, span)
    }

    pub fn unexpected_token(span: Span) -> Self {
        Self::error(DiagnosticKind::UnexpectedToken, span)
    }

    pub fn unexpected_keyword(span: Span) -> Self {
        Self::error(DiagnosticKind::UnexpectedKeyword, span)
    }

    pub fn no_element_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoElementToClose, span)
    }

    pub fn no_if_block_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoIfBlockToClose, span)
    }

    pub fn no_if_block_for_else(span: Span) -> Self {
        Self::error(DiagnosticKind::NoIfBlockForElse, span)
    }

    pub fn unclosed_node(span: Span) -> Self {
        Self::error(DiagnosticKind::UnclosedNode, span)
    }

    pub fn invalid_expression(span: Span) -> Self {
        Self::error(DiagnosticKind::InvalidExpression, span)
    }

    pub fn only_single_top_level_script(span: Span) -> Self {
        Self::error(DiagnosticKind::OnlyOneTopLevelScript, span)
    }

    pub fn only_single_top_level_style(span: Span) -> Self {
        Self::error(DiagnosticKind::OnlyOneTopLevelStyle, span)
    }

    pub fn unknown_directive(span: Span) -> Self {
        Self::error(DiagnosticKind::UnknownDirective, span)
    }

    pub fn no_each_block_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoEachBlockToClose, span)
    }

    pub fn no_key_block_to_close(span: Span) -> Self {
        Self::error(DiagnosticKind::NoKeyBlockToClose, span)
    }

    pub fn void_element_invalid_content(span: Span) -> Self {
        Self::error(DiagnosticKind::VoidElementInvalidContent, span)
    }

    pub fn svelte_options_unknown_attribute(span: Span, name: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SvelteOptionsUnknownAttribute(name),
            span,
            severity: Severity::Error,
        }
    }

    pub fn svelte_options_invalid_attribute_value(span: Span, expected: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SvelteOptionsInvalidAttributeValue(expected),
            span,
            severity: Severity::Error,
        }
    }

    pub fn svelte_options_invalid_custom_element_tag(span: Span) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SvelteOptionsInvalidCustomElementTag,
            span,
            severity: Severity::Error,
        }
    }

    pub fn svelte_options_reserved_tag_name(span: Span) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SvelteOptionsReservedTagName,
            span,
            severity: Severity::Error,
        }
    }

    pub fn svelte_options_no_children(span: Span) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SvelteOptionsNoChildren,
            span,
            severity: Severity::Error,
        }
    }

    pub fn svelte_options_invalid_attribute(span: Span) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SvelteOptionsInvalidAttribute,
            span,
            severity: Severity::Error,
        }
    }

    pub fn svelte_options_duplicate(span: Span) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SvelteOptionsDuplicate,
            span,
            severity: Severity::Error,
        }
    }

    /// LEGACY(svelte4): `tag` attribute renamed to `customElement`.
    pub fn svelte_options_deprecated_tag(span: Span) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SvelteOptionsDeprecatedTag,
            span,
            severity: Severity::Warning,
        }
    }

    pub fn internal_error(message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::InternalError(message),
            span: Span::new(0, 0),
            severity: Severity::Error,
        }
    }

    pub fn as_err<T>(self) -> Result<T, Diagnostic> {
        Err(self)
    }
}

/// Converts byte offset to (line, column) pair.
/// Lines and columns are 0-based.
pub struct LineIndex {
    line_starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }
        LineIndex { line_starts }
    }

    /// Returns (line, column) for a byte offset. Both 0-based.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let line = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        let col = offset - self.line_starts[line];
        (line, col)
    }

    /// Renders a code frame showing ±2 lines of context around the error.
    /// Returns `None` if the span is out of bounds.
    pub fn code_frame(&self, source: &str, span: Span) -> Option<String> {
        let total_lines = self.line_starts.len();
        if total_lines == 0 {
            return None;
        }

        let (error_line, error_col) = self.line_col(span.start as usize);

        let frame_start = error_line.saturating_sub(2);
        let frame_end = (error_line + 3).min(total_lines);

        let lines: Vec<&str> = source.split('\n').collect();
        if error_line >= lines.len() {
            return None;
        }

        let max_line_num = frame_end; // 1-based
        let gutter_width = max_line_num.to_string().len();

        let mut out = String::new();
        for i in frame_start..frame_end {
            if i >= lines.len() {
                break;
            }
            let line_num = i + 1; // 1-based
            let display_line = lines[i].replace('\t', "  ");

            if i == error_line {
                out.push_str(&format!(
                    "{:>width$} | {}\n",
                    line_num,
                    display_line,
                    width = gutter_width
                ));
                // Add pointer line
                let pointer_col = lines[i][..error_col.min(lines[i].len())]
                    .chars()
                    .map(|c| if c == '\t' { 2 } else { 1 })
                    .sum::<usize>();
                out.push_str(&format!(
                    "{:>width$} | {}^\n",
                    "",
                    " ".repeat(pointer_col),
                    width = gutter_width
                ));
            } else {
                out.push_str(&format!(
                    "{:>width$} | {}\n",
                    line_num,
                    display_line,
                    width = gutter_width
                ));
            }
        }

        // Remove trailing newline
        if out.ends_with('\n') {
            out.pop();
        }

        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes() {
        assert_eq!(DiagnosticKind::UnexpectedEndOfFile.code(), "unexpected_eof");
        assert_eq!(DiagnosticKind::InvalidTagName.code(), "tag_invalid_name");
        assert_eq!(
            DiagnosticKind::NoElementToClose.code(),
            "element_invalid_closing_tag"
        );
        assert_eq!(DiagnosticKind::UnclosedNode.code(), "element_unclosed");
        assert_eq!(
            DiagnosticKind::OnlyOneTopLevelScript.code(),
            "script_duplicate"
        );
        assert_eq!(
            DiagnosticKind::OnlyOneTopLevelStyle.code(),
            "style_duplicate"
        );
        assert_eq!(
            DiagnosticKind::VoidElementInvalidContent.code(),
            "void_element_invalid_content"
        );
        assert_eq!(
            DiagnosticKind::InternalError("test".into()).code(),
            "internal_error"
        );
    }

    #[test]
    fn warning_codes() {
        assert_eq!(DiagnosticKind::BlockEmpty.code(), "block_empty");
        assert_eq!(DiagnosticKind::A11yAccesskey.code(), "a11y_accesskey");
        assert_eq!(
            DiagnosticKind::A11yAriaAttributes { name: "div".into() }.code(),
            "a11y_aria_attributes"
        );
        assert_eq!(
            DiagnosticKind::CssUnusedSelector {
                name: ".foo".into()
            }
            .code(),
            "css_unused_selector"
        );
        assert_eq!(
            DiagnosticKind::EventDirectiveDeprecated {
                name: "click".into()
            }
            .code(),
            "event_directive_deprecated"
        );
    }

    #[test]
    fn warning_messages() {
        assert_eq!(DiagnosticKind::BlockEmpty.message(), "Empty block");
        assert_eq!(
            DiagnosticKind::A11yAriaAttributes { name: "div".into() }.message(),
            "`<div>` should not have aria-* attributes"
        );
        assert_eq!(
            DiagnosticKind::LegacyCode {
                code: "empty-block".into(),
                suggestion: "block_empty".into()
            }
            .message(),
            "`empty-block` is no longer valid \u{2014} please use `block_empty` instead"
        );
        assert_eq!(
            DiagnosticKind::UnknownCode {
                code: "foo".into(),
                suggestion: Some("block_empty".into())
            }
            .message(),
            "`foo` is not a recognised code (did you mean `block_empty`?)"
        );
        assert_eq!(
            DiagnosticKind::UnknownCode {
                code: "foo".into(),
                suggestion: None
            }
            .message(),
            "`foo` is not a recognised code"
        );
    }

    #[test]
    fn severity_from_kind() {
        assert_eq!(
            DiagnosticKind::UnexpectedEndOfFile.severity(),
            Severity::Error
        );
        assert_eq!(DiagnosticKind::InvalidTagName.severity(), Severity::Error);
        assert_eq!(DiagnosticKind::BlockEmpty.severity(), Severity::Warning);
        assert_eq!(DiagnosticKind::A11yAccesskey.severity(), Severity::Warning);
        assert_eq!(
            DiagnosticKind::SvelteOptionsDeprecatedTag.severity(),
            Severity::Warning
        );
    }

    #[test]
    fn all_warning_codes_complete() {
        let codes = DiagnosticKind::all_warning_codes();
        assert_eq!(codes.len(), 81);
        assert!(codes.contains(&"block_empty"));
        assert!(codes.contains(&"a11y_accesskey"));
        assert!(codes.contains(&"css_unused_selector"));
        assert!(codes.contains(&"svelte_self_deprecated"));
    }

    #[test]
    fn warning_constructor() {
        let d = Diagnostic::warning(DiagnosticKind::BlockEmpty, Span::new(10, 20));
        assert_eq!(d.severity, Severity::Warning);
        assert_eq!(d.span.start, 10);
    }

    #[test]
    fn error_messages() {
        assert_eq!(
            DiagnosticKind::UnexpectedEndOfFile.message(),
            "Unexpected end of input"
        );
        assert_eq!(
            DiagnosticKind::NoIfBlockToClose.message(),
            "Unexpected {/if} \u{2014} there is no matching {#if}"
        );
        assert_eq!(
            DiagnosticKind::InternalError("oops".into()).message(),
            "Internal compiler error: oops"
        );
    }

    #[test]
    fn svelte_doc_urls() {
        // Has Svelte doc page
        assert_eq!(
            DiagnosticKind::UnexpectedEndOfFile.svelte_doc_url(),
            Some("https://svelte.dev/e/unexpected_eof".into())
        );
        assert_eq!(
            DiagnosticKind::VoidElementInvalidContent.svelte_doc_url(),
            Some("https://svelte.dev/e/void_element_invalid_content".into())
        );
        assert_eq!(
            DiagnosticKind::BlockEmpty.svelte_doc_url(),
            Some("https://svelte.dev/e/block_empty".into())
        );

        // No Svelte doc page
        assert_eq!(DiagnosticKind::UnexpectedToken.svelte_doc_url(), None);
        assert_eq!(DiagnosticKind::UnknownDirective.svelte_doc_url(), None);
        assert_eq!(
            DiagnosticKind::InternalError("x".into()).svelte_doc_url(),
            None
        );
    }

    #[test]
    fn display_with_url() {
        let d = Diagnostic::unexpected_end_of_file(Span::new(0, 0));
        let output = format!("{d}");
        assert!(output.contains("Unexpected end of input"));
        assert!(output.contains("https://svelte.dev/e/unexpected_eof"));
    }

    #[test]
    fn display_without_url() {
        let d = Diagnostic::error(DiagnosticKind::UnexpectedToken, Span::new(0, 0));
        let output = format!("{d}");
        assert_eq!(output, "Unexpected token");
    }

    #[test]
    fn code_frame_basic() {
        let source = "line1\nline2\nline3\nline4\nline5";
        let idx = LineIndex::new(source);
        // Error at start of line3 (byte offset 12)
        let frame = idx
            .code_frame(source, Span::new(12, 17))
            .expect("code_frame returns Some for valid spans");
        assert!(frame.contains("1 | line1"));
        assert!(frame.contains("3 | line3"));
        assert!(frame.contains("5 | line5"));
        assert!(frame.contains("^"));
    }

    #[test]
    fn code_frame_first_line() {
        let source = "error_here\nline2\nline3";
        let idx = LineIndex::new(source);
        let frame = idx
            .code_frame(source, Span::new(0, 5))
            .expect("code_frame returns Some for valid spans");
        assert!(frame.contains("1 | error_here"));
        assert!(frame.contains("^"));
        assert!(frame.contains("3 | line3"));
    }

    #[test]
    fn code_frame_last_line() {
        let source = "line1\nline2\nline3\nline4\nerror_here";
        let idx = LineIndex::new(source);
        // Error at start of line5 (byte offset = 24)
        let frame = idx
            .code_frame(source, Span::new(24, 34))
            .expect("code_frame returns Some for valid spans");
        assert!(frame.contains("5 | error_here"));
        assert!(frame.contains("^"));
        assert!(frame.contains("3 | line3"));
    }

    #[test]
    fn code_frame_with_tabs() {
        let source = "\tindented";
        let idx = LineIndex::new(source);
        let frame = idx
            .code_frame(source, Span::new(1, 5))
            .expect("code_frame returns Some for valid spans");
        // Tab should be replaced with 2 spaces in display
        assert!(frame.contains("  indented"));
        // Pointer should account for tab width
        assert!(frame.contains("  ^"));
    }
}

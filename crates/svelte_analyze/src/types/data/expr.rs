use super::*;

/// Legacy catch-all bag of per-expression facts collected by
/// `collect_symbols`. Consumers that reach for it are rebuilding
/// semantic meaning from scattered booleans — exactly what the semantic
/// cluster migration aims to eliminate. The proper path for any new
/// consumer question is:
///
/// - reactive per-reference decisions → `reactivity_semantics`
/// - per-cluster higher-level answers → the owning cluster
///   (`block_semantics` / `attribute_semantics` / `element_shape_semantics`)
///
/// This type is kept only while existing call sites are migrated away.
#[deprecated(note = "ExpressionInfo is a legacy bag of facts. For new code: \
            use reactivity_semantics for per-reference decisions, or add \
            the needed higher-level answer to the owning semantic cluster \
            (block_semantics / attribute_semantics / element_shape_semantics).")]
#[derive(Debug, Clone)]
pub struct ExpressionInfo {
    kind: ExpressionKind,
    expr_role: Option<ExprRole>,
    ref_symbols: SmallVec<[SymbolId; 2]>,
    uses_legacy_slots: bool,
    has_store_ref: bool,
    has_side_effects: bool,
    has_call: bool,
    has_await: bool,
    has_state_rune: bool,
    has_store_member_mutation: bool,
    needs_context: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprRole {
    Static,
    DynamicPure,
    DynamicWithContext,
    Async,
    RenderTag,
}

impl ExpressionInfo {
    pub(crate) fn new(kind: ExpressionKind) -> Self {
        Self {
            kind,
            expr_role: None,
            ref_symbols: SmallVec::new(),
            uses_legacy_slots: false,
            has_store_ref: false,
            has_side_effects: false,
            has_call: false,
            has_await: false,
            has_state_rune: false,
            has_store_member_mutation: false,
            needs_context: false,
        }
    }

    pub(crate) fn set_initial_flags(
        &mut self,
        uses_legacy_slots: bool,
        has_store_ref: bool,
        has_side_effects: bool,
        has_call: bool,
        has_await: bool,
        has_state_rune: bool,
        has_store_member_mutation: bool,
    ) {
        self.uses_legacy_slots = uses_legacy_slots;
        self.has_store_ref = has_store_ref;
        self.has_side_effects = has_side_effects;
        self.has_call = has_call;
        self.has_await = has_await;
        self.has_state_rune = has_state_rune;
        self.has_store_member_mutation = has_store_member_mutation;
    }

    pub(crate) fn set_ref_symbols(&mut self, ref_symbols: SmallVec<[SymbolId; 2]>) {
        self.ref_symbols = ref_symbols;
    }

    pub(crate) fn set_needs_context(&mut self, needs_context: bool) {
        self.needs_context = needs_context;
        // `ClassifyNeedsContext` runs before `Dynamism`, so `ExprRole` is
        // provisionally set to `DynamicWithContext` here and may be overwritten
        // later by `set_expr_role_from_dynamism` (which preserves
        // `DynamicWithContext` when `needs_context` is set).
        if self.expr_role == Some(ExprRole::RenderTag) {
            return;
        }
        if needs_context {
            self.expr_role = Some(ExprRole::DynamicWithContext);
        }
    }

    /// Applied by the `dynamism` pass after `DynamismData` is built. `ExprRole`
    /// is deliberately non-authoritative: the bitsets on `DynamismData` are the
    /// source of truth; this mirror exists only so consumers that historically
    /// asked "what kind of template role does this expression play?" keep the
    /// same answer without re-querying the bitsets.
    pub(crate) fn set_expr_role_from_dynamism(&mut self, is_dynamic: bool) {
        if self.expr_role == Some(ExprRole::RenderTag) {
            return;
        }
        self.expr_role = Some(if self.has_await {
            ExprRole::Async
        } else if self.needs_context {
            ExprRole::DynamicWithContext
        } else if is_dynamic {
            ExprRole::DynamicPure
        } else {
            ExprRole::Static
        });
    }

    pub(crate) fn mark_render_tag(&mut self) {
        self.expr_role = Some(ExprRole::RenderTag);
    }

    pub(crate) fn merge_in(&mut self, other: &Self) {
        self.uses_legacy_slots |= other.uses_legacy_slots;
        self.has_call |= other.has_call;
        self.has_await |= other.has_await;
        self.has_store_ref |= other.has_store_ref;
        self.has_side_effects |= other.has_side_effects;
        self.has_state_rune |= other.has_state_rune;
        self.has_store_member_mutation |= other.has_store_member_mutation;
        self.needs_context |= other.needs_context;
        for &sym in &other.ref_symbols {
            if !self.ref_symbols.contains(&sym) {
                self.ref_symbols.push(sym);
            }
        }
    }

    pub fn kind(&self) -> &ExpressionKind {
        &self.kind
    }

    pub fn identifier_name(&self) -> Option<&str> {
        match &self.kind {
            ExpressionKind::Identifier(name) => Some(name.as_str()),
            _ => None,
        }
    }

    pub fn is_identifier(&self) -> bool {
        self.identifier_name().is_some()
    }

    pub fn is_identifier_or_member_expression(&self) -> bool {
        matches!(
            self.kind,
            ExpressionKind::Identifier(_) | ExpressionKind::MemberExpression
        )
    }

    pub fn has_context_sensitive_shape(&self) -> bool {
        matches!(
            self.kind,
            ExpressionKind::MemberExpression | ExpressionKind::CallExpression { .. }
        )
    }

    pub fn is_simple_shape(&self) -> bool {
        self.kind.is_simple()
    }

    pub fn expr_role(&self) -> Option<ExprRole> {
        self.expr_role
    }

    pub fn is_async_role(&self) -> bool {
        self.expr_role == Some(ExprRole::Async)
    }

    pub fn is_dynamic_with_context_role(&self) -> bool {
        self.expr_role == Some(ExprRole::DynamicWithContext)
    }

    pub fn ref_symbols(&self) -> &[SymbolId] {
        self.ref_symbols.as_slice()
    }

    pub fn uses_legacy_slots(&self) -> bool {
        self.uses_legacy_slots
    }

    pub fn has_store_ref(&self) -> bool {
        self.has_store_ref
    }

    pub fn has_side_effects(&self) -> bool {
        self.has_side_effects
    }

    pub fn has_call(&self) -> bool {
        self.has_call
    }

    pub fn has_await(&self) -> bool {
        self.has_await
    }

    pub fn has_state_rune(&self) -> bool {
        self.has_state_rune
    }

    pub fn has_store_member_mutation(&self) -> bool {
        self.has_store_member_mutation
    }

    pub fn needs_context(&self) -> bool {
        self.needs_context
    }

    pub fn needs_memoized_value(&self) -> bool {
        self.has_call || self.has_await
    }

    pub fn needs_legacy_coarse_wrap(&self) -> bool {
        self.has_call
            || matches!(
                self.kind,
                ExpressionKind::MemberExpression | ExpressionKind::Assignment
            )
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprSite {
    Node(NodeId),
    Attr(NodeId),
}

#[derive(Debug, Clone)]
pub struct ExprDeps<'a> {
    pub info: &'a ExpressionInfo,
    pub blockers: SmallVec<[u32; 2]>,
    pub needs_memo: bool,
}

impl ExprDeps<'_> {
    pub fn has_await(&self) -> bool {
        self.info.has_await()
    }

    pub fn has_call(&self) -> bool {
        self.info.has_call()
    }

    pub fn has_blockers(&self) -> bool {
        !self.blockers.is_empty()
    }
}

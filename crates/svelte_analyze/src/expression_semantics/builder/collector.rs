use crate::reactivity_semantics::data::{ReactivitySemantics, ReferenceSemantics};
use crate::types::script::RuneKind;
use crate::utils::script_info::detect_rune_from_call;
use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentTargetPropertyIdentifier, AwaitExpression, CallExpression,
    ChainElement, Expression, Function, IdentifierReference, MemberExpression,
    SimpleAssignmentTarget, SpreadElement, UpdateExpression,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_await_expression, walk_call_expression,
    walk_function, walk_member_expression, walk_simple_assignment_target,
    walk_spread_element, walk_update_expression,
};
use oxc_semantic::ScopeFlags;
use smallvec::SmallVec;
use svelte_component_semantics::{ComponentSemantics, SymbolId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TopLevelForm {
    Identifier,
    Member,
    Assignment,
    Update,
    Call,
    Other,
}

pub(super) struct ExprFacts {
    pub references: SmallVec<[SymbolId; 2]>,
    pub member_or_call_roots: SmallVec<[SymbolId; 2]>,
    pub top_member_or_call_roots: SmallVec<[SymbolId; 2]>,
    pub has_await: bool,
    pub has_call: bool,
    pub has_impure_call: bool,
    pub has_member: bool,
    pub has_state_rune: bool,
    pub has_store_ref: bool,
    pub has_store_member_mutation: bool,
    pub reads_legacy_props: bool,
    pub reads_legacy_rest_props: bool,
    pub has_legacy_props_member_root: bool,
    pub has_unsafe_member_root: bool,
    pub has_runtime_root: bool,
    pub top_level_form: TopLevelForm,
}

pub(super) fn collect<'a>(
    expr: &Expression<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
) -> ExprFacts {
    let top_level_form = top_level_form_of(expr);
    let mut visitor = Collector {
        semantics,
        reactivity,
        references: SmallVec::new(),
        member_or_call_roots: SmallVec::new(),
        top_member_or_call_roots: SmallVec::new(),
        has_await: false,
        has_call: false,
        has_impure_call: false,
        has_member: false,
        has_state_rune: false,
        has_store_ref: false,
        has_store_member_mutation: false,
        reads_legacy_props: false,
        reads_legacy_rest_props: false,
        has_legacy_props_member_root: false,
        has_unsafe_member_root: false,
        fn_depth: 0,
        in_write_position: false,
    };
    visitor.visit_expression(expr);
    ExprFacts {
        references: visitor.references,
        member_or_call_roots: visitor.member_or_call_roots,
        top_member_or_call_roots: visitor.top_member_or_call_roots,
        has_await: visitor.has_await,
        has_call: visitor.has_call,
        has_impure_call: visitor.has_impure_call,
        has_member: visitor.has_member,
        has_state_rune: visitor.has_state_rune,
        has_store_ref: visitor.has_store_ref,
        has_store_member_mutation: visitor.has_store_member_mutation,
        reads_legacy_props: visitor.reads_legacy_props,
        reads_legacy_rest_props: visitor.reads_legacy_rest_props,
        has_legacy_props_member_root: visitor.has_legacy_props_member_root,
        has_unsafe_member_root: visitor.has_unsafe_member_root,
        has_runtime_root: peeled_root_is_runtime(expr),
        top_level_form,
    }
}

fn callee_is_pure_shape(callee: &Expression<'_>) -> bool {
    let mut node = callee;
    loop {
        match node {
            Expression::Identifier(_) => return true,
            Expression::StaticMemberExpression(m) => node = &m.object,
            Expression::ComputedMemberExpression(m) => node = &m.object,
            Expression::PrivateFieldExpression(m) => node = &m.object,
            Expression::ChainExpression(c) => match &c.expression {
                ChainElement::StaticMemberExpression(m) => node = &m.object,
                ChainElement::ComputedMemberExpression(m) => node = &m.object,
                ChainElement::PrivateFieldExpression(m) => node = &m.object,
                ChainElement::CallExpression(_) => return false,
                ChainElement::TSNonNullExpression(t) => node = &t.expression,
            },
            _ => return false,
        }
    }
}

fn member_object_peels_to_identifier(expr: &Expression<'_>) -> bool {
    let mut node = expr;
    loop {
        match node {
            Expression::StaticMemberExpression(m) => node = &m.object,
            Expression::ComputedMemberExpression(m) => node = &m.object,
            Expression::PrivateFieldExpression(m) => node = &m.object,
            _ => break,
        }
    }
    matches!(node, Expression::Identifier(_))
}

fn peeled_root_is_runtime(expr: &Expression<'_>) -> bool {
    let mut node = expr;
    loop {
        match node {
            Expression::StaticMemberExpression(m) => node = &m.object,
            Expression::ComputedMemberExpression(m) => node = &m.object,
            Expression::PrivateFieldExpression(m) => node = &m.object,
            Expression::ChainExpression(c) => match &c.expression {
                ChainElement::StaticMemberExpression(m) => node = &m.object,
                ChainElement::ComputedMemberExpression(m) => node = &m.object,
                ChainElement::PrivateFieldExpression(m) => node = &m.object,
                ChainElement::CallExpression(call) => node = &call.callee,
                ChainElement::TSNonNullExpression(_) => {
                    unreachable!("TS stripped at parse")
                }
            },
            Expression::TSAsExpression(_)
            | Expression::TSSatisfiesExpression(_)
            | Expression::TSNonNullExpression(_)
            | Expression::TSTypeAssertion(_)
            | Expression::TSInstantiationExpression(_) => {
                unreachable!("TS stripped at parse")
            }
            Expression::ParenthesizedExpression(p) => node = &p.expression,
            _ => break,
        }
    }
    matches!(node, Expression::MetaProperty(_))
}

fn top_level_form_of(expr: &Expression<'_>) -> TopLevelForm {
    match expr.get_inner_expression() {
        Expression::Identifier(_) => TopLevelForm::Identifier,
        Expression::StaticMemberExpression(_)
        | Expression::ComputedMemberExpression(_)
        | Expression::PrivateFieldExpression(_) => TopLevelForm::Member,
        Expression::AssignmentExpression(_) => TopLevelForm::Assignment,
        Expression::UpdateExpression(_) => TopLevelForm::Update,
        Expression::CallExpression(_) => TopLevelForm::Call,
        Expression::ChainExpression(chain) => match &chain.expression {
            ChainElement::CallExpression(_) => TopLevelForm::Call,
            _ => TopLevelForm::Member,
        },
        _ => TopLevelForm::Other,
    }
}

struct Collector<'c, 'a> {
    semantics: &'c ComponentSemantics<'a>,
    reactivity: &'c ReactivitySemantics,
    references: SmallVec<[SymbolId; 2]>,
    member_or_call_roots: SmallVec<[SymbolId; 2]>,
    top_member_or_call_roots: SmallVec<[SymbolId; 2]>,
    has_await: bool,
    has_call: bool,
    has_impure_call: bool,
    has_member: bool,
    has_state_rune: bool,
    has_store_ref: bool,
    has_store_member_mutation: bool,
    reads_legacy_props: bool,
    reads_legacy_rest_props: bool,
    has_legacy_props_member_root: bool,
    has_unsafe_member_root: bool,
    fn_depth: u32,
    in_write_position: bool,
}

impl<'a> Collector<'_, 'a> {
    fn expression_root_sym(&self, expr: &Expression<'a>) -> Option<SymbolId> {
        let mut node = expr;
        loop {
            match node {
                Expression::StaticMemberExpression(m) => node = &m.object,
                Expression::ComputedMemberExpression(m) => node = &m.object,
                Expression::PrivateFieldExpression(m) => node = &m.object,
                Expression::ChainExpression(c) => match &c.expression {
                    ChainElement::StaticMemberExpression(m) => node = &m.object,
                    ChainElement::ComputedMemberExpression(m) => node = &m.object,
                    ChainElement::PrivateFieldExpression(m) => node = &m.object,
                    ChainElement::CallExpression(call) => node = &call.callee,
                    ChainElement::TSNonNullExpression(_) => {
                        unreachable!("TS stripped at parse")
                    }
                },
                Expression::TSAsExpression(_)
                | Expression::TSSatisfiesExpression(_)
                | Expression::TSNonNullExpression(_)
                | Expression::TSTypeAssertion(_)
                | Expression::TSInstantiationExpression(_) => {
                    unreachable!("TS stripped at parse")
                }
                Expression::ParenthesizedExpression(p) => node = &p.expression,
                _ => break,
            }
        }
        let Expression::Identifier(id) = node else {
            return None;
        };
        let ref_id = id.reference_id.get()?;
        self.semantics.get_reference(ref_id).symbol_id()
    }

    fn expression_root_is_synthetic_legacy_props(&self, expr: &Expression<'a>) -> bool {
        let mut node = expr;
        loop {
            match node {
                Expression::StaticMemberExpression(m) => node = &m.object,
                Expression::ComputedMemberExpression(m) => node = &m.object,
                Expression::PrivateFieldExpression(m) => node = &m.object,
                Expression::ChainExpression(c) => match &c.expression {
                    ChainElement::StaticMemberExpression(m) => node = &m.object,
                    ChainElement::ComputedMemberExpression(m) => node = &m.object,
                    ChainElement::PrivateFieldExpression(m) => node = &m.object,
                    ChainElement::CallExpression(call) => node = &call.callee,
                    ChainElement::TSNonNullExpression(_) => {
                        unreachable!("TS stripped at parse")
                    }
                },
                Expression::TSAsExpression(_)
                | Expression::TSSatisfiesExpression(_)
                | Expression::TSNonNullExpression(_)
                | Expression::TSTypeAssertion(_)
                | Expression::TSInstantiationExpression(_) => {
                    unreachable!("TS stripped at parse")
                }
                Expression::ParenthesizedExpression(p) => node = &p.expression,
                _ => break,
            }
        }
        let Expression::Identifier(id) = node else {
            return false;
        };
        let Some(ref_id) = id.reference_id.get() else {
            return false;
        };
        matches!(
            self.reactivity.reference_semantics(ref_id),
            ReferenceSemantics::LegacyPropsIdentifierRead
                | ReferenceSemantics::LegacyRestPropsIdentifierRead
        )
    }
}

impl<'a> Visit<'a> for Collector<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let name = ident.name.as_str();
        if name == "$$props" {
            self.reads_legacy_props = true;
        } else if name == "$$restProps" {
            self.reads_legacy_rest_props = true;
        }
        if name.starts_with('$') && name.len() > 1 && name != "$$slots" {
            self.has_store_ref = true;
        }
        self.in_write_position = false;
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        let sym = match self.reactivity.reference_semantics(ref_id) {
            ReferenceSemantics::StoreRead { symbol }
            | ReferenceSemantics::StoreWrite { symbol }
            | ReferenceSemantics::StoreUpdate { symbol } => Some(symbol),
            ReferenceSemantics::LegacyStateSubscribedRead { store_symbol, .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { store_symbol }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { store_symbol, .. }
            | ReferenceSemantics::ImportSubscribedRead { store_symbol } => Some(store_symbol),
            _ => self.semantics.get_reference(ref_id).symbol_id(),
        };
        let Some(sym) = sym else { return };
        if !self.references.contains(&sym) {
            self.references.push(sym);
        }
    }

    fn visit_simple_assignment_target(&mut self, it: &SimpleAssignmentTarget<'a>) {
        self.in_write_position = true;
        walk_simple_assignment_target(self, it);
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &AssignmentTargetPropertyIdentifier<'a>,
    ) {
        self.in_write_position = true;
        self.visit_identifier_reference(&it.binding);
        if let Some(init) = &it.init {
            self.visit_expression(init);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        let object = match expr {
            MemberExpression::StaticMemberExpression(m) => Some(&m.object),
            MemberExpression::ComputedMemberExpression(m) => Some(&m.object),
            MemberExpression::PrivateFieldExpression(m) => Some(&m.object),
        };
        if self.fn_depth == 0 {
            self.has_member = true;
        }
        if self.in_write_position && object.is_some_and(member_root_is_store) {
            self.has_store_member_mutation = true;
        }
        if let Some(obj) = object {
            if let Some(sym) = self.expression_root_sym(obj) {
                if !self.member_or_call_roots.contains(&sym) {
                    self.member_or_call_roots.push(sym);
                }
                if self.fn_depth == 0 && !self.top_member_or_call_roots.contains(&sym) {
                    self.top_member_or_call_roots.push(sym);
                }
            } else if self.expression_root_is_synthetic_legacy_props(obj) {
                self.has_legacy_props_member_root = true;
            } else if !member_object_peels_to_identifier(obj) {
                self.has_unsafe_member_root = true;
            }
        }
        self.in_write_position = false;
        walk_member_expression(self, expr);
    }

    fn visit_update_expression(&mut self, upd: &UpdateExpression<'a>) {
        self.in_write_position = true;
        walk_update_expression(self, upd);
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_await = true;
        }
        walk_await_expression(self, expr);
    }

    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_call = true;
            if !callee_is_pure_shape(&expr.callee) {
                self.has_impure_call = true;
            }
            if let Some(rune) = detect_rune_from_call(expr)
                && matches!(rune, RuneKind::EffectPending | RuneKind::StateEager)
            {
                self.has_state_rune = true;
            }
        }
        if let Some(sym) = self.expression_root_sym(&expr.callee) {
            if !self.member_or_call_roots.contains(&sym) {
                self.member_or_call_roots.push(sym);
            }
            if self.fn_depth == 0 && !self.top_member_or_call_roots.contains(&sym) {
                self.top_member_or_call_roots.push(sym);
            }
        }
        walk_call_expression(self, expr);
    }

    fn visit_spread_element(&mut self, spread: &SpreadElement<'a>) {
        if self.fn_depth == 0 {
            self.has_call = true;
            self.has_impure_call = true;
        }
        walk_spread_element(self, spread);
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &ArrowFunctionExpression<'a>,
    ) {
        self.fn_depth += 1;
        walk_arrow_function_expression(self, arrow);
        self.fn_depth -= 1;
    }

    fn visit_function(
        &mut self,
        func: &Function<'a>,
        flags: ScopeFlags,
    ) {
        self.fn_depth += 1;
        walk_function(self, func, flags);
        self.fn_depth -= 1;
    }
}

fn member_root_is_store(expr: &Expression<'_>) -> bool {
    let mut node = expr.get_inner_expression();
    loop {
        match node {
            Expression::StaticMemberExpression(m) => node = m.object.get_inner_expression(),
            Expression::ComputedMemberExpression(m) => node = m.object.get_inner_expression(),
            _ => break,
        }
    }
    if let Expression::Identifier(id) = node {
        id.name.starts_with('$') && id.name.len() > 1
    } else {
        false
    }
}

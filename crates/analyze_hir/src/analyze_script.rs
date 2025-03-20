use oxc_ast::{
    Visit,
    ast::{BindingPatternKind, Expression, VariableDeclarator},
};

use crate::HirAnalyses;

pub struct AnalyzeScript<'a> {
    pub analyses: &'a mut HirAnalyses,
}

pub enum SvelteRune {
    State,
    StateRaw,
    StateSnapshot,
    Props,
    PropsId,
    Bindable,
    Derived,
    DerivedBy,
    Effect,
    EffectPre,
    EffectTracking,
    EffectRoot,
    Inspect,
    InspectWith,
    InspectTrace,
    Host,
}

impl SvelteRune {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "$state" => Self::State.into(),
            "$state.raw" => Self::StateRaw.into(),
            "$state.snapshot" => Self::StateSnapshot.into(),
            "$props" => Self::Props.into(),
            "$props.id" => Self::PropsId.into(),
            "$bindable" => Self::Bindable.into(),
            "$derived" => Self::Derived.into(),
            "$derived.by" => Self::DerivedBy.into(),
            "$effect" => Self::Effect.into(),
            "$effect.pre" => Self::EffectPre.into(),
            "$effect.tracking" => Self::EffectTracking.into(),
            "$effect.root" => Self::EffectRoot.into(),
            "$inspect" => Self::Inspect.into(),
            "$inspect().with" => Self::InspectWith.into(),
            "$inspect.trace" => Self::InspectTrace.into(),
            "$host" => Self::Host.into(),
            _ => None,
        }
    }
}

impl<'hir> Visit<'hir> for AnalyzeScript<'hir> {
    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'hir>) {
        if let Some(Expression::CallExpression(call)) = &declarator.init {
            let rune = SvelteRune::from_str(call.callee_name().unwrap_or(""));

            if let Some(rune) = rune {
                if let BindingPatternKind::BindingIdentifier(id) = &declarator.id.kind {
                    let symbol_id = id.symbol_id();

                    self.analyses.add_rune(symbol_id, rune);
                }
            }
        }
    }
}

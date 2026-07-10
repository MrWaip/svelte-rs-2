use svelte_ast::{Attribute, Component};

use super::data::{ChildPropMode, RuntimeSemantics, RuntimeSemanticsStore};

pub(crate) fn build(component: &Component) -> RuntimeSemanticsStore {
    let mut out = RuntimeSemanticsStore::new();
    out.record(RuntimeSemantics::new(child_prop_mode(component)));
    out
}

fn child_prop_mode(component: &Component) -> ChildPropMode {
    for node in component.store.iter_nodes() {
        let Some(view) = node.as_component_like() else {
            continue;
        };
        if view.attributes.iter().any(is_inout_child_prop) {
            return ChildPropMode::InOut;
        }
    }
    ChildPropMode::In
}

fn is_inout_child_prop(attr: &Attribute) -> bool {
    matches!(attr, Attribute::BindDirective(directive) if directive.name != "this")
}

use compiler_tests::harness::{assert_compiler, assert_compiler_module};

macro_rules! compiler_module_case {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            assert_compiler_module($path);
        }
    };
}

macro_rules! compiler_case {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            assert_compiler($path);
        }
    };
    ($name:ident, $path:literal, ignore = $reason:literal) => {
        #[test]
        #[ignore = $reason]
        fn $name() {
            assert_compiler($path);
        }
    };
}

#[path = "clusters/events.rs"]
mod events;

#[path = "clusters/legacy_slots.rs"]
mod legacy_slots;

#[path = "clusters/special_element_order.rs"]
mod special_element_order;

#[path = "clusters/component_props.rs"]
mod component_props;

#[path = "clusters/attach_component.rs"]
mod attach_component;

#[path = "clusters/snippet_hoist.rs"]
mod snippet_hoist;

#[path = "clusters/snippet_placement.rs"]
mod snippet_placement;

#[path = "clusters/css_scope_svelte_fragment.rs"]
mod css_scope_svelte_fragment;

#[path = "clusters/text_reactivity.rs"]
mod text_reactivity;

#[path = "clusters/text_node_marker.rs"]
mod text_node_marker;

#[path = "clusters/style_directive.rs"]
mod style_directive;

#[path = "clusters/customizable_select.rs"]
mod customizable_select;

#[path = "clusters/special_value_attribute.rs"]
mod special_value_attribute;

#[path = "clusters/runes_state.rs"]
mod runes_state;

#[path = "clusters/runes_derived.rs"]
mod runes_derived;

#[path = "clusters/runes_props.rs"]
mod runes_props;

#[path = "clusters/legacy_state.rs"]
mod legacy_state;

#[path = "clusters/legacy_props.rs"]
mod legacy_props;

#[path = "clusters/legacy_specifier_destructure_props.rs"]
mod legacy_specifier_destructure_props;

#[path = "clusters/legacy_exports.rs"]
mod legacy_exports;

#[path = "clusters/legacy_rest_props.rs"]
mod legacy_rest_props;

#[path = "clusters/multi_declarator_split.rs"]
mod multi_declarator_split;

#[path = "clusters/legacy_coarse_deps.rs"]
mod legacy_coarse_deps;

#[path = "clusters/legacy_reactive_break.rs"]
mod legacy_reactive_break;

#[path = "clusters/legacy_reactive_import_wrap.rs"]
mod legacy_reactive_import_wrap;

#[path = "clusters/legacy_rune_named_stores.rs"]
mod legacy_rune_named_stores;

#[path = "clusters/legacy_component_marker.rs"]
mod legacy_component_marker;

#[path = "clusters/legacy_quoted_directive.rs"]
mod legacy_quoted_directive;

#[path = "clusters/svelte_element_this_tag.rs"]
mod svelte_element_this_tag;

#[path = "clusters/svelte_element_named_slot.rs"]
mod svelte_element_named_slot;

#[path = "clusters/svelte_component_this_tag.rs"]
mod svelte_component_this_tag;

#[path = "clusters/let_directive.rs"]
mod let_directive;

#[path = "clusters/stores.rs"]
mod stores;

#[path = "clusters/each.rs"]
mod each;

#[path = "clusters/each_item_writeback.rs"]
mod each_item_writeback;

#[path = "clusters/legacy_select_indirect.rs"]
mod legacy_select_indirect;

#[path = "clusters/await_block.rs"]
mod await_block;

#[path = "clusters/const_tag.rs"]
mod const_tag;

#[path = "clusters/snippets.rs"]
mod snippets;

#[path = "clusters/attribute_memo.rs"]
mod attribute_memo;

#[path = "clusters/if_else.rs"]
mod if_else;

#[path = "clusters/attribute_single_expr.rs"]
mod attribute_single_expr;

#[path = "clusters/attribute_unquoted_value.rs"]
mod attribute_unquoted_value;

#[path = "clusters/custom_element.rs"]
mod custom_element;

#[path = "clusters/attribute_autofocus.rs"]
mod attribute_autofocus;

#[path = "clusters/runes_imports.rs"]
mod runes_imports;

#[path = "clusters/needs_context.rs"]
mod needs_context;

#[path = "clusters/select_value_dispatch.rs"]
mod select_value_dispatch;

#[path = "clusters/bind_this.rs"]
mod bind_this;

#[path = "clusters/bind_group.rs"]
mod bind_group;

#[path = "clusters/bind_property.rs"]
mod bind_property;

#[path = "clusters/template_element.rs"]
mod template_element;

#[path = "clusters/textarea_content.rs"]
mod textarea_content;

#[path = "clusters/element_reset.rs"]
mod element_reset;

#[path = "clusters/closing_tag.rs"]
mod closing_tag;

#[path = "clusters/template_runes.rs"]
mod template_runes;

#[path = "clusters/boundary.rs"]
mod boundary;

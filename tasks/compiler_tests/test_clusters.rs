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

#[path = "clusters/component_props.rs"]
mod component_props;

#[path = "clusters/snippet_hoist.rs"]
mod snippet_hoist;

#[path = "clusters/css_scope_svelte_fragment.rs"]
mod css_scope_svelte_fragment;

#[path = "clusters/text_reactivity.rs"]
mod text_reactivity;

#[path = "clusters/customizable_select.rs"]
mod customizable_select;

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

#[path = "clusters/legacy_exports.rs"]
mod legacy_exports;

#[path = "clusters/legacy_rest_props.rs"]
mod legacy_rest_props;

#[path = "clusters/legacy_coarse_deps.rs"]
mod legacy_coarse_deps;

#[path = "clusters/legacy_rune_named_stores.rs"]
mod legacy_rune_named_stores;

#[path = "clusters/legacy_component_marker.rs"]
mod legacy_component_marker;

#[path = "clusters/legacy_quoted_directive.rs"]
mod legacy_quoted_directive;

#[path = "clusters/svelte_element_this_tag.rs"]
mod svelte_element_this_tag;

#[path = "clusters/let_directive.rs"]
mod let_directive;

#[path = "clusters/stores.rs"]
mod stores;

#[path = "clusters/each.rs"]
mod each;

#[path = "clusters/each_item_writeback.rs"]
mod each_item_writeback;

#[path = "clusters/await_block.rs"]
mod await_block;

#[path = "clusters/const_tag.rs"]
mod const_tag;

#[path = "clusters/snippets.rs"]
mod snippets;

#[path = "clusters/attribute_memo.rs"]
mod attribute_memo;

#[path = "clusters/runes_imports.rs"]
mod runes_imports;

#[path = "clusters/needs_context.rs"]
mod needs_context;

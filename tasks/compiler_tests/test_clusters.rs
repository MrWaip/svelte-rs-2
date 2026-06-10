use compiler_tests::harness::assert_compiler;

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

#[path = "clusters/legacy_coarse_deps.rs"]
mod legacy_coarse_deps;

#[path = "clusters/legacy_rune_named_stores.rs"]
mod legacy_rune_named_stores;

#[path = "clusters/legacy_component_marker.rs"]
mod legacy_component_marker;

#[path = "clusters/let_directive.rs"]
mod let_directive;

#[path = "clusters/stores.rs"]
mod stores;

#[path = "clusters/each.rs"]
mod each;

#[path = "clusters/await_block.rs"]
mod await_block;

#[path = "clusters/const_tag.rs"]
mod const_tag;

#[path = "clusters/snippets.rs"]
mod snippets;

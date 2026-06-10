use std::{
    fs::{File, read_to_string},
    io::Write,
};

use compiler_tests::cases::{load_v3_case, load_v3_module_case, v3_case_dir};
use compiler_tests::sourcemap_invariants::assert_sourcemap_invariants;
use pretty_assertions::assert_eq;
use rstest::rstest;
use svelte_compiler::{compile, compile_module};
use test_support::strip_reference_only_css_markers;

fn normalize_css(s: &str) -> String {
    let stripped = strip_reference_only_css_markers(s);
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn assert_compiler(case: &str) {
    let dir = v3_case_dir(case);
    let (input, opts) = load_v3_case(case);
    let result = compile(&input, &opts);
    let js_output = result
        .js
        .unwrap_or_else(|| panic!("[{case}] compile produced no JS"));
    let js = js_output.code;

    let expected_js = read_to_string(dir.join("case-svelte.js")).expect("test invariant");

    File::create(dir.join("case-rust.js"))
        .expect("test invariant")
        .write_all(js.as_bytes())
        .expect("test invariant");

    assert_eq!(js, expected_js, "[{case}] JS mismatch");

    if let Some(map) = js_output.map.as_ref() {
        assert_sourcemap_invariants(case, &input, map, svelte_compiler::SourcemapKind::Default);
    }

    let expected_css_path = dir.join("case-svelte.css");
    if expected_css_path.exists() {
        let expected_css = read_to_string(&expected_css_path).expect("test invariant");
        let actual_css = result.css.map(|out| out.code).unwrap_or_default();
        File::create(dir.join("case-rust.css"))
            .expect("test invariant")
            .write_all(actual_css.as_bytes())
            .expect("test invariant");
        assert_eq!(
            normalize_css(&actual_css),
            normalize_css(&expected_css),
            "[{case}] CSS mismatch"
        );
    }
}

#[rstest]
fn legacy_const_each_bind_member_chain() {
    assert_compiler("legacy_const_each_bind_member_chain");
}

#[rstest]
fn diagnose_js_object_method_shorthand() {
    assert_compiler("diagnose_js_object_method_shorthand");
}

#[rstest]
fn diagnose_props_default_identifier_prop_reference() {
    assert_compiler("diagnose_props_default_identifier_prop_reference");
}

#[rstest]
fn diagnose_props_default_identifier_non_reactive() {
    assert_compiler("diagnose_props_default_identifier_non_reactive");
}

#[rstest]
fn css_scope_class_in_snippet() {
    assert_compiler("css_scope_class_in_snippet");
}

#[rstest]
fn css_scope_svelte_element_class() {
    assert_compiler("css_scope_svelte_element_class");
}

#[rstest]
fn css_scope_class_object() {
    assert_compiler("css_scope_class_object");
}

#[rstest]
fn css_scope_class_array_no_directive() {
    assert_compiler("css_scope_class_array_no_directive");
}

#[rstest]
fn css_scope_class_array_with_state() {
    assert_compiler("css_scope_class_array_with_state");
}

#[rstest]
fn css_scope_spread_attribute() {
    assert_compiler("css_scope_spread_attribute");
}

#[rstest]
fn css_unused_external() {
    assert_compiler("css_unused_external");
}

#[rstest]
fn css_unused_injected() {
    assert_compiler("css_unused_injected");
}

#[rstest]
fn css_nested_style() {
    assert_compiler("css_nested_style");
}

#[rstest]
fn css_nested_pseudo_element_no_scope_class() {
    assert_compiler("css_nested_pseudo_element_no_scope_class");
}

#[rstest]
fn css_nested_amp_compound_no_scope_class() {
    assert_compiler("css_nested_amp_compound_no_scope_class");
}

#[rstest]
fn diagnose_css_nested_sibling_amp_scope_class() {
    assert_compiler("diagnose_css_nested_sibling_amp_scope_class");
}

#[rstest]
fn diagnose_css_animation_vendor_prefix() {
    assert_compiler("diagnose_css_animation_vendor_prefix");
}

#[rstest]
fn diagnose_css_vendor_keyframes_rename() {
    assert_compiler("diagnose_css_vendor_keyframes_rename");
}

#[rstest]
fn css_scoped_id_selector() {
    assert_compiler("css_scoped_id_selector");
}

#[rstest]
fn css_scoped_attr_presence() {
    assert_compiler("css_scoped_attr_presence");
}

#[rstest]
fn css_scoped_attr_value_selector() {
    assert_compiler("css_scoped_attr_value_selector");
}

#[rstest]
fn css_scoped_attr_matcher_operators() {
    assert_compiler("css_scoped_attr_matcher_operators");
}

#[rstest]
fn css_scoped_attr_name_casefolding() {
    assert_compiler("css_scoped_attr_name_casefolding");
}

#[rstest]
fn css_pseudo_compound_unused_but_scoped() {
    assert_compiler("css_pseudo_compound_unused_but_scoped");
}

#[rstest]
fn css_scope_child_combinator_bare_pseudo() {
    assert_compiler("css_scope_child_combinator_bare_pseudo");
}

#[rstest]
fn css_scope_child_combinator_global_pseudo_unscoped() {
    assert_compiler("css_scope_child_combinator_global_pseudo_unscoped");
}

#[rstest]
fn css_snippet_descendant_scope_boundary() {
    assert_compiler("css_snippet_descendant_scope_boundary");
}

#[rstest]
fn css_snippet_sibling_boundary() {
    assert_compiler("css_snippet_sibling_boundary");
}

#[rstest]
fn diagnose_css_recursive_snippet_sibling_overflow() {
    assert_compiler("diagnose_css_recursive_snippet_sibling_overflow");
}

#[rstest]
fn css_component_snippet_descendant_boundary() {
    assert_compiler("css_component_snippet_descendant_boundary");
}

#[rstest]
fn css_pseudo_has() {
    assert_compiler("css_pseudo_has");
}

#[rstest]
fn css_pseudo_not_scoped() {
    assert_compiler("css_pseudo_not_scoped");
}

#[rstest]
fn css_nesting_selector_scoped() {
    assert_compiler("css_nesting_selector_scoped");
}

#[rstest]
fn css_root_has_scoped() {
    assert_compiler("css_root_has_scoped");
}

#[rstest]
fn css_escaped_selector_scoped() {
    assert_compiler("css_escaped_selector_scoped");
}

#[rstest]
fn css_dynamic_attr_selector_match() {
    assert_compiler("css_dynamic_attr_selector_match");
}

#[rstest]
fn css_comments_preserved() {
    assert_compiler("css_comments_preserved");
}

#[rstest]
fn script_module_exports_ordering_with_snippets() {
    assert_compiler("script_module_exports_ordering_with_snippets");
}

#[rstest]
fn script_jsdoc_preserve() {
    assert_compiler("script_jsdoc_preserve");
}

#[rstest]
fn state_raw_dev_ce_with_props_rest() {
    assert_compiler("state_raw_dev_ce_with_props_rest");
}

#[rstest]
fn warn_attr_avoid_is() {
    assert_compiler("warn_attr_avoid_is");
}

#[rstest]
fn warn_attr_illegal_colon() {
    assert_compiler("warn_attr_illegal_colon");
}

#[rstest]
fn warn_attr_invalid_prop_name() {
    assert_compiler("warn_attr_invalid_prop_name");
}

#[rstest]
fn warn_slot_deprecated() {
    assert_compiler("warn_slot_deprecated");
}

#[rstest]
fn slot_named_fallback() {
    assert_compiler("slot_named_fallback");
}

#[rstest]
fn legacy_slot_dev_mixed() {
    assert_compiler("legacy_slot_dev_mixed");
}

#[rstest]
fn slot_props_default() {
    assert_compiler("slot_props_default");
}

#[rstest]
fn slot_props_spread() {
    assert_compiler("slot_props_spread");
}

#[test]
fn slot_props_dynamic_state() {
    assert_compiler("slot_props_dynamic_state");
}

#[test]
fn slot_props_dynamic_call() {
    assert_compiler("slot_props_dynamic_call");
}

#[rstest]
fn diagnose_legacy_slot_props_store_member() {
    assert_compiler("diagnose_legacy_slot_props_store_member");
}

#[rstest]
fn diagnose_legacy_slot_prop_conditional_no_derived_wrap() {
    assert_compiler("diagnose_legacy_slot_prop_conditional_no_derived_wrap");
}

#[rstest]
fn diagnose_legacy_slot_prop_non_simple_stateful_shapes() {
    assert_compiler("diagnose_legacy_slot_prop_non_simple_stateful_shapes");
}

#[rstest]
fn warn_script_context_deprecated() {
    assert_compiler("warn_script_context_deprecated");
}

#[rstest]
fn head_with_special_elements() {
    assert_compiler("head_with_special_elements");
}

#[rstest]
fn head_with_snippets() {
    assert_compiler("head_with_snippets");
}

#[rstest]
fn head_with_if_body() {
    assert_compiler("head_with_if_body");
}

#[rstest]
fn head_with_render_and_component() {
    assert_compiler("head_with_render_and_component");
}

#[rstest]
fn diagnose_head_script_in_if() {
    assert_compiler("diagnose_head_script_in_if");
}

#[rstest]
fn diagnose_head_inline_script_template_literal() {
    assert_compiler("diagnose_head_inline_script_template_literal");
}

#[rstest]
fn head_nested_if_with_body_if_id_order() {
    assert_compiler("head_nested_if_with_body_if_id_order");
}

#[rstest]
fn head_title_then_meta_effect_order() {
    assert_compiler("head_title_then_meta_effect_order");
}

#[rstest]
fn push_binding_group_order() {
    assert_compiler("push_binding_group_order");
}

#[rstest]
fn bind_group_order_with_stores() {
    assert_compiler("bind_group_order_with_stores");
}

#[rstest]
fn bind_group_order_with_legacy_reactive() {
    assert_compiler("bind_group_order_with_legacy_reactive");
}

#[rstest]
fn component_bind_group_multiple_targets() {
    assert_compiler("component_bind_group_multiple_targets");
}

#[rstest]
fn bind_group_value_defined() {
    assert_compiler("bind_group_value_defined");
}

#[rstest]
fn bind_member_expression_no_runes() {
    assert_compiler("bind_member_expression_no_runes");
}

#[rstest]
fn legacy_const_destructured_member_bind() {
    assert_compiler("legacy_const_destructured_member_bind");
}

#[rstest]
fn diagnose_legacy_const_destructure_keeps_siblings() {
    assert_compiler("diagnose_legacy_const_destructure_keeps_siblings");
}

#[rstest]
fn legacy_const_member_mutation_through_ts_non_null() {
    assert_compiler("legacy_const_member_mutation_through_ts_non_null");
}

#[rstest]
fn css_injected_append_styles_with_stores_order() {
    assert_compiler("css_injected_append_styles_with_stores_order");
}

#[rstest]
fn css_scoped_basic() {
    assert_compiler("css_scoped_basic");
}

#[rstest]
fn css_injected() {
    assert_compiler("css_injected");
}

#[rstest]
fn css_global_basic() {
    assert_compiler("css_global_basic");
}

#[rstest]
fn css_global_block() {
    assert_compiler("css_global_block");
}

#[rstest]
fn css_global_compound() {
    assert_compiler("css_global_compound");
}

#[rstest]
fn css_global_in_pseudo() {
    assert_compiler("css_global_in_pseudo");
}

#[rstest]
fn css_global_with_combinators() {
    assert_compiler("css_global_with_combinators");
}

#[rstest]
fn diagnose_css_global_leading_combinator() {
    assert_compiler("diagnose_css_global_leading_combinator");
}

#[rstest]
fn css_keyframes_scoped() {
    assert_compiler("css_keyframes_scoped");
}

#[rstest]
fn css_keyframes_percentage_scopes_all() {
    assert_compiler("css_keyframes_percentage_scopes_all");
}

#[rstest]
fn bind_this_with_children_and_class_directive() {
    assert_compiler("bind_this_with_children_and_class_directive");
}

#[rstest]
fn head_position_with_body() {
    assert_compiler("head_position_with_body");
}

#[rstest]
fn special_elements_all() {
    assert_compiler("special_elements_all");
}

#[rstest]
fn empty() {
    assert_compiler("empty");
}

#[rstest]
fn simple() {
    assert_compiler("hello_state");
}

#[rstest]
fn single_text_node() {
    assert_compiler("single_text_node");
}

#[rstest]
fn single_element() {
    assert_compiler("single_element");
}

#[rstest]
fn single_interpolation() {
    assert_compiler("single_interpolation");
}

#[rstest]
fn text_entity_decoding() {
    assert_compiler("text_entity_decoding");
}

#[rstest]
fn text_entity_decoding_root() {
    assert_compiler("text_entity_decoding_root");
}

#[rstest]
fn title_entity_decoding() {
    assert_compiler("title_entity_decoding");
}

#[rstest]
fn single_if_block() {
    assert_compiler("single_if_block");
}

#[rstest]
fn single_if_else_block() {
    assert_compiler("single_if_else_block");
}

#[test]
fn if_call_condition() {
    assert_compiler("if_call_condition");
}

#[rstest]
fn if_block_empty_consequent() {
    assert_compiler("if_block_empty_consequent");
}

#[rstest]
fn if_block_empty_alternate() {
    assert_compiler("if_block_empty_alternate");
}

#[rstest]
fn element_attributes() {
    assert_compiler("element_attributes");
}

#[rstest]
fn element_autofocus() {
    assert_compiler("element_autofocus");
}

#[rstest]
fn textarea_child_value_dynamic() {
    assert_compiler("textarea_child_value_dynamic");
}

#[rstest]
fn option_expr_child_value() {
    assert_compiler("option_expr_child_value");
}

#[rstest]
fn option_expr_value() {
    assert_compiler("option_expr_value");
}

#[rstest]
fn option_concat_value() {
    assert_compiler("option_concat_value");
}

#[rstest]
fn option_expr_value_multi() {
    assert_compiler("option_expr_value_multi");
}

#[rstest]
fn option_expr_value_defined() {
    assert_compiler("option_expr_value_defined");
}

#[rstest]
fn option_expr_value_use_action_cache_var_order() {
    assert_compiler("option_expr_value_use_action_cache_var_order");
}

#[rstest]
fn bind_value_dev_named_fns() {
    assert_compiler("bind_value_dev_named_fns");
}

#[rstest]
fn bind_component_prop_dev_ownership() {
    assert_compiler("bind_component_prop_dev_ownership");
}

#[rstest]
fn bind_component_plain_prop_dev_ownership() {
    assert_compiler("bind_component_plain_prop_dev_ownership");
}

#[rstest]
fn bind_dynamic_component_dev_ownership() {
    assert_compiler("bind_dynamic_component_dev_ownership");
}

#[rstest]
fn bind_component_dev_ownership_ignore() {
    assert_compiler("bind_component_dev_ownership_ignore");
}

#[rstest]
fn bind_component_explicit_source() {
    assert_compiler("bind_component_explicit_source");
}

#[rstest]
fn customizable_select_option_el() {
    assert_compiler("customizable_select_option_el");
}

#[rstest]
fn customizable_select_select_div() {
    assert_compiler("customizable_select_select_div");
}

#[rstest]
fn selectedcontent_basic() {
    assert_compiler("selectedcontent_basic");
}

#[rstest]
fn state_runes() {
    assert_compiler("state_runes");
}

#[rstest]
fn state_raw() {
    assert_compiler("state_raw");
}

#[rstest]
fn state_eager_basic() {
    assert_compiler("state_eager_basic");
}

#[rstest]
fn state_eager_reactive() {
    assert_compiler("state_eager_reactive");
}

#[rstest]
fn state_eager_template() {
    assert_compiler("state_eager_template");
}

#[rstest]
fn state_snapshot_basic() {
    assert_compiler("state_snapshot_basic");
}

#[rstest]
fn state_snapshot_expression() {
    assert_compiler("state_snapshot_expression");
}

#[rstest]
fn state_snapshot_reactive() {
    assert_compiler("state_snapshot_reactive");
}

#[rstest]
fn each_block() {
    assert_compiler("each_block");
}

#[rstest]
fn each_inner_shadow() {
    assert_compiler("each_inner_shadow");
}

#[rstest]
fn each_nested_array_destructure_no_inner_shadow() {
    assert_compiler("each_nested_array_destructure_no_inner_shadow");
}

#[rstest]
fn each_legacy_shadow_with_script_array_assign() {
    assert_compiler("each_legacy_shadow_with_script_array_assign");
}

#[rstest]
fn bind_directives() {
    assert_compiler("bind_directives");
}

#[rstest]
fn nested_elements() {
    assert_compiler("nested_elements");
}

#[rstest]
fn nested_resets() {
    assert_compiler("nested_resets");
}

#[rstest]
fn single_concatenation() {
    assert_compiler("single_concatenation");
}

#[rstest]
fn elements_childs() {
    assert_compiler("elements_childs");
}

#[rstest]
fn generic_root_sequence() {
    assert_compiler("generic_root_sequence");
}

#[rstest]
fn spread_attribute() {
    assert_compiler("spread_attribute");
}

#[rstest]
fn attribute_effect_shorthand_prop_unwrap() {
    assert_compiler("attribute_effect_shorthand_prop_unwrap");
}

#[rstest]
fn img_spread_replay_events() {
    assert_compiler("img_spread_replay_events");
}

#[rstest]
fn embed_spread_replay_events() {
    assert_compiler("embed_spread_replay_events");
}

#[rstest]
fn object_spread_replay_events() {
    assert_compiler("object_spread_replay_events");
}

#[rstest]
fn source_spread_in_each_no_replay_events() {
    assert_compiler("source_spread_in_each_no_replay_events");
}

#[rstest]
fn spread_class_directive() {
    assert_compiler("spread_class_directive");
}

#[rstest]
fn spread_style_directive() {
    assert_compiler("spread_style_directive");
}

#[rstest]
fn utf8() {
    assert_compiler("utf8");
}

#[rstest]
fn smoke() {
    assert_compiler("smoke");
}

#[rstest]
fn class_directive() {
    assert_compiler("class_directive");
}

#[rstest]
fn diagnose_class_directive_name_with_underscore() {
    assert_compiler("diagnose_class_directive_name_with_underscore");
}

#[rstest]
fn diagnose_class_directive_named_like_slot_attribute_no_placement_error() {
    assert_compiler("diagnose_class_directive_named_like_slot_attribute_no_placement_error");
}

#[rstest]
fn diagnose_class_directive_call_in_template_effect() {
    assert_compiler("diagnose_class_directive_call_in_template_effect");
}

#[rstest]
fn diagnose_class_directive_slots_member_legacy() {
    assert_compiler("diagnose_class_directive_slots_member_legacy");
}

#[rstest]
fn diagnose_attribute_effect_spread_call_memo() {
    assert_compiler("diagnose_attribute_effect_spread_call_memo");
}

#[rstest]
fn diagnose_attribute_effect_concat_call_part_memo() {
    assert_compiler("diagnose_attribute_effect_concat_call_part_memo");
}

#[rstest]
fn diagnose_attribute_effect_hydration_ignored_eighth_arg() {
    assert_compiler("diagnose_attribute_effect_hydration_ignored_eighth_arg");
}

#[rstest]
fn diagnose_set_attribute_hydration_ignored_fourth_arg() {
    assert_compiler("diagnose_set_attribute_hydration_ignored_fourth_arg");
}

#[rstest]
fn diagnose_set_xlink_attribute_hydration_ignored_fourth_arg() {
    assert_compiler("diagnose_set_xlink_attribute_hydration_ignored_fourth_arg");
}

#[rstest]
fn class_concat() {
    assert_compiler("class_concat");
}

#[rstest]
fn empty_class_attribute_static_elided() {
    assert_compiler("empty_class_attribute_static_elided");
}

#[rstest]
fn diagnose_class_directive_on_svg_is_html_flag() {
    assert_compiler("diagnose_class_directive_on_svg_is_html_flag");
}

#[rstest]
fn class_concat_literal_fold() {
    assert_compiler("class_concat_literal_fold");
}

#[rstest]
fn attribute_concat_literal_fold() {
    assert_compiler("attribute_concat_literal_fold");
}

#[rstest]
fn component_prop_concat_literal_fold() {
    assert_compiler("component_prop_concat_literal_fold");
}

#[rstest]
fn rune_update() {
    assert_compiler("rune_update");
}

#[rstest]
fn assign_in_template() {
    assert_compiler("assign_in_template");
}

#[rstest]
fn only_script() {
    assert_compiler("only_script");
}

#[rstest]
fn hoist_imports() {
    assert_compiler("hoist_imports");
}

#[rstest]
fn bind_directives_extended() {
    assert_compiler("bind_directives_extended");
}

#[rstest]
fn mutated_state_rune() {
    assert_compiler("mutated_state_rune");
}

#[rstest]
fn static_interpolation() {
    assert_compiler("static_interpolation");
}

#[rstest]
fn props_basic() {
    assert_compiler("props_basic");
}

#[rstest]
fn props_rest() {
    assert_compiler("props_rest");
}

#[rstest]
fn props_renamed() {
    assert_compiler("props_renamed");
}

#[rstest]
fn props_const_destructured_with_default() {
    assert_compiler("props_const_destructured_with_default");
}

#[rstest]
fn template_effect_merge_class_state_with_memo_text() {
    assert_compiler("template_effect_merge_class_state_with_memo_text");
}

#[rstest]
fn props_renamed_bindable() {
    assert_compiler("props_renamed_bindable");
}

#[rstest]
fn props_bindable() {
    assert_compiler("props_bindable");
}

#[rstest]
fn props_lazy_default() {
    assert_compiler("props_lazy_default");
}

#[rstest]
fn props_mutated() {
    assert_compiler("props_mutated");
}

#[rstest]
fn props_member_mutation_computed() {
    assert_compiler("props_member_mutation_computed");
}

#[rstest]
fn props_renamed_member_update_computed() {
    assert_compiler("props_renamed_member_update_computed");
}

#[rstest]
fn props_mixed() {
    assert_compiler("props_mixed");
}

#[rstest]
fn exports() {
    assert_compiler("exports");
}

#[rstest]
fn snippet_basic() {
    assert_compiler("snippet_basic");
}

#[rstest]
fn component_basic() {
    assert_compiler("component_basic");
}

#[rstest]
fn svelte_component_basic() {
    assert_compiler("svelte_component_basic");
}

#[rstest]
fn svelte_component_children() {
    assert_compiler("svelte_component_children");
}

#[rstest]
fn svelte_component_if_child() {
    assert_compiler("svelte_component_if_child");
}

#[rstest]
fn svelte_component_each_child() {
    assert_compiler("svelte_component_each_child");
}

#[rstest]
fn svelte_component_slot_legacy_store_reactivity() {
    assert_compiler("svelte_component_slot_legacy_store_reactivity");
}

#[rstest]
fn svelte_component_slot_legacy_if_store_untrack() {
    assert_compiler("svelte_component_slot_legacy_if_store_untrack");
}

#[rstest]
fn component_non_self_closing() {
    assert_compiler("component_non_self_closing");
}

#[rstest]
fn component_in_element() {
    assert_compiler("component_in_element");
}

#[rstest]
fn component_mixed() {
    assert_compiler("component_mixed");
}

#[rstest]
fn component_props() {
    assert_compiler("component_props");
}

#[rstest]
fn component_children() {
    assert_compiler("component_children");
}

#[rstest]
fn diagnose_component_slot_node_naming() {
    assert_compiler("diagnose_component_slot_node_naming");
}

#[rstest]
fn diagnose_sibling_after_deep_nested_elements() {
    assert_compiler("diagnose_sibling_after_deep_nested_elements");
}

#[rstest]
fn legacy_slot_fallback_if_sibling_node_naming() {
    assert_compiler("legacy_slot_fallback_if_sibling_node_naming");
}

#[rstest]
fn component_events() {
    assert_compiler("component_events");
}

#[rstest]
fn component_events_dev_apply() {
    assert_compiler("component_events_dev_apply");
}

#[rstest]
fn component_element_children() {
    assert_compiler("component_element_children");
}

#[rstest]
fn component_named_slot() {
    assert_compiler("component_named_slot");
}

#[rstest]
fn legacy_slot_forward_named_into_child_component() {
    assert_compiler("legacy_slot_forward_named_into_child_component");
}

#[rstest]
fn component_default_slot_let() {
    assert_compiler("component_default_slot_let");
}

#[rstest]
fn diagnose_component_callee_from_slot_let() {
    assert_compiler("diagnose_component_callee_from_slot_let");
}

#[rstest]
fn diagnose_component_callee_member_legacy_reactive_root() {
    assert_compiler("diagnose_component_callee_member_legacy_reactive_root");
}

#[rstest]
fn diagnose_component_callee_from_slot_let_shadowed_by_script_binding() {
    assert_compiler("diagnose_component_callee_from_slot_let_shadowed_by_script_binding");
}

#[rstest]
fn component_let_directive_name_starts_with_on() {
    assert_compiler("component_let_directive_name_starts_with_on");
}

#[rstest]
fn diagnose_component_events_slot_let_handler() {
    assert_compiler("diagnose_component_events_slot_let_handler");
}

#[rstest]
fn diagnose_component_attr_on_prefix_false_positive() {
    assert_compiler("diagnose_component_attr_on_prefix_false_positive");
}

#[rstest]
fn component_default_slot_let_alias() {
    assert_compiler("component_default_slot_let_alias");
}

#[rstest]
fn component_named_slot_let_element() {
    assert_compiler("component_named_slot_let_element");
}

#[rstest]
fn component_named_slot_let_element_destructure() {
    assert_compiler("component_named_slot_let_element_destructure");
}

#[rstest]
fn component_named_slot_let_element_multiple() {
    assert_compiler("component_named_slot_let_element_multiple");
}

#[rstest]
fn component_child_slot_attribute() {
    assert_compiler("component_child_slot_attribute");
}

#[rstest]
fn smoke_all() {
    assert_compiler("smoke_all");
}

#[rstest]
fn derived_basic() {
    assert_compiler("derived_basic");
}

#[rstest]
fn diagnose_derived_rune_with_ts_as_cast() {
    assert_compiler("diagnose_derived_rune_with_ts_as_cast");
}

#[rstest]
fn derived_by() {
    assert_compiler("derived_by");
}

#[rstest]
fn derived_dynamic() {
    assert_compiler("derived_dynamic");
}

#[rstest]
fn derived_write_assignment() {
    assert_compiler("derived_write_assignment");
}

#[rstest]
fn unmutated_state_optimization() {
    assert_compiler("unmutated_state_optimization");
}

#[rstest]
fn effect_runes() {
    assert_compiler("effect_runes");
}

#[rstest]
fn effect_root_basic() {
    assert_compiler("effect_root_basic");
}

#[rstest]
fn effect_root_cleanup() {
    assert_compiler("effect_root_cleanup");
}

#[rstest]
fn effect_tracking() {
    assert_compiler("effect_tracking");
}

#[rstest]
fn effect_pending() {
    assert_compiler("effect_pending");
}

#[rstest]
fn effect_pending_script_init() {
    assert_compiler("effect_pending_script_init");
}

#[rstest]
fn effect_pending_script_derived() {
    assert_compiler("effect_pending_script_derived");
}

#[rstest]
fn host_basic() {
    assert_compiler("host_basic");
}

#[rstest]
fn host_props_rest() {
    assert_compiler("host_props_rest");
}

#[rstest]
fn custom_element_props() {
    assert_compiler("custom_element_props");
}

#[rstest]
fn custom_element_props_config() {
    assert_compiler("custom_element_props_config");
}

#[rstest]
fn custom_element_boolean_default() {
    assert_compiler("custom_element_boolean_default");
}

#[rstest]
fn custom_element_exports() {
    assert_compiler("custom_element_exports");
}

#[rstest]
fn custom_element_shadow_none() {
    assert_compiler("custom_element_shadow_none");
}

#[rstest]
fn custom_element_object_full() {
    assert_compiler("custom_element_object_full");
}

#[rstest]
fn custom_element_shadow_open() {
    assert_compiler("custom_element_shadow_open");
}

#[rstest]
fn custom_element_extend() {
    assert_compiler("custom_element_extend");
}

#[rstest]
fn custom_element_no_tag() {
    assert_compiler("custom_element_no_tag");
}

#[rstest]
fn custom_element_prop_alias() {
    assert_compiler("custom_element_prop_alias");
}

#[rstest]
fn custom_element_compile_option_default() {
    assert_compiler("custom_element_compile_option_default");
}

#[rstest]
fn custom_element_dev_exports_legacy_api() {
    assert_compiler("custom_element_dev_exports_legacy_api");
}

#[rstest]
fn custom_element_slots() {
    assert_compiler("custom_element_slots");
}

#[rstest]
fn legacy_props_basic() {
    assert_compiler("legacy_props_basic");
}

#[rstest]
fn legacy_reactivity_let_basic() {
    assert_compiler("legacy_reactivity_let_basic");
}

#[rstest]
fn legacy_reactivity_var_basic() {
    assert_compiler("legacy_reactivity_var_basic");
}

#[rstest]
fn legacy_reactivity_member_mutation() {
    assert_compiler("legacy_reactivity_member_mutation");
}

#[rstest]
fn legacy_reactivity_array_self_assign() {
    assert_compiler("legacy_reactivity_array_self_assign");
}

#[rstest]
fn legacy_reactivity_destructure() {
    assert_compiler("legacy_reactivity_destructure");
}

#[rstest]
fn diagnose_legacy_array_destructure_mixed_targets() {
    assert_compiler("diagnose_legacy_array_destructure_mixed_targets");
}

#[rstest]
fn legacy_reactive_assignment_basic() {
    assert_compiler("legacy_reactive_assignment_basic");
}

#[rstest]
fn diagnose_legacy_pre_effect_reset_with_module_script() {
    assert_compiler("diagnose_legacy_pre_effect_reset_with_module_script");
}

#[rstest]
fn legacy_reactive_assignment_declared_dependency() {
    assert_compiler("legacy_reactive_assignment_declared_dependency");
}

#[rstest]
fn legacy_reactive_assignment_block_destructure() {
    assert_compiler("legacy_reactive_assignment_block_destructure");
}

#[rstest]
fn legacy_reactive_assignment_coarse_deps() {
    assert_compiler("legacy_reactive_assignment_coarse_deps");
}

#[rstest]
fn legacy_reactive_assignment_import_topology() {
    assert_compiler("legacy_reactive_assignment_import_topology");
}

#[rstest]
fn legacy_rest_props_basic() {
    assert_compiler("legacy_rest_props_basic");
}

#[rstest]
fn legacy_slots_if() {
    assert_compiler("legacy_slots_if");
}

#[rstest]
fn legacy_slots_script_basic() {
    assert_compiler("legacy_slots_script_basic");
}

#[rstest]
fn legacy_before_after_update_basic() {
    assert_compiler("legacy_before_after_update_basic");
}

#[rstest]
fn legacy_before_after_update_alias() {
    assert_compiler("legacy_before_after_update_alias");
}

#[rstest]
fn custom_element_css_default_injected() {
    assert_compiler("custom_element_css_default_injected");
}

#[rstest]
fn custom_element_shadow_object() {
    assert_compiler("custom_element_shadow_object");
}

#[rstest]
fn html_tag() {
    assert_compiler("html_tag");
}

#[rstest]
fn html_tag_mathml() {
    assert_compiler("html_tag_mathml");
}

#[rstest]
fn svg_foreignobject_fragment_html() {
    assert_compiler("svg_foreignobject_fragment_html");
}

#[rstest]
fn mathml_root_html_fragment() {
    assert_compiler("mathml_root_html_fragment");
}

#[rstest]
fn mathml_annotation_xml_fragment_html() {
    assert_compiler("mathml_annotation_xml_fragment_html");
}

#[rstest]
fn key_block() {
    assert_compiler("key_block");
}

#[rstest]
fn key_block_nested() {
    assert_compiler("key_block_nested");
}

#[rstest]
fn style_directive() {
    assert_compiler("style_directive");
}

#[rstest]
fn css_custom_prop_component() {
    assert_compiler("css_custom_prop_component");
}

#[rstest]
fn css_custom_prop_component_svg() {
    assert_compiler("css_custom_prop_component_svg");
}

#[rstest]
fn css_custom_prop_component_nested() {
    assert_compiler("css_custom_prop_component_nested");
}

#[rstest]
fn css_custom_prop_component_string_value() {
    assert_compiler("css_custom_prop_component_string_value");
}

#[rstest]
fn css_custom_prop_component_concat_value() {
    assert_compiler("css_custom_prop_component_concat_value");
}

#[rstest]
fn css_custom_prop_component_with_memoized_prop() {
    assert_compiler("css_custom_prop_component_with_memoized_prop");
}

#[rstest]
fn css_custom_prop_component_slot_fill() {
    assert_compiler("css_custom_prop_component_slot_fill");
}

#[rstest]
fn style_directive_important() {
    assert_compiler("style_directive_important");
}

#[rstest]
fn style_directive_string() {
    assert_compiler("style_directive_string");
}

#[rstest]
fn style_directive_concat() {
    assert_compiler("style_directive_concat");
}

#[rstest]
fn on_directive() {
    assert_compiler("on_directive");
}

#[rstest]
fn on_directive_modifiers() {
    assert_compiler("on_directive_modifiers");
}

#[rstest]
fn on_directive_with_use() {
    assert_compiler("on_directive_with_use");
}

#[rstest]
fn on_directive_with_use_bind_this_order() {
    assert_compiler("on_directive_with_use_bind_this_order");
}

#[rstest]
fn on_directive_nonpassive() {
    assert_compiler("on_directive_nonpassive");
}

#[rstest]
fn on_directive_dev_apply() {
    assert_compiler("on_directive_dev_apply");
}

#[rstest]
fn use_action_basic() {
    assert_compiler("use_action_basic");
}

#[rstest]
fn use_action_expression() {
    assert_compiler("use_action_expression");
}

#[rstest]
fn use_action_reactive() {
    assert_compiler("use_action_reactive");
}

#[rstest]
fn use_action_dotted() {
    assert_compiler("use_action_dotted");
}

#[rstest]
fn use_action_dotted_hyphen() {
    assert_compiler("use_action_dotted_hyphen");
}

#[rstest]
fn use_action_multiple() {
    assert_compiler("use_action_multiple");
}

#[rstest]
fn use_action_in_if() {
    assert_compiler("use_action_in_if");
}

#[rstest]
fn use_action_in_each() {
    assert_compiler("use_action_in_each");
}

#[rstest]
fn use_action_with_children() {
    assert_compiler("use_action_with_children");
}

#[rstest]
fn void_elements() {
    assert_compiler("void_elements");
}

#[rstest]
fn non_void_self_closing() {
    assert_compiler("non_void_self_closing");
}

#[rstest]
fn mixed_html_elements() {
    assert_compiler("mixed_html_elements");
}

#[rstest]
fn store_basic() {
    assert_compiler("store_basic");
}

#[rstest]
fn store_legacy_let_synthetic_reassign() {
    assert_compiler("store_legacy_let_synthetic_reassign");
}

#[rstest]
fn store_legacy_each_invalidate() {
    assert_compiler("store_legacy_each_invalidate");
}

#[rstest]
fn store_legacy_each_member_iterable() {
    assert_compiler("store_legacy_each_member_iterable");
}

#[rstest]
fn store_bind_value_thunk_arrow() {
    assert_compiler("store_bind_value_thunk_arrow");
}

#[rstest]
fn store_legacy_var_basic() {
    assert_compiler("store_legacy_var_basic");
}

#[rstest]
fn store_legacy_member_mutation() {
    assert_compiler("store_legacy_member_mutation");
}

#[rstest]
fn store_runes_id_assign_ops() {
    assert_compiler("store_runes_id_assign_ops");
}

#[rstest]
fn store_runes_id_ops_template() {
    assert_compiler("store_runes_id_ops_template");
}

#[rstest]
fn store_runes_member_ops_script() {
    assert_compiler("store_runes_member_ops_script");
}

#[rstest]
fn store_runes_member_ops_template() {
    assert_compiler("store_runes_member_ops_template");
}

#[rstest]
fn store_runes_computed_member() {
    assert_compiler("store_runes_computed_member");
}

#[rstest]
fn store_runes_dev_smoke() {
    assert_compiler("store_runes_dev_smoke");
}

#[rstest]
fn store_runes_each_member_mutation() {
    assert_compiler("store_runes_each_member_mutation");
}

#[rstest]
fn store_runes_prop_thunk() {
    assert_compiler("store_runes_prop_thunk");
}

#[rstest]
fn store_runes_synthetic_thunk_derived_base() {
    assert_compiler("store_runes_synthetic_thunk_derived_base");
}

#[rstest]
fn store_runes_prop_assign_bind() {
    assert_compiler("store_runes_prop_assign_bind");
}

#[rstest]
fn diagnose_bindable_prop_store_only() {
    assert_compiler("diagnose_bindable_prop_store_only");
}

#[rstest]
fn store_runes_component_bind_prop_store() {
    assert_compiler("store_runes_component_bind_prop_store");
}

#[rstest]
fn component_bind_ref_state_flag() {
    assert_compiler("component_bind_ref_state_flag");
}

#[rstest]
fn component_bind_prop_order() {
    assert_compiler("component_bind_prop_order");
}

#[rstest]
fn component_snippet_node_ident_ordering() {
    assert_compiler("component_snippet_node_ident_ordering");
}

#[rstest]
fn store_legacy_id_assign_ops() {
    assert_compiler("store_legacy_id_assign_ops");
}

#[rstest]
fn store_legacy_id_ops_template() {
    assert_compiler("store_legacy_id_ops_template");
}

#[rstest]
fn store_legacy_member_ops_script() {
    assert_compiler("store_legacy_member_ops_script");
}

#[rstest]
fn store_legacy_dev_smoke() {
    assert_compiler("store_legacy_dev_smoke");
}

#[rstest]
fn store_legacy_bind_value() {
    assert_compiler("store_legacy_bind_value");
}

#[rstest]
fn diagnose_legacy_bind_value_writable_store_shadow() {
    assert_compiler("diagnose_legacy_bind_value_writable_store_shadow");
}

#[rstest]
fn legacy_dev_synthetic_store_thunk() {
    assert_compiler("legacy_dev_synthetic_store_thunk");
}

#[rstest]
fn legacy_dev_writable_no_mutable_source() {
    assert_compiler("legacy_dev_writable_no_mutable_source");
}

#[rstest]
fn legacy_dev_store_thunk_call_read() {
    assert_compiler("legacy_dev_store_thunk_call_read");
}

#[rstest]
fn legacy_dev_store_set_assignment() {
    assert_compiler("legacy_dev_store_set_assignment");
}

#[rstest]
fn legacy_dev_bind_store_unsub() {
    assert_compiler("legacy_dev_bind_store_unsub");
}

#[rstest]
fn legacy_dev_bind_this_promotes_state() {
    assert_compiler("legacy_dev_bind_this_promotes_state");
}

#[rstest]
fn legacy_dev_reactive_text_only_element() {
    assert_compiler("legacy_dev_reactive_text_only_element");
}

#[rstest]
fn legacy_dev_deferred_template_effect() {
    assert_compiler("legacy_dev_deferred_template_effect");
}

#[rstest]
fn diagnose_legacy_head_title_store_deps_coarse_wrap() {
    assert_compiler("diagnose_legacy_head_title_store_deps_coarse_wrap");
}

#[rstest]
fn store_if_block_condition() {
    assert_compiler("store_if_block_condition");
}

#[rstest]
fn store_key_block_expression() {
    assert_compiler("store_key_block_expression");
}

#[rstest]
fn store_await_block_promise() {
    assert_compiler("store_await_block_promise");
}

#[rstest]
fn store_render_tag_snippet() {
    assert_compiler("store_render_tag_snippet");
}

#[rstest]
fn store_bind_this_element_ref() {
    assert_compiler("store_bind_this_element_ref");
}

#[rstest]
fn store_bare_identifier_method_call() {
    assert_compiler("store_bare_identifier_method_call");
}

#[rstest]
fn store_write() {
    assert_compiler("store_write");
}

#[rstest]
fn store_validate_dev() {
    assert_compiler("store_validate_dev");
}

#[rstest]
fn store_reassign_unsub() {
    assert_compiler("store_reassign_unsub");
}

#[rstest]
fn store_each_invalidate() {
    assert_compiler("store_each_invalidate");
}

#[rstest]
fn store_mark_binding() {
    assert_compiler("store_mark_binding");
}

#[rstest]
fn const_tag() {
    assert_compiler("const_tag");
}

#[rstest]
fn const_tag_destructured() {
    assert_compiler("const_tag_destructured");
}

#[rstest]
fn const_tag_destructured_multi() {
    assert_compiler("const_tag_destructured_multi");
}

#[rstest]
fn const_tag_destructured_if() {
    assert_compiler("const_tag_destructured_if");
}

#[rstest]
fn const_tag_destructured_single_key() {
    assert_compiler("const_tag_destructured_single_key");
}

#[rstest]
fn const_tag_destructured_default() {
    assert_compiler("const_tag_destructured_default");
}

#[rstest]
fn diagnose_const_tag_legacy_destructure_into_component() {
    assert_compiler("diagnose_const_tag_legacy_destructure_into_component");
}

#[rstest]
fn const_tag_key_block() {
    assert_compiler("const_tag_key_block");
}

#[rstest]
fn const_tag_await() {
    assert_compiler("const_tag_await");
}

#[rstest]
fn const_tag_component() {
    assert_compiler("const_tag_component");
}

#[rstest]
fn class_array() {
    assert_compiler("class_array");
}

#[rstest]
fn class_object() {
    assert_compiler("class_object");
}

#[rstest]
fn class_variable() {
    assert_compiler("class_variable");
}

#[rstest]
fn class_expr_with_directives() {
    assert_compiler("class_expr_with_directives");
}

#[rstest]
fn class_concat_logical_and_string() {
    assert_compiler("class_concat_logical_and_string");
}

#[rstest]
fn bind_select_value() {
    assert_compiler("bind_select_value");
}

#[rstest]
fn diagnose_select_value_attribute_emits_special_value_triple() {
    assert_compiler("diagnose_select_value_attribute_emits_special_value_triple");
}

#[rstest]
fn bind_files() {
    assert_compiler("bind_files");
}

#[rstest]
fn bind_property() {
    assert_compiler("bind_property");
}

#[rstest]
fn bind_content_editable() {
    assert_compiler("bind_content_editable");
}

#[rstest]
fn bind_element_size() {
    assert_compiler("bind_element_size");
}

#[rstest]
fn bind_element_size_bindable_prop_source() {
    assert_compiler("bind_element_size_bindable_prop_source");
}

#[rstest]
fn bind_resize_observer() {
    assert_compiler("bind_resize_observer");
}

#[rstest]
fn bind_resize_observer_border_box_size() {
    assert_compiler("bind_resize_observer_border_box_size");
}

#[rstest]
fn bind_resize_observer_device_pixel_content_box_size() {
    assert_compiler("bind_resize_observer_device_pixel_content_box_size");
}

#[rstest]
fn bind_textarea_value() {
    assert_compiler("bind_textarea_value");
}

#[rstest]
fn bind_media_rw() {
    assert_compiler("bind_media_rw");
}

#[rstest]
fn bind_media_ro() {
    assert_compiler("bind_media_ro");
}

#[rstest]
fn bind_media_property() {
    assert_compiler("bind_media_property");
}

#[rstest]
fn bind_img() {
    assert_compiler("bind_img");
}

#[rstest]
fn bind_this() {
    assert_compiler("bind_this");
}

#[rstest]
fn component_bind_this() {
    assert_compiler("component_bind_this");
}

#[rstest]
fn component_bind_this_variants() {
    assert_compiler("component_bind_this_variants");
}

#[rstest]
fn svelte_self_if() {
    assert_compiler("svelte_self_if");
}

#[rstest]
fn svelte_self_each() {
    assert_compiler("svelte_self_each");
}

#[rstest]
fn svelte_self_snippet() {
    assert_compiler("svelte_self_snippet");
}

#[rstest]
fn svelte_self_slot() {
    assert_compiler("svelte_self_slot");
}

#[rstest]
fn svelte_self_props() {
    assert_compiler("svelte_self_props");
}

#[rstest]
fn svelte_self_bind_this() {
    assert_compiler("svelte_self_bind_this");
}

#[rstest]
fn bind_focused() {
    assert_compiler("bind_focused");
}

// ---------------------------------------------------------------------------
// Transition tests
// ---------------------------------------------------------------------------

#[rstest]
fn transition_basic() {
    assert_compiler("transition_basic");
}

#[rstest]
fn transition_params() {
    assert_compiler("transition_params");
}

#[rstest]
fn transition_in() {
    assert_compiler("transition_in");
}

#[rstest]
fn transition_out() {
    assert_compiler("transition_out");
}

#[rstest]
fn transition_in_out_separate() {
    assert_compiler("transition_in_out_separate");
}

#[rstest]
fn transition_local() {
    assert_compiler("transition_local");
}

#[rstest]
fn transition_global() {
    assert_compiler("transition_global");
}

#[rstest]
fn transition_dotted_name() {
    assert_compiler("transition_dotted_name");
}

#[rstest]
fn transition_in_if() {
    assert_compiler("transition_in_if");
}

#[rstest]
fn transition_reactive_params() {
    assert_compiler("transition_reactive_params");
}

#[rstest]
fn transition_elseif_local() {
    assert_compiler("transition_elseif_local");
}

#[rstest]
fn transition_after_delegated() {
    assert_compiler("transition_after_delegated");
}

#[rstest]
fn transition_after_delegated_descendant() {
    assert_compiler("transition_after_delegated_descendant");
}

#[rstest]
fn transition_before_lifecycle_events() {
    assert_compiler("transition_before_lifecycle_events");
}

// ---------------------------------------------------------------------------
// Animate directive tests
// ---------------------------------------------------------------------------

#[rstest]
fn animate_basic() {
    assert_compiler("animate_basic");
}

#[rstest]
fn animate_params() {
    assert_compiler("animate_params");
}

#[rstest]
fn animate_dotted_name() {
    assert_compiler("animate_dotted_name");
}

#[rstest]
fn animate_reactive_params() {
    assert_compiler("animate_reactive_params");
}

#[rstest]
fn animate_svelte_element() {
    assert_compiler("animate_svelte_element");
}

#[rstest]
fn animate_with_const_tag() {
    assert_compiler("animate_with_const_tag");
}

// ---------------------------------------------------------------------------
// Attach tag tests
// ---------------------------------------------------------------------------

#[rstest]
fn attach_basic() {
    assert_compiler("attach_basic");
}

#[rstest]
fn attach_inline_arrow() {
    assert_compiler("attach_inline_arrow");
}

#[rstest]
fn attach_conditional() {
    assert_compiler("attach_conditional");
}

#[rstest]
fn attach_multiple() {
    assert_compiler("attach_multiple");
}

#[rstest]
fn attach_with_directives() {
    assert_compiler("attach_with_directives");
}

#[rstest]
fn attach_in_if() {
    assert_compiler("attach_in_if");
}

#[rstest]
fn attach_in_each() {
    assert_compiler("attach_in_each");
}

// ---------------------------------------------------------------------------
// $state/$state.raw destructuring
// ---------------------------------------------------------------------------

#[rstest]
fn state_destructure() {
    assert_compiler("state_destructure");
}

#[rstest]
fn state_raw_destructure_object() {
    assert_compiler("state_raw_destructure_object");
}

#[rstest]
fn state_raw_destructure_array() {
    assert_compiler("state_raw_destructure_array");
}

// ---------------------------------------------------------------------------
// $state/$state.raw class fields
// ---------------------------------------------------------------------------

#[rstest]
fn state_class_field() {
    assert_compiler("state_class_field");
}

#[rstest]
fn state_raw_class_field() {
    assert_compiler("state_raw_class_field");
}

#[rstest]
fn state_private_class_field() {
    assert_compiler("state_private_class_field");
}

#[rstest]
fn state_class_constructor() {
    assert_compiler("state_class_constructor");
}

#[rstest]
fn state_class_multiple() {
    assert_compiler("state_class_multiple");
}

#[rstest]
fn state_constructor_private_read() {
    assert_compiler("state_constructor_private_read");
}

#[rstest]
fn state_constructor_read_v() {
    assert_compiler("state_constructor_read_v");
}

#[rstest]
fn state_constructor_read_derived() {
    assert_compiler("state_constructor_read_derived");
}

#[rstest]
fn state_class_raw_field() {
    assert_compiler("state_class_raw_field");
}

#[rstest]
fn state_no_init() {
    assert_compiler("state_no_init");
}

#[rstest]
fn state_snapshot_in_template() {
    assert_compiler("state_snapshot_in_template");
}

#[rstest]
fn state_snapshot_ignored() {
    assert_compiler("state_snapshot_ignored");
}

#[rstest]
fn state_snapshot_not_ignored() {
    assert_compiler("state_snapshot_not_ignored");
}

#[rstest]
fn state_snapshot_ignored_return() {
    assert_compiler("state_snapshot_ignored_return");
}

#[rstest]
fn for_await_ignored() {
    assert_compiler("for_await_ignored");
}

#[rstest]
fn await_reactivity_ignored() {
    assert_compiler("await_reactivity_ignored");
}

#[rstest]
fn state_class_constructor_proxy() {
    assert_compiler("state_class_constructor_proxy");
}

#[rstest]
fn derived_class_field() {
    assert_compiler("derived_class_field");
}

#[rstest]
fn derived_by_class_fields() {
    assert_compiler("derived_by_class_fields");
}

#[rstest]
fn derived_by_class_constructor_only() {
    assert_compiler("derived_by_class_constructor_only");
}

#[rstest]
fn derived_by_class_placeholder_preserves_plain_fields() {
    assert_compiler("derived_by_class_placeholder_preserves_plain_fields");
}

#[rstest]
fn state_class_field_constructor_assign() {
    assert_compiler("state_class_field_constructor_assign");
}

#[rstest]
fn svg_inner_whitespace_trimming() {
    assert_compiler("svg_inner_whitespace_trimming");
}

#[rstest]
fn svg_inner_template_from_svg() {
    assert_compiler("svg_inner_template_from_svg");
}

#[rstest]
fn template_effect_call_deps() {
    assert_compiler("template_effect_call_deps");
}

#[rstest]
fn svg_text_preserves_whitespace() {
    assert_compiler("svg_text_preserves_whitespace");
}

#[rstest]
fn template_effect_multiple_call_deps() {
    assert_compiler("template_effect_multiple_call_deps");
}

#[rstest]
fn template_effect_attr_before_text_order() {
    assert_compiler("template_effect_attr_before_text_order");
}

#[rstest]
fn component_local_underscored_bind_this() {
    assert_compiler("component_local_underscored_bind_this");
}

#[rstest]
fn component_dynamic_dotted_identifier_root() {
    assert_compiler("component_dynamic_dotted_identifier_root");
}

#[rstest]
fn component_dynamic_props_access() {
    assert_compiler("component_dynamic_props_access");
}

#[rstest]
fn component_dynamic_dotted_props_root() {
    assert_compiler("component_dynamic_dotted_props_root");
}

// ---------------------------------------------------------------------------
// Module compilation tests
// ---------------------------------------------------------------------------

fn strip_leading_block_comments(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    while i < bytes.len() {
        if i + 2 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' && bytes[i + 2] == b'*' {
            let mut j = i + 3;
            while j + 1 < bytes.len() && !(bytes[j] == b'*' && bytes[j + 1] == b'/') {
                j += 1;
            }
            if j + 1 >= bytes.len() {
                out.push_str(&src[i..]);
                break;
            }
            let mut k = j + 2;
            while k < bytes.len() && matches!(bytes[k], b' ' | b'\t') {
                k += 1;
            }
            if k < bytes.len() && bytes[k] == b'\n' {
                k += 1;
                let line_start = out.rfind('\n').map_or(0, |p| p + 1);
                if out[line_start..].chars().all(|c| c == ' ' || c == '\t') {
                    out.truncate(line_start);
                }
                i = k;
            } else {
                i = j + 2;
            }
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn assert_compiler_module(case: &str) {
    let dir = v3_case_dir(case);
    let (input, opts) = load_v3_module_case(case);

    let result = compile_module(&input, &opts);
    let js_output = result
        .js
        .unwrap_or_else(|| panic!("[{case}] compile_module produced no JS"));
    let js = js_output.code;

    let expected = read_to_string(dir.join("case-svelte.js")).expect("test invariant");

    File::create(dir.join("case-rust.js"))
        .expect("test invariant")
        .write_all(js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_leading_block_comments(&js),
        strip_leading_block_comments(&expected)
    );

    if let Some(map) = js_output.map.as_ref() {
        assert_sourcemap_invariants(case, &input, map, svelte_compiler::SourcemapKind::Default);
    }
}

#[rstest]
fn state_class_field_proxy_init() {
    assert_compiler_module("state_class_field_proxy_init");
}

#[rstest]
fn module_derived_arrow_wrap_with_state_deps() {
    assert_compiler_module("module_derived_arrow_wrap_with_state_deps");
}

#[rstest]
fn module_state_destructure() {
    assert_compiler_module("module_state_destructure");
}

#[rstest]
fn module_compilation() {
    assert_compiler_module("module_compilation");
}

#[rstest]
fn state_raw_class_constructor_object_ts() {
    assert_compiler_module("state_raw_class_constructor_object_ts");
}

#[rstest]
fn module_derived_arrow_wrap_in_class_method() {
    assert_compiler_module("module_derived_arrow_wrap_in_class_method");
}
#[rstest]
fn module_ts_strip() {
    assert_compiler_module("module_ts_strip");
}

#[rstest]
fn module_derived_arrow_wrap_no_state_deps() {
    assert_compiler_module("module_derived_arrow_wrap_no_state_deps");
}

#[rstest]
fn diagnose_state_private_constructor_init_from_decl_only() {
    assert_compiler_module("diagnose_state_private_constructor_init_from_decl_only");
}

#[rstest]
fn state_private_class_field_array_proxy() {
    assert_compiler_module("state_private_class_field_array_proxy");
}

#[rstest]
fn diagnose_module_leading_jsdoc_dropped() {
    assert_compiler_module("diagnose_module_leading_jsdoc_dropped");
}

#[rstest]
fn ts_strip_non_null_chain() {
    assert_compiler_module("ts_strip_non_null_chain");
}

#[rstest]
fn module_dev_state_tag() {
    assert_compiler_module("module_dev_state_tag");
}

#[rstest]
fn module_dev_derived_tag() {
    assert_compiler_module("module_dev_derived_tag");
}

#[rstest]
fn module_dev_console_log_wrap() {
    assert_compiler_module("module_dev_console_log_wrap");
}

#[rstest]
fn diagnose_module_import_between_top_level_statements() {
    assert_compiler_module("diagnose_module_import_between_top_level_statements");
}

#[rstest]
fn script_module_exports() {
    assert_compiler("script_module_exports");
}

#[rstest]
fn script_module_export_specifiers() {
    assert_compiler("script_module_export_specifiers");
}

#[rstest]
fn script_module_imports() {
    assert_compiler("script_module_imports");
}

#[rstest]
fn script_module_empty() {
    assert_compiler("script_module_empty");
}

#[rstest]
fn script_module_runes() {
    assert_compiler("script_module_runes");
}

#[rstest]
fn script_module_instance_ref() {
    assert_compiler("script_module_instance_ref");
}

#[rstest]
fn script_module_only() {
    assert_compiler("script_module_only");
}

#[rstest]
fn script_module_with_instance() {
    assert_compiler("script_module_with_instance");
}

#[rstest]
fn svelte_options_basic() {
    assert_compiler("svelte_options_basic");
}

#[rstest]
fn svelte_options_runes_false_override() {
    assert_compiler("svelte_options_runes_false_override");
}

#[rstest]
fn svelte_options_accessors_legacy() {
    assert_compiler("svelte_options_accessors_legacy");
}

#[rstest]
fn svelte_options_immutable_legacy() {
    assert_compiler("svelte_options_immutable_legacy");
}

#[rstest]
fn legacy_export_let_required() {
    assert_compiler("legacy_export_let_required");
}

#[rstest]
fn legacy_export_var_basic() {
    assert_compiler("legacy_export_var_basic");
}

#[rstest]
fn legacy_export_specifier() {
    assert_compiler("legacy_export_specifier");
}

#[rstest]
fn legacy_export_specifier_alias() {
    assert_compiler("legacy_export_specifier_alias");
}

#[rstest]
fn legacy_export_destructure() {
    assert_compiler("legacy_export_destructure");
}

#[rstest]
fn legacy_export_let_typed() {
    assert_compiler("legacy_export_let_typed");
}

#[rstest]
fn legacy_export_let_member_mutation() {
    assert_compiler("legacy_export_let_member_mutation");
}

#[rstest]
fn legacy_export_let_bind_to_inner() {
    assert_compiler("legacy_export_let_bind_to_inner");
}

#[rstest]
fn diagnose_legacy_export_let_element_bind_this() {
    assert_compiler("diagnose_legacy_export_let_element_bind_this");
}

#[rstest]
fn diagnose_legacy_bind_group_radio_export_let_prop() {
    assert_compiler("diagnose_legacy_bind_group_radio_export_let_prop");
}

#[rstest]
fn diagnose_legacy_bind_group_radio_attr_update_order() {
    assert_compiler("diagnose_legacy_bind_group_radio_attr_update_order");
}

#[rstest]
fn diagnose_legacy_bind_group_value_use_action_cache_var_order() {
    assert_compiler("diagnose_legacy_bind_group_value_use_action_cache_var_order");
}

#[rstest]
fn diagnose_legacy_bind_group_radio_value_shorthand_spread() {
    assert_compiler("diagnose_legacy_bind_group_radio_value_shorthand_spread");
}

#[rstest]
fn diagnose_legacy_each_bind_group_input_value_merges_with_text() {
    assert_compiler("diagnose_legacy_each_bind_group_input_value_merges_with_text");
}

#[rstest]
fn diagnose_legacy_each_bind_group_radio_value_index_no_cache() {
    assert_compiler("diagnose_legacy_each_bind_group_radio_value_index_no_cache");
}

#[rstest]
fn diagnose_legacy_input_bind_spread_omits_remove_defaults() {
    assert_compiler("diagnose_legacy_input_bind_spread_omits_remove_defaults");
}

#[rstest]
fn diagnose_input_spread_only_should_remove_defaults() {
    assert_compiler("diagnose_input_spread_only_should_remove_defaults");
}

#[rstest]
fn diagnose_input_boolean_value_emits_remove_defaults() {
    assert_compiler("diagnose_input_boolean_value_emits_remove_defaults");
}

#[rstest]
fn diagnose_input_default_value_attribute_omits_remove_defaults() {
    assert_compiler("diagnose_input_default_value_attribute_omits_remove_defaults");
}

#[rstest]
fn diagnose_spread_bind_this_action_order() {
    assert_compiler("diagnose_spread_bind_this_action_order");
}

#[rstest]
fn diagnose_legacy_attr_expression_restprops_member_coarse_wrap() {
    assert_compiler("diagnose_legacy_attr_expression_restprops_member_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_attribute_effect_expression_restprops_coarse_wrap() {
    assert_compiler("diagnose_legacy_attribute_effect_expression_restprops_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_restprops_template_only_function_param() {
    assert_compiler("diagnose_legacy_restprops_template_only_function_param");
}

#[rstest]
fn diagnose_legacy_child_bind_after_parent_spread_event_order() {
    assert_compiler("diagnose_legacy_child_bind_after_parent_spread_event_order");
}

#[rstest]
fn diagnose_legacy_bind_value_on_implicit_reactive_declaration() {
    assert_compiler("diagnose_legacy_bind_value_on_implicit_reactive_declaration");
}

#[rstest]
fn diagnose_script_let_multi_declarator_splits() {
    assert_compiler("diagnose_script_let_multi_declarator_splits");
}

#[rstest]
fn diagnose_script_empty_named_import_normalized_to_bare() {
    assert_compiler("diagnose_script_empty_named_import_normalized_to_bare");
}

#[rstest]
fn diagnose_script_ts_empty_named_import_normalized_to_bare() {
    assert_compiler("diagnose_script_ts_empty_named_import_normalized_to_bare");
}

#[rstest]
fn diagnose_legacy_bind_value_textarea_export_let_prop() {
    assert_compiler("diagnose_legacy_bind_value_textarea_export_let_prop");
}

#[rstest]
fn diagnose_legacy_bind_files_export_let_prop() {
    assert_compiler("diagnose_legacy_bind_files_export_let_prop");
}

#[rstest]
fn diagnose_textarea_bind_value_with_spread_init_order() {
    assert_compiler("diagnose_textarea_bind_value_with_spread_init_order");
}

#[rstest]
fn diagnose_textarea_spread_only_emits_remove_child() {
    assert_compiler("diagnose_textarea_spread_only_emits_remove_child");
}

#[rstest]
fn diagnose_textarea_dynamic_value_attribute_emits_remove_child() {
    assert_compiler("diagnose_textarea_dynamic_value_attribute_emits_remove_child");
}

#[rstest]
fn diagnose_legacy_export_let_store_prop_subscription() {
    assert_compiler("diagnose_legacy_export_let_store_prop_subscription");
}

#[rstest]
fn diagnose_legacy_export_let_store_prop_writes() {
    assert_compiler("diagnose_legacy_export_let_store_prop_writes");
}

#[rstest]
fn diagnose_legacy_export_let_leaks_into_exports() {
    assert_compiler("diagnose_legacy_export_let_leaks_into_exports");
}

#[rstest]
fn diagnose_runes_prop_export_specifier() {
    assert_compiler("diagnose_runes_prop_export_specifier");
}

#[rstest]
fn runes_prop_export_specifier_alias() {
    assert_compiler("runes_prop_export_specifier_alias");
}

#[rstest]
fn runes_prop_export_specifier_dev() {
    assert_compiler("runes_prop_export_specifier_dev");
}

#[rstest]
fn runes_state_export_specifier() {
    assert_compiler("runes_state_export_specifier");
}

#[rstest]
fn runes_raw_state_export_specifier() {
    assert_compiler("runes_raw_state_export_specifier");
}

#[rstest]
fn runes_local_let_export_specifier() {
    assert_compiler("runes_local_let_export_specifier");
}

#[rstest]
fn runes_local_const_export_specifier() {
    assert_compiler("runes_local_const_export_specifier");
}

#[rstest]
fn runes_derived_export_specifier() {
    assert_compiler("runes_derived_export_specifier");
}

#[rstest]
fn diagnose_legacy_export_const_init_order() {
    assert_compiler("diagnose_legacy_export_const_init_order");
}

#[rstest]
fn diagnose_legacy_props_spread_no_init() {
    assert_compiler("diagnose_legacy_props_spread_no_init");
}

#[rstest]
fn diagnose_legacy_props_spread_only_no_push() {
    assert_compiler("diagnose_legacy_props_spread_only_no_push");
}

#[rstest]
fn diagnose_legacy_export_const_excluded_from_rest_props() {
    assert_compiler("diagnose_legacy_export_const_excluded_from_rest_props");
}

#[rstest]
fn diagnose_legacy_export_const_bind_prop_after_template_render() {
    assert_compiler("diagnose_legacy_export_const_bind_prop_after_template_render");
}

#[rstest]
fn diagnose_css_attribute_selector_quote_style() {
    assert_compiler("diagnose_css_attribute_selector_quote_style");
}

#[rstest]
fn diagnose_css_adjacent_sibling_across_if_block() {
    assert_compiler("diagnose_css_adjacent_sibling_across_if_block");
}

#[rstest]
fn diagnose_css_attribute_selector_unquoted_value() {
    assert_compiler("diagnose_css_attribute_selector_unquoted_value");
}

#[rstest]
fn diagnose_css_declaration_value_comment() {
    assert_compiler("diagnose_css_declaration_value_comment");
}

#[rstest]
fn diagnose_css_type_class_selector_on_svelte_element() {
    assert_compiler("diagnose_css_type_class_selector_on_svelte_element");
}

#[rstest]
fn diagnose_legacy_component_bind_store_prop() {
    assert_compiler("diagnose_legacy_component_bind_store_prop");
}

#[rstest]
fn diagnose_legacy_component_bind_store_reactive_destructured_base() {
    assert_compiler("diagnose_legacy_component_bind_store_reactive_destructured_base");
}

#[rstest]
fn legacy_export_let_compound_assign_prop() {
    assert_compiler("legacy_export_let_compound_assign_prop");
}

#[rstest]
fn legacy_export_let_update_prop_in_template() {
    assert_compiler("legacy_export_let_update_prop_in_template");
}

#[rstest]
fn legacy_export_let_assign_prop_in_template() {
    assert_compiler("legacy_export_let_assign_prop_in_template");
}

#[rstest]
fn legacy_state_member_update_in_template() {
    assert_compiler("legacy_state_member_update_in_template");
}

#[rstest]
fn legacy_state_member_compound_in_template() {
    assert_compiler("legacy_state_member_compound_in_template");
}

#[rstest]
fn legacy_export_let_member_update_in_template() {
    assert_compiler("legacy_export_let_member_update_in_template");
}

#[rstest]
fn diagnose_legacy_export_let_member_assign_computed_member_key() {
    assert_compiler("diagnose_legacy_export_let_member_assign_computed_member_key");
}

#[rstest]
fn legacy_export_let_default_typed_cast_arrow() {
    assert_compiler("legacy_export_let_default_typed_cast_arrow");
}

#[rstest]
fn diagnose_legacy_export_let_default_prop_reference() {
    assert_compiler("diagnose_legacy_export_let_default_prop_reference");
}

#[rstest]
fn diagnose_legacy_export_let_default_prop_reference_in_conditional() {
    assert_compiler("diagnose_legacy_export_let_default_prop_reference_in_conditional");
}

#[rstest]
fn diagnose_legacy_export_let_closure_capture_needs_push_init_pop() {
    assert_compiler("diagnose_legacy_export_let_closure_capture_needs_push_init_pop");
}

#[rstest]
fn diagnose_legacy_each_key_prop_call_needs_push_init_pop() {
    assert_compiler("diagnose_legacy_each_key_prop_call_needs_push_init_pop");
}

#[rstest]
fn svelte_component_legacy_derived_hoist_block() {
    assert_compiler("svelte_component_legacy_derived_hoist_block");
}

#[rstest]
fn legacy_export_let_key_block_member_coarse_wrap() {
    assert_compiler("legacy_export_let_key_block_member_coarse_wrap");
}

#[rstest]
fn legacy_pre_effect_store_subscription_dep() {
    assert_compiler("legacy_pre_effect_store_subscription_dep");
}

#[rstest]
fn diagnose_legacy_reactive_ts_type_position_dep() {
    assert_compiler("diagnose_legacy_reactive_ts_type_position_dep");
}

#[rstest]
fn diagnose_legacy_reactive_array_destructure_with_store() {
    assert_compiler("diagnose_legacy_reactive_array_destructure_with_store");
}

#[rstest]
fn legacy_reactive_assignment_object_destructure_with_store() {
    assert_compiler("legacy_reactive_assignment_object_destructure_with_store");
}

#[rstest]
fn legacy_reactive_object_destructure_single_store_leaf() {
    assert_compiler("legacy_reactive_object_destructure_single_store_leaf");
}

#[rstest]
fn legacy_reactive_assignment_mixed_destructure() {
    assert_compiler("legacy_reactive_assignment_mixed_destructure");
}

#[rstest]
fn diagnose_legacy_reactive_destructure_identifier_rhs() {
    assert_compiler("diagnose_legacy_reactive_destructure_identifier_rhs");
}

#[rstest]
fn runes_prop_member_update_in_template() {
    assert_compiler("runes_prop_member_update_in_template");
}

#[rstest]
fn runes_prop_member_compound_in_template() {
    assert_compiler("runes_prop_member_compound_in_template");
}

#[rstest]
fn smoke_legacy_reactive_mutations_all() {
    assert_compiler("smoke_legacy_reactive_mutations_all");
}

#[rstest]
fn smoke_runes_reactive_mutations_all() {
    assert_compiler("smoke_runes_reactive_mutations_all");
}

#[rstest]
fn smoke_legacy_contextual_mutations_all() {
    assert_compiler("smoke_legacy_contextual_mutations_all");
}

#[rstest]
fn smoke_legacy_rune_fallback_all() {
    assert_compiler("smoke_legacy_rune_fallback_all");
}

#[rstest]
fn derived_non_runes_invalid_usage() {
    assert_compiler("derived_non_runes_invalid_usage");
}

#[rstest]
fn smoke_runes_declarator_gaps_all() {
    assert_compiler("smoke_runes_declarator_gaps_all");
}

#[rstest]
fn smoke_runes_state_eager_panic() {
    assert_compiler("smoke_runes_state_eager_panic");
}

#[rstest]
fn smoke_ts_non_null_assertion_mutations() {
    assert_compiler("smoke_ts_non_null_assertion_mutations");
}

#[rstest]
fn text_expression_binary_no_nullish_fallback() {
    assert_compiler("text_expression_binary_no_nullish_fallback");
}

#[rstest]
fn text_expression_conditional_memoized_needs_nullish_fallback() {
    assert_compiler("text_expression_conditional_memoized_needs_nullish_fallback");
}

#[rstest]
fn svelte_options_preserve_whitespace() {
    assert_compiler("svelte_options_preserve_whitespace");
}

#[rstest]
fn preserve_whitespace_compile_option_true() {
    assert_compiler("preserve_whitespace_compile_option_true");
}

#[rstest]
fn preserve_whitespace_pre_first_newline() {
    assert_compiler("preserve_whitespace_pre_first_newline");
}

#[rstest]
fn preserve_whitespace_inner_trailing_text() {
    assert_compiler("preserve_whitespace_inner_trailing_text");
}

#[rstest]
fn preserve_whitespace_script_element() {
    assert_compiler("preserve_whitespace_script_element");
}

#[rstest]
fn diagnose_svg_fragment_root_ws_between_siblings() {
    assert_compiler("diagnose_svg_fragment_root_ws_between_siblings");
}

#[rstest]
fn diagnose_svg_block_fragment_ws_between_siblings() {
    assert_compiler("diagnose_svg_block_fragment_ws_between_siblings");
}

#[rstest]
fn diagnose_svg_component_slot_ws_between_siblings() {
    assert_compiler("diagnose_svg_component_slot_ws_between_siblings");
}

#[rstest]
fn diagnose_svg_text_block_ws_preserved() {
    assert_compiler("diagnose_svg_text_block_ws_preserved");
}

#[rstest]
fn diagnose_svg_snippet_body_ws_between_siblings() {
    assert_compiler("diagnose_svg_snippet_body_ws_between_siblings");
}

#[rstest]
fn diagnose_svg_legacy_slot_ws_between_siblings() {
    assert_compiler("diagnose_svg_legacy_slot_ws_between_siblings");
}

#[rstest]
fn diagnose_svg_root_html_tag_strategy() {
    assert_compiler("diagnose_svg_root_html_tag_strategy");
}

#[rstest]
fn diagnose_svg_root_block_only_html_tag() {
    assert_compiler("diagnose_svg_root_block_only_html_tag");
}

#[rstest]
fn preserve_comments_basic() {
    assert_compiler("preserve_comments_basic");
}

#[rstest]
fn preserve_comments_only_child() {
    assert_compiler("preserve_comments_only_child");
}

#[rstest]
fn preserve_comments_between_elements() {
    assert_compiler("preserve_comments_between_elements");
}

#[rstest]
fn preserve_comments_in_block() {
    assert_compiler("preserve_comments_in_block");
}

#[rstest]
fn preserve_comments_svelte_ignore() {
    assert_compiler("preserve_comments_svelte_ignore");
}

#[rstest]
fn preserve_comments_only_in_block() {
    assert_compiler("preserve_comments_only_in_block");
}

#[rstest]
fn preserve_comments_consecutive() {
    assert_compiler("preserve_comments_consecutive");
}

#[rstest]
fn diagnose_consecutive_comments_between_components() {
    assert_compiler("diagnose_consecutive_comments_between_components");
}

#[rstest]
fn preserve_comments_empty() {
    assert_compiler("preserve_comments_empty");
}

#[rstest]
fn preserve_comments_in_each() {
    assert_compiler("preserve_comments_in_each");
}

// ---------------------------------------------------------------------------
// svelte:head tests
// ---------------------------------------------------------------------------

#[rstest]
fn svelte_head_basic() {
    assert_compiler("svelte_head_basic");
}

#[rstest]
fn svelte_head_reactive() {
    assert_compiler("svelte_head_reactive");
}

#[rstest]
fn svelte_head_with_content() {
    assert_compiler("svelte_head_with_content");
}

// <title> in <svelte:head> tests
#[rstest]
fn title_variants() {
    assert_compiler("title_variants");
}

#[rstest]
fn async_title_basic() {
    assert_compiler("async_title_basic");
}

// svelte:window tests
#[rstest]
fn svelte_window_event_legacy() {
    assert_compiler("svelte_window_event_legacy");
}

#[rstest]
fn svelte_window_event_legacy_with_if() {
    assert_compiler("svelte_window_event_legacy_with_if");
}

#[rstest]
fn diagnose_svelte_window_on_directive_legacy_prop_handler_wraps() {
    assert_compiler("diagnose_svelte_window_on_directive_legacy_prop_handler_wraps");
}

#[rstest]
fn diagnose_svelte_document_on_directive_legacy_prop_handler_wraps() {
    assert_compiler("diagnose_svelte_document_on_directive_legacy_prop_handler_wraps");
}

#[rstest]
fn diagnose_svelte_body_on_directive_legacy_prop_handler_wraps() {
    assert_compiler("diagnose_svelte_body_on_directive_legacy_prop_handler_wraps");
}

#[rstest]
fn svelte_window_action() {
    assert_compiler("svelte_window_action");
}

#[rstest]
fn svelte_document_action() {
    assert_compiler("svelte_document_action");
}

#[rstest]
fn svelte_window_event_attr() {
    assert_compiler("svelte_window_event_attr");
}

#[rstest]
fn svelte_window_event_attr_props_handler() {
    assert_compiler("svelte_window_event_attr_props_handler");
}

#[rstest]
fn svelte_document_event_attr_props_handler() {
    assert_compiler("svelte_document_event_attr_props_handler");
}

#[rstest]
fn svelte_body_event_attr_props_handler() {
    assert_compiler("svelte_body_event_attr_props_handler");
}

#[rstest]
fn svelte_window_bind_scroll() {
    assert_compiler("svelte_window_bind_scroll");
}

#[rstest]
fn svelte_window_bind_size() {
    assert_compiler("svelte_window_bind_size");
}

#[rstest]
fn svelte_window_bind_size_legacy() {
    assert_compiler("svelte_window_bind_size_legacy");
}

#[rstest]
fn svelte_window_bind_size_with_template() {
    assert_compiler("svelte_window_bind_size_with_template");
}

#[rstest]
fn svelte_window_bind_online() {
    assert_compiler("svelte_window_bind_online");
}

#[rstest]
fn svelte_window_combined() {
    assert_compiler("svelte_window_combined");
}

#[rstest]
fn svelte_window_reactive() {
    assert_compiler("svelte_window_reactive");
}

#[rstest]
fn svelte_document_bindings() {
    assert_compiler("svelte_document_bindings");
}

#[rstest]
fn svelte_document_events() {
    assert_compiler("svelte_document_events");
}

#[rstest]
fn svelte_document_bubble() {
    assert_compiler("svelte_document_bubble");
}

#[rstest]
fn svelte_document_combined() {
    assert_compiler("svelte_document_combined");
}

#[rstest]
fn svelte_element_basic() {
    assert_compiler("svelte_element_basic");
}

#[rstest]
fn svelte_element_self_closing() {
    assert_compiler("svelte_element_self_closing");
}

#[rstest]
fn svelte_fragment_named_slot() {
    assert_compiler("svelte_fragment_named_slot");
}

#[rstest]
fn svelte_fragment_named_slot_inside_svelte_component() {
    assert_compiler("svelte_fragment_named_slot_inside_svelte_component");
}

#[rstest]
fn svelte_fragment_named_slot_component_expr_attr() {
    assert_compiler("svelte_fragment_named_slot_component_expr_attr");
}

#[rstest]
fn component_named_slot_let_fragment() {
    assert_compiler("component_named_slot_let_fragment");
}

#[rstest]
fn component_named_slot_let_fragment_destructure() {
    assert_compiler("component_named_slot_let_fragment_destructure");
}

#[rstest]
fn svelte_fragment_default_slot_wrapper() {
    assert_compiler("svelte_fragment_default_slot_wrapper");
}

#[rstest]
fn svelte_fragment_default_slot_let() {
    assert_compiler("svelte_fragment_default_slot_let");
}

#[rstest]
fn svelte_fragment_explicit_default_slot_attribute_lowers_to_children_prop() {
    assert_compiler("svelte_fragment_explicit_default_slot_attribute_lowers_to_children_prop");
}

#[rstest]
fn svelte_fragment_named_slot_with_const_tag() {
    assert_compiler("svelte_fragment_named_slot_with_const_tag");
}

#[rstest]
fn svelte_element_static_tag() {
    assert_compiler("svelte_element_static_tag");
}

#[rstest]
fn svelte_element_attributes() {
    assert_compiler("svelte_element_attributes");
}

#[rstest]
fn svelte_element_spread() {
    assert_compiler("svelte_element_spread");
}

#[rstest]
fn diagnose_svelte_element_spread_with_class_directive() {
    assert_compiler("diagnose_svelte_element_spread_with_class_directive");
}

#[rstest]
fn diagnose_svelte_element_class_directive_with_dynamic_attr() {
    assert_compiler("diagnose_svelte_element_class_directive_with_dynamic_attr");
}

#[rstest]
fn diagnose_svelte_element_spread_style_directive() {
    assert_compiler("diagnose_svelte_element_spread_style_directive");
}

#[rstest]
fn diagnose_svelte_element_spread_class_with_bind() {
    assert_compiler("diagnose_svelte_element_spread_class_with_bind");
}

#[rstest]
fn diagnose_svelte_element_static_class_with_directive() {
    assert_compiler("diagnose_svelte_element_static_class_with_directive");
}

#[rstest]
fn diagnose_svelte_element_static_style_with_directive() {
    assert_compiler("diagnose_svelte_element_static_style_with_directive");
}

#[rstest]
fn diagnose_svelte_element_dynamic_class_attr_with_directive() {
    assert_compiler("diagnose_svelte_element_dynamic_class_attr_with_directive");
}

#[rstest]
fn diagnose_svelte_element_style_directive_with_dynamic_attr() {
    assert_compiler("diagnose_svelte_element_style_directive_with_dynamic_attr");
}

#[rstest]
fn diagnose_svelte_element_class_and_style_directives() {
    assert_compiler("diagnose_svelte_element_class_and_style_directives");
}

#[rstest]
fn audit_svelte_element_scoped_no_class() {
    assert_compiler("audit_svelte_element_scoped_no_class");
}

#[rstest]
fn audit_svelte_element_scoped_class_directive() {
    assert_compiler("audit_svelte_element_scoped_class_directive");
}

#[rstest]
fn audit_svelte_element_scoped_static_class_with_directive() {
    assert_compiler("audit_svelte_element_scoped_static_class_with_directive");
}

#[rstest]
fn audit_svelte_element_scoped_dynamic_class_with_directive() {
    assert_compiler("audit_svelte_element_scoped_dynamic_class_with_directive");
}

#[rstest]
fn audit_svelte_element_class_dynamic_alone() {
    assert_compiler("audit_svelte_element_class_dynamic_alone");
}

#[rstest]
fn audit_svelte_element_class_concat_alone() {
    assert_compiler("audit_svelte_element_class_concat_alone");
}

#[rstest]
fn audit_svelte_element_class_concat_with_directive() {
    assert_compiler("audit_svelte_element_class_concat_with_directive");
}

#[rstest]
fn audit_svelte_element_class_array_alone() {
    assert_compiler("audit_svelte_element_class_array_alone");
}

#[rstest]
fn audit_svelte_element_class_array_with_directive() {
    assert_compiler("audit_svelte_element_class_array_with_directive");
}

#[rstest]
fn audit_svelte_element_style_dynamic_alone() {
    assert_compiler("audit_svelte_element_style_dynamic_alone");
}

#[rstest]
fn audit_svelte_element_style_concat_alone() {
    assert_compiler("audit_svelte_element_style_concat_alone");
}

#[rstest]
fn audit_svelte_element_style_dynamic_with_directive() {
    assert_compiler("audit_svelte_element_style_dynamic_with_directive");
}

#[rstest]
fn audit_svelte_element_scoped_style_directive() {
    assert_compiler("audit_svelte_element_scoped_style_directive");
}

#[rstest]
fn audit_svelte_element_class_directive_with_bind() {
    assert_compiler("audit_svelte_element_class_directive_with_bind");
}

#[rstest]
fn audit_svelte_element_class_directive_with_use() {
    assert_compiler("audit_svelte_element_class_directive_with_use");
}

#[rstest]
fn audit_svelte_element_dynamic_attr_with_both_directives() {
    assert_compiler("audit_svelte_element_dynamic_attr_with_both_directives");
}

#[rstest]
fn audit_svelte_element_xmlns_with_class_directive() {
    assert_compiler("audit_svelte_element_xmlns_with_class_directive");
}

#[rstest]
fn audit_svelte_element_static_class_with_style_directive() {
    assert_compiler("audit_svelte_element_static_class_with_style_directive");
}

#[rstest]
fn audit_svelte_element_static_class_with_both_directives() {
    assert_compiler("audit_svelte_element_static_class_with_both_directives");
}

#[rstest]
fn audit_svelte_element_on_legacy_with_class_directive() {
    assert_compiler("audit_svelte_element_on_legacy_with_class_directive");
}

#[rstest]
fn audit_svelte_element_transition_only() {
    assert_compiler("audit_svelte_element_transition_only");
}

#[rstest]
fn audit_svelte_element_spread_with_use() {
    assert_compiler("audit_svelte_element_spread_with_use");
}

#[rstest]
fn diagnose_svelte_element_attribute_effect_call_value_memo() {
    assert_compiler("diagnose_svelte_element_attribute_effect_call_value_memo");
}

#[rstest]
fn diagnose_svelte_element_use_action_with_on_event_legacy() {
    assert_compiler("diagnose_svelte_element_use_action_with_on_event_legacy");
}

#[rstest]
fn audit_svelte_element_style_directive_with_bind() {
    assert_compiler("audit_svelte_element_style_directive_with_bind");
}

#[rstest]
fn audit_svelte_element_class_directive_expression() {
    assert_compiler("audit_svelte_element_class_directive_expression");
}

#[rstest]
fn audit_svelte_element_style_directive_important() {
    assert_compiler("audit_svelte_element_style_directive_important");
}

#[rstest]
fn audit_svelte_element_style_directive_custom_prop() {
    assert_compiler("audit_svelte_element_style_directive_custom_prop");
}

#[rstest]
fn audit_svelte_element_animate_in_each() {
    assert_compiler("audit_svelte_element_animate_in_each");
}

#[rstest]
fn audit_svelte_element_bind_this_store() {
    assert_compiler("audit_svelte_element_bind_this_store");
}

#[rstest]
fn audit_svelte_element_attach_tag() {
    assert_compiler("audit_svelte_element_attach_tag");
}

#[rstest]
fn audit_svelte_element_multiple_class_directives() {
    assert_compiler("audit_svelte_element_multiple_class_directives");
}

#[rstest]
fn audit_svelte_element_multiple_style_directives() {
    assert_compiler("audit_svelte_element_multiple_style_directives");
}

#[rstest]
fn audit_svelte_element_bind_this_member() {
    assert_compiler("audit_svelte_element_bind_this_member");
}

#[rstest]
fn audit_svelte_element_multiple_spreads() {
    assert_compiler("audit_svelte_element_multiple_spreads");
}

#[rstest]
fn audit_svelte_element_transition_with_params() {
    assert_compiler("audit_svelte_element_transition_with_params");
}

#[rstest]
fn audit_svelte_element_transition_local() {
    assert_compiler("audit_svelte_element_transition_local");
}

#[rstest]
fn audit_svelte_element_in_out_separate() {
    assert_compiler("audit_svelte_element_in_out_separate");
}

#[rstest]
fn audit_svelte_element_use_action_with_params() {
    assert_compiler("audit_svelte_element_use_action_with_params");
}

#[rstest]
fn audit_svelte_element_modern_event_handler() {
    assert_compiler("audit_svelte_element_modern_event_handler");
}

#[rstest]
fn audit_svelte_element_async_tag_with_class_directive() {
    assert_compiler("audit_svelte_element_async_tag_with_class_directive");
}

#[rstest]
fn audit_svelte_element_async_tag_with_spread() {
    assert_compiler("audit_svelte_element_async_tag_with_spread");
}

#[rstest]
fn audit_svelte_element_scoped_with_spread() {
    assert_compiler("audit_svelte_element_scoped_with_spread");
}

#[rstest]
fn audit_svelte_element_class_directive_with_call() {
    assert_compiler("audit_svelte_element_class_directive_with_call");
}

#[rstest]
fn audit_svelte_element_dev_spread() {
    assert_compiler("audit_svelte_element_dev_spread");
}

#[rstest]
fn audit_svelte_element_dev_class_directive() {
    assert_compiler("audit_svelte_element_dev_class_directive");
}

#[rstest]
fn audit_svelte_element_transition_in_with_params() {
    assert_compiler("audit_svelte_element_transition_in_with_params");
}

#[rstest]
fn audit_svelte_element_transition_global() {
    assert_compiler("audit_svelte_element_transition_global");
}

#[rstest]
fn audit_svelte_element_class_directive_with_each() {
    assert_compiler("audit_svelte_element_class_directive_with_each");
}

#[rstest]
fn audit_svelte_element_dev_static_class_with_directive() {
    assert_compiler("audit_svelte_element_dev_static_class_with_directive");
}

#[rstest]
fn svelte_element_onclick() {
    assert_compiler("svelte_element_onclick");
}

#[rstest]
fn svelte_element_bind() {
    assert_compiler("svelte_element_bind");
}

#[rstest]
fn svelte_element_null_tag() {
    assert_compiler("svelte_element_null_tag");
}

#[rstest]
fn svelte_element_xmlns() {
    assert_compiler("svelte_element_xmlns");
}

#[rstest]
fn svelte_element_dynamic_xmlns() {
    assert_compiler("svelte_element_dynamic_xmlns");
}

#[rstest]
fn svelte_element_children_expr() {
    assert_compiler("svelte_element_children_expr");
}

#[rstest]
fn svelte_element_body_multi_child_fragment() {
    assert_compiler("svelte_element_body_multi_child_fragment");
}

#[rstest]
fn svelte_body_event_attr() {
    assert_compiler("svelte_body_event_attr");
}

#[rstest]
fn svelte_body_event_legacy() {
    assert_compiler("svelte_body_event_legacy");
}

#[rstest]
fn svelte_body_action() {
    assert_compiler("svelte_body_action");
}

#[rstest]
fn svelte_body_combined() {
    assert_compiler("svelte_body_combined");
}

#[rstest]
fn boundary_basic() {
    assert_compiler("boundary_basic");
}

#[rstest]
fn boundary_failed_snippet() {
    assert_compiler("boundary_failed_snippet");
}

#[rstest]
fn boundary_onerror() {
    assert_compiler("boundary_onerror");
}

#[rstest]
fn boundary_pending_snippet() {
    assert_compiler("boundary_pending_snippet");
}

#[rstest]
fn boundary_failed_onerror() {
    assert_compiler("boundary_failed_onerror");
}

#[rstest]
fn boundary_failed_attribute() {
    assert_compiler("boundary_failed_attribute");
}

#[rstest]
fn boundary_all_three() {
    assert_compiler("boundary_all_three");
}

#[rstest]
fn boundary_reactive_onerror() {
    assert_compiler("boundary_reactive_onerror");
}

#[rstest]
fn boundary_nested() {
    assert_compiler("boundary_nested");
}

#[rstest]
fn boundary_const_tag() {
    assert_compiler("boundary_const_tag");
}

#[rstest]
fn boundary_in_if() {
    assert_compiler("boundary_in_if");
}

#[rstest]
fn boundary_other_snippets() {
    assert_compiler("boundary_other_snippets");
}

#[rstest]
fn boundary_pending_attribute() {
    assert_compiler("boundary_pending_attribute");
}

#[rstest]
fn boundary_pending_imported() {
    assert_compiler("boundary_pending_imported");
}

#[rstest]
fn boundary_failed_attribute_override() {
    assert_compiler("boundary_failed_attribute_override");
}

#[rstest]
fn boundary_pending_attribute_override() {
    assert_compiler("boundary_pending_attribute_override");
}

#[rstest]
fn await_basic() {
    assert_compiler("await_basic");
}

#[rstest]
fn await_short_then() {
    assert_compiler("await_short_then");
}

#[rstest]
fn await_short_catch() {
    assert_compiler("await_short_catch");
}

#[rstest]
fn await_then_catch() {
    assert_compiler("await_then_catch");
}

#[rstest]
fn await_no_bindings() {
    assert_compiler("await_no_bindings");
}

#[rstest]
fn await_pending_only() {
    assert_compiler("await_pending_only");
}

#[rstest]
fn await_destructured() {
    assert_compiler("await_destructured");
}

#[rstest]
fn await_in_if() {
    assert_compiler("await_in_if");
}

#[rstest]
fn await_in_each() {
    assert_compiler("await_in_each");
}

#[rstest]
fn await_reactive() {
    assert_compiler("await_reactive");
}

#[rstest]
fn await_nested_content() {
    assert_compiler("await_nested_content");
}

// ---------------------------------------------------------------------------
// Event attribute tests (Svelte 5)
// ---------------------------------------------------------------------------

#[rstest]
fn event_attr_non_delegatable() {
    assert_compiler("event_attr_non_delegatable");
}

#[rstest]
fn event_attr_delegated_after_non_delegated_order() {
    assert_compiler("event_attr_delegated_after_non_delegated_order");
}

#[rstest]
fn event_attr_capture() {
    assert_compiler("event_attr_capture");
}

#[rstest]
fn event_attr_capture_non_deleg() {
    assert_compiler("event_attr_capture_non_deleg");
}

#[rstest]
fn event_attr_gotpointercapture() {
    assert_compiler("event_attr_gotpointercapture");
}

#[rstest]
fn event_attr_passive() {
    assert_compiler("event_attr_passive");
}

#[rstest]
fn event_attr_passive_window() {
    assert_compiler("event_attr_passive_window");
}

#[rstest]
fn event_attr_import_handler() {
    assert_compiler("event_attr_import_handler");
}

#[rstest]
fn event_attr_member_handler() {
    assert_compiler("event_attr_member_handler");
}

#[rstest]
fn event_attr_const_tag_destructure() {
    assert_compiler("event_attr_const_tag_destructure");
}

#[rstest]
fn event_attr_props_handler() {
    assert_compiler("event_attr_props_handler");
}

#[rstest]
fn event_attr_has_call() {
    assert_compiler("event_attr_has_call");
}

#[rstest]
fn event_attr_dev_apply() {
    assert_compiler("event_attr_dev_apply");
}

// ---------------------------------------------------------------------------
// Expression memoization tests
// ---------------------------------------------------------------------------

#[rstest]
fn component_prop_has_call() {
    assert_compiler("component_prop_has_call");
}

#[rstest]
fn component_prop_has_call_multi() {
    assert_compiler("component_prop_has_call_multi");
}

#[rstest]
fn component_prop_has_call_mixed() {
    assert_compiler("component_prop_has_call_mixed");
}

#[rstest]
fn component_prop_concat_call_memo() {
    assert_compiler("component_prop_concat_call_memo");
}

#[rstest]
fn diagnose_component_prop_concat_legacy_ternary() {
    assert_compiler("diagnose_component_prop_concat_legacy_ternary");
}

#[rstest]
fn component_prop_concat_call_with_literal() {
    assert_compiler("component_prop_concat_call_with_literal");
}

#[rstest]
fn component_prop_concat_import_identifier() {
    assert_compiler("component_prop_concat_import_identifier");
}

#[rstest]
fn component_prop_concat_import_and_call() {
    assert_compiler("component_prop_concat_import_and_call");
}

#[rstest]
fn html_concat_call_with_literal() {
    assert_compiler("html_concat_call_with_literal");
}

#[rstest]
fn html_class_concat_call_with_literal() {
    assert_compiler("html_class_concat_call_with_literal");
}

#[rstest]
fn component_dynamic_dotted() {
    assert_compiler("component_dynamic_dotted");
}

#[rstest]
fn component_prop_memo_state() {
    assert_compiler("component_prop_memo_state");
}

#[rstest]
fn component_prop_const_call_init_getter() {
    assert_compiler("component_prop_const_call_init_getter");
}

#[rstest]
fn component_prop_const_member_init_getter() {
    assert_compiler("component_prop_const_member_init_getter");
}

#[rstest]
fn component_prop_imported_direct_getter() {
    assert_compiler("component_prop_imported_direct_getter");
}

#[rstest]
fn component_prop_import_meta_getter() {
    assert_compiler("component_prop_import_meta_getter");
}

#[rstest]
fn element_attr_import_meta_template_effect() {
    assert_compiler("element_attr_import_meta_template_effect");
}

#[rstest]
fn render_tag_arg_has_call() {
    assert_compiler("render_tag_arg_has_call");
}

#[rstest]
fn render_tag_arg_has_call_multi() {
    assert_compiler("render_tag_arg_has_call_multi");
}

#[rstest]
fn render_tag_arg_mixed() {
    assert_compiler("render_tag_arg_mixed");
}

#[rstest]
fn render_tag_dynamic_prop() {
    assert_compiler("render_tag_dynamic_prop");
}

#[rstest]
fn render_tag_dynamic_state() {
    assert_compiler("render_tag_dynamic_state");
}

#[rstest]
fn render_tag_dynamic_snippet_param() {
    assert_compiler("render_tag_dynamic_snippet_param");
}

#[rstest]
fn render_tag_optional() {
    assert_compiler("render_tag_optional");
}

#[rstest]
fn render_tag_optional_dynamic() {
    assert_compiler("render_tag_optional_dynamic");
}

// ---------------------------------------------------------------------------
// $inspect rune tests
// ---------------------------------------------------------------------------

#[rstest]
fn inspect_basic() {
    assert_compiler("inspect_basic");
}

#[rstest]
fn inspect_with_callback() {
    assert_compiler("inspect_with_callback");
}

#[rstest]
fn inspect_prod_strip() {
    assert_compiler("inspect_prod_strip");
}

// ---------------------------------------------------------------------------
// $inspect.trace() rune tests
// ---------------------------------------------------------------------------

#[rstest]
fn inspect_trace_basic() {
    assert_compiler("inspect_trace_basic");
}

#[rstest]
fn inspect_trace_contexts() {
    assert_compiler("inspect_trace_contexts");
}

#[rstest]
fn inspect_trace_prod_strip() {
    assert_compiler("inspect_trace_prod_strip");
}

#[rstest]
fn inspect_trace_reactive_contexts() {
    assert_compiler("inspect_trace_reactive_contexts");
}

// ---------------------------------------------------------------------------
// $props.id() rune tests
// ---------------------------------------------------------------------------

#[rstest]
fn props_id_basic() {
    assert_compiler("props_id_basic");
}

#[rstest]
fn props_id_with_props() {
    assert_compiler("props_id_with_props");
}

// ---------------------------------------------------------------------------
// {@debug} tests
// ---------------------------------------------------------------------------

#[rstest]
fn debug_basic() {
    assert_compiler("debug_basic");
}

#[rstest]
fn debug_in_blocks() {
    assert_compiler("debug_in_blocks");
}

// ---------------------------------------------------------------------------
// TypeScript stripping tests
// ---------------------------------------------------------------------------

#[rstest]
fn ts_strip_expression_tag() {
    assert_compiler("ts_strip_expression_tag");
}

#[rstest]
fn ts_strip_satisfies() {
    assert_compiler("ts_strip_satisfies");
}

#[rstest]
fn ts_strip_non_null() {
    assert_compiler("ts_strip_non_null");
}

#[rstest]
fn ts_strip_const_tag() {
    assert_compiler("ts_strip_const_tag");
}

#[rstest]
fn ts_strip_attribute() {
    assert_compiler("ts_strip_attribute");
}

#[rstest]
fn ts_strip_script_types() {
    assert_compiler("ts_strip_script_types");
}

#[rstest]
fn ts_strip_handler_param_annotation() {
    assert_compiler("ts_strip_handler_param_annotation");
}

#[rstest]
fn ts_strip_as_paren_optional_chain() {
    assert_compiler("ts_strip_as_paren_optional_chain");
}

#[rstest]
fn ts_strip_catch_empty_comment_orphan() {
    assert_compiler("ts_strip_catch_empty_comment_orphan");
}

#[rstest]
fn diagnose_ts_cast_const_init_marks_attr_dynamic() {
    assert_compiler("diagnose_ts_cast_const_init_marks_attr_dynamic");
}

#[rstest]
fn namespace_svg() {
    assert_compiler("namespace_svg");
}

#[rstest]
fn namespace_mathml() {
    assert_compiler("namespace_mathml");
}

#[rstest]
fn svg_fragment_ambiguous_a() {
    assert_compiler("svg_fragment_ambiguous_a");
}

#[rstest]
fn svg_fragment_ambiguous_title() {
    assert_compiler("svg_fragment_ambiguous_title");
}

#[rstest]
fn svelte_element_in_if() {
    assert_compiler("svelte_element_in_if");
}

#[rstest]
fn svelte_element_class_directive() {
    assert_compiler("svelte_element_class_directive");
}

#[rstest]
fn svelte_element_style_directive() {
    assert_compiler("svelte_element_style_directive");
}

#[rstest]
fn svelte_element_dev_invalid_tag() {
    assert_compiler("svelte_element_dev_invalid_tag");
}

#[rstest]
fn svelte_element_dev_void_children() {
    assert_compiler("svelte_element_dev_void_children");
}

#[rstest]
fn boundary_const_in_snippet() {
    assert_compiler("boundary_const_in_snippet");
}

#[rstest]
fn boundary_imported_handler() {
    assert_compiler("boundary_imported_handler");
}

#[rstest]
fn bind_this_sequence() {
    assert_compiler("bind_this_sequence");
}

// ---------------------------------------------------------------------------
// Tier 2b — Template Tags
// ---------------------------------------------------------------------------

#[rstest]
fn await_array_destructured() {
    assert_compiler("await_array_destructured");
}

#[rstest]
fn html_tag_controlled() {
    assert_compiler("html_tag_controlled");
}

#[rstest]
fn html_tag_svg() {
    assert_compiler("html_tag_svg");
}

#[rstest]
fn html_tag_nested_svg() {
    assert_compiler("html_tag_nested_svg");
}

#[rstest]
fn html_tag_nested_mathml() {
    assert_compiler("html_tag_nested_mathml");
}

#[rstest]
fn html_tag_hydration_ignore() {
    assert_compiler("html_tag_hydration_ignore");
}

#[rstest]
fn const_tag_dev() {
    assert_compiler("const_tag_dev");
}

#[rstest]
fn rune_compound_template() {
    assert_compiler("rune_compound_template");
}

#[rstest]
fn store_assign_template() {
    assert_compiler("store_assign_template");
}

#[rstest]
fn store_compound_template() {
    assert_compiler("store_compound_template");
}

#[rstest]
fn store_update_template() {
    assert_compiler("store_update_template");
}

#[rstest]
fn store_deep_mutation() {
    assert_compiler("store_deep_mutation");
}

#[rstest]
fn store_deep_update() {
    assert_compiler("store_deep_update");
}

// ---------------------------------------------------------------------------
// Tier 2c — Bind Directive Edge Cases
// ---------------------------------------------------------------------------

#[rstest]
fn bind_function_value() {
    assert_compiler("bind_function_value");
}

#[rstest]
fn bind_function_checked() {
    assert_compiler("bind_function_checked");
}

#[rstest]
fn bind_use_deferral() {
    assert_compiler("bind_use_deferral");
}

#[rstest]
fn bind_contenteditable_flag() {
    assert_compiler("bind_contenteditable_flag");
}

#[rstest]
fn bind_group_each() {
    assert_compiler("bind_group_each");
}

#[rstest]
fn bind_group_keyed_each() {
    assert_compiler("bind_group_keyed_each");
}

#[rstest]
fn bind_group_nested_each() {
    assert_compiler("bind_group_nested_each");
}

#[rstest]
fn bind_group_value_attr() {
    assert_compiler("bind_group_value_attr");
}

#[rstest]
fn bind_group_value_attr_before_bind() {
    assert_compiler("bind_group_value_attr_before_bind");
}

#[rstest]
fn bind_group_each_legacy_item_member_untrack() {
    assert_compiler("bind_group_each_legacy_item_member_untrack");
}

#[rstest]
fn bind_group_each_var() {
    assert_compiler("bind_group_each_var");
}

#[rstest]
fn bind_group_each_var_keyed() {
    assert_compiler("bind_group_each_var_keyed");
}

#[rstest]
fn each_fallback() {
    assert_compiler("each_fallback");
}

#[rstest]
fn each_keyed_index() {
    assert_compiler("each_keyed_index");
}

#[rstest]
fn diagnose_each_key_legacy_prop_no_rewrite() {
    assert_compiler("diagnose_each_key_legacy_prop_no_rewrite");
}

#[rstest]
fn each_keyed_index_plain_in_body() {
    assert_compiler("each_keyed_index_plain_in_body");
}

#[rstest]
fn each_key_is_index_literal_diagnose() {
    assert_compiler("each_key_is_index_literal_diagnose");
}

#[rstest]
fn each_key_uses_index() {
    assert_compiler("each_key_uses_index");
}

#[rstest]
fn each_key_is_item() {
    assert_compiler("each_key_is_item");
}

#[rstest]
fn each_destructured_obj() {
    assert_compiler("each_destructured_obj");
}

#[rstest]
fn each_destructured_default() {
    assert_compiler("each_destructured_default");
}

#[rstest]
fn each_destructured_array() {
    assert_compiler("each_destructured_array");
}

#[rstest]
fn diagnose_each_rest_only_pattern_binding() {
    assert_compiler("diagnose_each_rest_only_pattern_binding");
}

#[rstest]
fn each_destructured_obj_with_rest() {
    assert_compiler("each_destructured_obj_with_rest");
}

#[rstest]
fn each_destructured_array_with_rest() {
    assert_compiler("each_destructured_array_with_rest");
}

#[rstest]
fn each_destructured_array_rest_only() {
    assert_compiler("each_destructured_array_rest_only");
}

#[rstest]
fn diagnose_legacy_each_component_css_prop_hoist_derived() {
    assert_compiler("diagnose_legacy_each_component_css_prop_hoist_derived");
}

#[rstest]
fn style_attr_object() {
    assert_compiler("style_attr_object");
}

#[rstest]
fn style_attr_dynamic() {
    assert_compiler("style_attr_dynamic");
}

#[rstest]
fn script_jsdoc_comment() {
    assert_compiler("script_jsdoc_comment");
}

#[rstest]
fn svelte_head_title_meta() {
    assert_compiler("svelte_head_title_meta");
}

#[rstest]
fn snippet_ident_conflict_with_script() {
    assert_compiler("snippet_ident_conflict_with_script");
}

#[rstest]
fn debug_non_dev() {
    assert_compiler("debug_non_dev");
}

#[rstest]
fn debug_non_runes_untrack() {
    assert_compiler("debug_non_runes_untrack");
}

#[rstest]
fn non_runes_simple_snapshot() {
    assert_compiler("non_runes_simple_snapshot");
}

#[rstest]
fn animate_with_spread() {
    assert_compiler("animate_with_spread");
}

#[rstest]
fn svelte_element_static_class_attr() {
    assert_compiler("svelte_element_static_class_attr");
}

#[rstest]
fn root_with_special_elements() {
    assert_compiler("root_with_special_elements");
}

#[rstest]
fn needs_context_method_chain() {
    assert_compiler("needs_context_method_chain");
}

#[rstest]
fn event_handler_derived_with_class_directives() {
    assert_compiler("event_handler_derived_with_class_directives");
}

#[rstest]
fn event_handler_derived_with_class_object() {
    assert_compiler("event_handler_derived_with_class_object");
}

#[rstest]
fn diagnose_class_directive_legacy_event_handler_derived_order() {
    assert_compiler("diagnose_class_directive_legacy_event_handler_derived_order");
}

#[rstest]
fn diagnose_class_directive_named_slot_let_classes_order_legacy() {
    assert_compiler("diagnose_class_directive_named_slot_let_classes_order_legacy");
}

#[rstest]
fn derived_inside_function() {
    assert_compiler("derived_inside_function");
}

#[rstest]
fn derived_nested_getter() {
    assert_compiler("derived_nested_getter");
}

#[rstest]
fn derived_shorthand_property() {
    assert_compiler("derived_shorthand_property");
}

#[rstest]
fn state_inside_function() {
    assert_compiler("state_inside_function");
}

#[rstest]
fn derived_by_inside_function() {
    assert_compiler("derived_by_inside_function");
}

#[rstest]
fn component_snippet_prop() {
    assert_compiler("component_snippet_prop");
}

#[rstest]
fn component_snippet_with_children() {
    assert_compiler("component_snippet_with_children");
}

#[rstest]
fn component_multiple_snippets() {
    assert_compiler("component_multiple_snippets");
}

#[rstest]
fn component_snippet_only() {
    assert_compiler("component_snippet_only");
}

// ---------------------------------------------------------------------------
// Diagnose: TypeScript import + spread + bind:prop tests
// ---------------------------------------------------------------------------

#[rstest]
fn ts_type_import_comment() {
    assert_compiler("ts_type_import_comment");
}

#[rstest]
fn rest_props_member_access() {
    assert_compiler("rest_props_member_access");
}

#[rstest]
fn component_spread_props() {
    assert_compiler("component_spread_props");
}

#[rstest]
fn component_bind_prop_forward() {
    assert_compiler("component_bind_prop_forward");
}

#[rstest]
fn component_bind_member_path() {
    assert_compiler("component_bind_member_path");
}

#[rstest]
fn component_bind_member_path_bindable_root() {
    assert_compiler("component_bind_member_path_bindable_root");
}

#[rstest]
fn component_bind_member_path_dev() {
    assert_compiler("component_bind_member_path_dev");
}

#[rstest]
fn component_bind_function() {
    assert_compiler("component_bind_function");
}

#[rstest]
fn component_bind_function_anchor_order() {
    assert_compiler("component_bind_function_anchor_order");
}

#[rstest]
fn diagnose_component_bind_function_props_position() {
    assert_compiler("diagnose_component_bind_function_props_position");
}

#[rstest]
fn diagnose_component_bind_derived_target_no_proxy_flag() {
    assert_compiler("diagnose_component_bind_derived_target_no_proxy_flag");
}

#[rstest]
fn diagnose_fragment_id_after_component_with_snippet() {
    assert_compiler("diagnose_fragment_id_after_component_with_snippet");
}

#[rstest]
fn diagnose_fragment_id_in_snippet_used_as_expression() {
    assert_compiler("diagnose_fragment_id_in_snippet_used_as_expression");
}

#[rstest]
fn diagnose_fragment_id_in_sibling_named_slot_after_component() {
    assert_compiler("diagnose_fragment_id_in_sibling_named_slot_after_component");
}

#[rstest]
fn component_prop_const_tag_member() {
    assert_compiler("component_prop_const_tag_member");
}

#[rstest]
fn diagnose_component_prop_const_tag_member_init() {
    assert_compiler("diagnose_component_prop_const_tag_member_init");
}

#[rstest]
fn diagnose_component_prop_const_tag_destructured_shorthand() {
    assert_compiler("diagnose_component_prop_const_tag_destructured_shorthand");
}

#[rstest]
fn diagnose_each_const_tag_shorthand_prop_to_component() {
    assert_compiler("diagnose_each_const_tag_shorthand_prop_to_component");
}

#[rstest]
fn diagnose_const_tag_legacy_dependency_destructure() {
    assert_compiler("diagnose_const_tag_legacy_dependency_destructure");
}

#[rstest]
fn diagnose_const_tag_legacy_dep_read_spread_of_derived() {
    assert_compiler("diagnose_const_tag_legacy_dep_read_spread_of_derived");
}

#[rstest]
fn diagnose_legacy_slot_let_array_destructure_dep_read() {
    assert_compiler("diagnose_legacy_slot_let_array_destructure_dep_read");
}

#[rstest]
fn diagnose_svelte_fragment_let_inside_named_slot_component() {
    assert_compiler("diagnose_svelte_fragment_let_inside_named_slot_component");
}

#[rstest]
fn diagnose_svelte_component_css_custom_prop_wrapper() {
    assert_compiler("diagnose_svelte_component_css_custom_prop_wrapper");
}

#[rstest]
fn diagnose_component_prop_computed_member_getter() {
    assert_compiler("diagnose_component_prop_computed_member_getter");
}

#[rstest]
fn diagnose_component_prop_hyphenated_key_derived() {
    assert_compiler("diagnose_component_prop_hyphenated_key_derived");
}

// ---------------------------------------------------------------------------
// Diagnose: svelte import patterns
// ---------------------------------------------------------------------------

#[rstest]
fn needs_context_nested_fn() {
    assert_compiler("needs_context_nested_fn");
}

#[rstest]
fn member_expr_dynamic_local() {
    assert_compiler("member_expr_dynamic_local");
}

#[rstest]
fn import_type_mixed() {
    assert_compiler("import_type_mixed");
}

#[rstest]
fn derived_in_nested_function() {
    assert_compiler("derived_in_nested_function");
}

#[rstest]
fn derived_local_signal_get() {
    assert_compiler("derived_local_signal_get");
}

#[rstest]
fn svelte_element_duplicate_naming() {
    assert_compiler("svelte_element_duplicate_naming");
}

#[rstest]
fn each_block_no_item() {
    assert_compiler("each_block_no_item");
}

#[rstest]
fn each_block_no_item_multi() {
    assert_compiler("each_block_no_item_multi");
}

#[rstest]
fn each_block_no_item_with_index() {
    assert_compiler("each_block_no_item_with_index");
}

#[rstest]
fn each_collection_call_reads_state() {
    assert_compiler("each_collection_call_reads_state");
}

#[rstest]
fn legacy_each_collection_member_const_wraps_untrack() {
    assert_compiler("legacy_each_collection_member_const_wraps_untrack");
}

#[rstest]
fn legacy_each_collection_member_reactive_let_wraps_with_read() {
    assert_compiler("legacy_each_collection_member_reactive_let_wraps_with_read");
}

#[rstest]
fn diagnose_legacy_each_collection_member_nullish_fallback_wraps_with_read() {
    assert_compiler("diagnose_legacy_each_collection_member_nullish_fallback_wraps_with_read");
}

#[rstest]
fn diagnose_legacy_each_collection_member_outer_each_item_wraps() {
    assert_compiler("diagnose_legacy_each_collection_member_outer_each_item_wraps");
}

#[rstest]
fn diagnose_legacy_each_collection_member_export_let_prop_wraps() {
    assert_compiler("diagnose_legacy_each_collection_member_export_let_prop_wraps");
}

#[rstest]
fn diagnose_legacy_each_collection_member_imported_wraps_with_read() {
    assert_compiler("diagnose_legacy_each_collection_member_imported_wraps_with_read");
}

#[rstest]
fn diagnose_legacy_each_call_imported_wraps() {
    assert_compiler("diagnose_legacy_each_call_imported_wraps");
}

#[rstest]
fn diagnose_legacy_each_call_imported_args_wrap() {
    assert_compiler("diagnose_legacy_each_call_imported_args_wrap");
}

#[rstest]
fn diagnose_legacy_each_call_local_fn_wraps() {
    assert_compiler("diagnose_legacy_each_call_local_fn_wraps");
}

#[rstest]
fn diagnose_legacy_each_collection_new_expression_wraps() {
    assert_compiler("diagnose_legacy_each_collection_new_expression_wraps");
}

#[rstest]
fn diagnose_legacy_each_collection_new_expression_prop_arg_wraps() {
    assert_compiler("diagnose_legacy_each_collection_new_expression_prop_arg_wraps");
}

#[rstest]
fn async_if_basic() {
    assert_compiler("async_if_basic");
}

#[test]
fn async_if_else_if_condition() {
    assert_compiler("async_if_else_if_condition");
}

#[test]
fn if_elseif_new_blockers() {
    assert_compiler("if_elseif_new_blockers");
}

#[rstest]
fn async_each_basic() {
    assert_compiler("async_each_basic");
}

#[rstest]
fn async_html_basic() {
    assert_compiler("async_html_basic");
}

#[rstest]
fn async_key_basic() {
    assert_compiler("async_key_basic");
}

#[rstest]
fn async_await_has_await() {
    assert_compiler("async_await_has_await");
}

#[rstest]
fn async_flag_import() {
    assert_compiler("async_flag_import");
}

#[rstest]
fn async_blockers_basic() {
    assert_compiler("async_blockers_basic");
}

#[rstest]
fn async_bind_basic() {
    assert_compiler("async_bind_basic");
}

#[rstest]
fn action_blockers() {
    assert_compiler("action_blockers");
}

#[rstest]
fn attach_blockers() {
    assert_compiler("attach_blockers");
}

#[rstest]
fn transition_blockers() {
    assert_compiler("transition_blockers");
}

#[rstest]
fn animate_blockers() {
    assert_compiler("animate_blockers");
}

#[rstest]
fn async_svelte_element() {
    assert_compiler("async_svelte_element");
}

#[rstest]
fn async_const_tag() {
    assert_compiler("async_const_tag");
}

#[rstest]
fn async_derived_basic() {
    assert_compiler("async_derived_basic");
}

#[rstest]
fn async_derived_destructured() {
    assert_compiler("async_derived_destructured");
}

#[rstest]
fn async_derived_dev() {
    assert_compiler("async_derived_dev");
}

#[rstest]
fn async_derived_dev_ignored() {
    assert_compiler("async_derived_dev_ignored");
}

#[rstest]
fn async_derived_dev_ignored_destructured() {
    assert_compiler("async_derived_dev_ignored_destructured");
}

#[rstest]
fn async_derived_nested_function() {
    assert_compiler("async_derived_nested_function");
}

#[rstest]
fn async_derived_nested_function_destructured() {
    assert_compiler("async_derived_nested_function_destructured");
}

#[rstest]
fn async_for_await_dev() {
    assert_compiler("async_for_await_dev");
}

#[rstest]
fn orthogonality_heavy_call() {
    assert_compiler("orthogonality_heavy_call");
}

#[rstest]
fn orthogonality_async_await() {
    assert_compiler("orthogonality_async_await");
}

#[rstest]
fn orthogonality_heavy_async_await_call() {
    assert_compiler("orthogonality_heavy_async_await_call");
}

#[rstest]
fn inline_await_basic() {
    assert_compiler("inline_await_basic");
}

#[rstest]
fn inline_await_global_callee() {
    assert_compiler("inline_await_global_callee");
}

#[rstest]
fn inline_await_text_concat() {
    assert_compiler("inline_await_text_concat");
}

#[rstest]
fn inline_await_attr() {
    assert_compiler("inline_await_attr");
}

#[test]
fn async_pickled_await_template() {
    assert_compiler("async_pickled_await_template");
}

#[rstest]
fn async_render_tag() {
    assert_compiler("async_render_tag");
}

#[rstest]
fn async_render_tag_complex_args() {
    assert_compiler("async_render_tag_complex_args");
}

#[rstest]
fn async_boundary_const() {
    assert_compiler("async_boundary_const");
}

#[rstest]
fn async_const_derived_chain() {
    assert_compiler("async_const_derived_chain");
}

#[rstest]
fn attach_on_component() {
    assert_compiler("attach_on_component");
}

#[rstest]
fn attach_on_component_dynamic() {
    assert_compiler("attach_on_component_dynamic");
}

#[rstest]
fn attach_on_document() {
    assert_compiler("attach_on_document");
}

#[rstest]
fn each_keyed_destructure() {
    assert_compiler("each_keyed_destructure");
}

#[rstest]
fn await_then_text_before_element() {
    assert_compiler("await_then_text_before_element");
}

#[rstest]
fn await_thunk_optimization() {
    assert_compiler("await_thunk_optimization");
}

#[rstest]
fn await_each_nested() {
    assert_compiler("await_each_nested");
}

#[rstest]
fn await_pending_then() {
    assert_compiler("await_pending_then");
}

#[rstest]
fn await_pending_catch() {
    assert_compiler("await_pending_catch");
}

#[rstest]
fn await_short_catch_no_binding() {
    assert_compiler("await_short_catch_no_binding");
}

#[rstest]
fn await_nested_await() {
    assert_compiler("await_nested_await");
}

#[rstest]
fn fragment_counter_with_nested_if() {
    assert_compiler("fragment_counter_with_nested_if");
}

#[rstest]
fn bind_group_radio_basic() {
    assert_compiler("bind_group_radio_basic");
}

#[rstest]
fn bind_multiple_on_element() {
    assert_compiler("bind_multiple_on_element");
}

#[rstest]
fn if_else_chain_with_const() {
    assert_compiler("if_else_chain_with_const");
}

#[rstest]
fn event_mixed_delegation() {
    assert_compiler("event_mixed_delegation");
}

#[rstest]
fn props_identifier_basic() {
    assert_compiler("props_identifier_basic");
}

#[rstest]
fn props_identifier_await_expression() {
    assert_compiler("props_identifier_await_expression");
}

#[rstest]
fn call_expr_local_method_dynamic() {
    assert_compiler("call_expr_local_method_dynamic");
}

#[rstest]
fn call_expr_nested_fn_dynamic() {
    assert_compiler("call_expr_nested_fn_dynamic");
}

#[rstest]
fn effect_cleanup_return() {
    assert_compiler("effect_cleanup_return");
}

#[rstest]
fn tag_derived_basic() {
    assert_compiler("tag_derived_basic");
}

#[rstest]
fn tag_derived_by() {
    assert_compiler("tag_derived_by");
}

#[rstest]
fn derived_destructured_object() {
    assert_compiler("derived_destructured_object");
}

#[rstest]
fn derived_destructured_object_prop_source() {
    assert_compiler("derived_destructured_object_prop_source");
}

#[rstest]
fn derived_destructured_props_whole_source() {
    assert_compiler("derived_destructured_props_whole_source");
}

#[rstest]
fn derived_destructured_array() {
    assert_compiler("derived_destructured_array");
}

#[rstest]
fn derived_destructured_by() {
    assert_compiler("derived_destructured_by");
}

#[rstest]
fn tag_state_unmutated() {
    assert_compiler("tag_state_unmutated");
}

#[rstest]
fn tag_state_unmutated_no_proxy() {
    assert_compiler("tag_state_unmutated_no_proxy");
}

#[rstest]
fn tag_bindable_proxy() {
    assert_compiler("tag_bindable_proxy");
}

#[rstest]
fn tag_class_field_public() {
    assert_compiler("tag_class_field_public");
}

#[rstest]
fn tag_class_field_private() {
    assert_compiler("tag_class_field_private");
}

#[rstest]
fn tag_class_constructor() {
    assert_compiler("tag_class_constructor");
}

#[rstest]
fn tag_snippet_dev() {
    assert_compiler("tag_snippet_dev");
}

#[rstest]
fn tag_render_dev() {
    assert_compiler("tag_render_dev");
}

#[rstest]
fn snippet_destructure_dev() {
    assert_compiler("snippet_destructure_dev");
}

#[rstest]
fn snippet_object_destructure() {
    assert_compiler("snippet_object_destructure");
}

#[rstest]
fn diagnose_legacy_snippet_param_member_to_component_prop() {
    assert_compiler("diagnose_legacy_snippet_param_member_to_component_prop");
}

#[rstest]
fn diagnose_legacy_component_prop_import_meta_getter_in_if() {
    assert_compiler("diagnose_legacy_component_prop_import_meta_getter_in_if");
}

#[rstest]
fn diagnose_snippet_inside_each_callback() {
    assert_compiler("diagnose_snippet_inside_each_callback");
}

#[rstest]
fn snippet_array_destructure() {
    assert_compiler("snippet_array_destructure");
}

#[rstest]
fn snippet_mixed_params() {
    assert_compiler("snippet_mixed_params");
}

#[rstest]
fn snippet_nested_destructure() {
    assert_compiler("snippet_nested_destructure");
}

#[rstest]
fn snippet_computed_key_destructure() {
    assert_compiler("snippet_computed_key_destructure");
}

#[rstest]
fn snippet_body_single_component() {
    assert_compiler("snippet_body_single_component");
}

#[rstest]
fn tag_state_destructured_array() {
    assert_compiler("tag_state_destructured_array");
}

#[rstest]
fn tag_state_destructured_object() {
    assert_compiler("tag_state_destructured_object");
}

#[rstest]
fn state_var_safe_get() {
    assert_compiler("state_var_safe_get");
}

#[rstest]
fn state_assign_dev() {
    assert_compiler("state_assign_dev");
}

#[rstest]
fn css_scoped_class_selector() {
    assert_compiler("css_scoped_class_selector");
}

#[rstest]
fn bind_select_static_option_value() {
    assert_compiler("bind_select_static_option_value");
}

#[rstest]
fn input_dynamic_special_attrs() {
    assert_compiler("input_dynamic_special_attrs");
}

#[rstest]
fn svg_dynamic_special_attrs() {
    assert_compiler("svg_dynamic_special_attrs");
}

#[rstest]
fn each_index_text_no_coalesce() {
    assert_compiler("each_index_text_no_coalesce");
}

#[rstest]
fn snippet_destructure_default_state_ref() {
    assert_compiler("snippet_destructure_default_state_ref");
}

#[rstest]
fn snippet_destructure_default_mutated_state_ref() {
    assert_compiler("snippet_destructure_default_mutated_state_ref");
}

#[rstest]
fn diagnose_props_bindable_icon_component() {
    assert_compiler("diagnose_props_bindable_icon_component");
}

#[rstest]
fn diagnose_props_identifier_in_snippet_body() {
    assert_compiler("diagnose_props_identifier_in_snippet_body");
}

#[rstest]
fn diagnose_filename_name_collides_with_ts_type_import() {
    assert_compiler("diagnose_filename_name_collides_with_ts_type_import");
}

#[rstest]
fn props_bindable_checkbox_disabled_shorthand_ts() {
    assert_compiler("props_bindable_checkbox_disabled_shorthand_ts");
}

#[rstest]
fn diagnose_component_onclick_state() {
    assert_compiler("diagnose_component_onclick_state");
}

#[rstest]
fn diagnose_svg_city_icon() {
    assert_compiler("diagnose_svg_city_icon");
}

#[rstest]
fn clock_svg_derived_onmount() {
    assert_compiler("clock_svg_derived_onmount");
}

#[rstest]
fn diagnose_component_default_and_named_slot_expr() {
    assert_compiler("diagnose_component_default_and_named_slot_expr");
}

#[rstest]
fn diagnose_component_named_slot_child_with_expression_prop() {
    assert_compiler("diagnose_component_named_slot_child_with_expression_prop");
}

#[rstest]
fn diagnose_component_named_slot_child_inflates_template_root_ids() {
    assert_compiler("diagnose_component_named_slot_child_inflates_template_root_ids");
}

#[rstest]
fn diagnose_component_named_slot_empty_element_child_kept() {
    assert_compiler("diagnose_component_named_slot_empty_element_child_kept");
}

#[rstest]
fn diagnose_component_named_slot_svelte_component_child_kept() {
    assert_compiler("diagnose_component_named_slot_svelte_component_child_kept");
}

#[rstest]
fn diagnose_svelte_component_named_slot_expression_binds() {
    assert_compiler("diagnose_svelte_component_named_slot_expression_binds");
}

#[rstest]
fn diagnose_legacy_slot_forward_inflates_template_root_ids() {
    assert_compiler("diagnose_legacy_slot_forward_inflates_template_root_ids");
}

#[rstest]
fn diagnose_legacy_dev_benchmark() {
    assert_compiler("diagnose_legacy_dev_benchmark");
}

#[rstest]
fn component_dev_default_children_wrap_snippet() {
    assert_compiler("component_dev_default_children_wrap_snippet");
}

#[rstest]
fn component_dev_named_slot_no_wrap_snippet() {
    assert_compiler("component_dev_named_slot_no_wrap_snippet");
}

#[rstest]
fn diagnose_dev_benchmark() {
    assert_compiler("diagnose_dev_benchmark");
}

#[rstest]
fn dev_binary_equals_wrap() {
    assert_compiler("dev_binary_equals_wrap");
}

#[rstest]
fn dev_filename_relative_to_root_dir() {
    assert_compiler("dev_filename_relative_to_root_dir");
}

#[rstest]
fn dev_filename_root_dir_no_match() {
    assert_compiler("dev_filename_root_dir_no_match");
}

#[rstest]
fn add_locations_svelte_head_skips_hoisted_title() {
    assert_compiler("add_locations_svelte_head_skips_hoisted_title");
}

#[rstest]
fn add_locations_named_slot_wrapper() {
    assert_compiler("add_locations_named_slot_wrapper");
}

#[rstest]
fn auto_softlegacy_member_read() {
    assert_compiler("auto_softlegacy_member_read");
}

#[rstest]
fn auto_hardlegacy_member_read_explicit() {
    assert_compiler("auto_hardlegacy_member_read_explicit");
}

#[rstest]
fn auto_hardlegacy_store_autosub_shadows_rune() {
    assert_compiler("auto_hardlegacy_store_autosub_shadows_rune");
}

#[rstest]
fn auto_hardlegacy_import_call_coarse_wrap() {
    assert_compiler("auto_hardlegacy_import_call_coarse_wrap");
}

#[rstest]
fn legacy_dev_inspect_fallback() {
    assert_compiler("legacy_dev_inspect_fallback");
}

#[rstest]
fn legacy_dev_const_deep_read_wrap() {
    assert_compiler("legacy_dev_const_deep_read_wrap");
}

#[rstest]
fn legacy_dev_each_const_deep_read() {
    assert_compiler("legacy_dev_each_const_deep_read");
}

#[rstest]
fn legacy_dev_attribute_effect_grouping() {
    assert_compiler("legacy_dev_attribute_effect_grouping");
}

#[rstest]
fn legacy_dev_component_event_prop_derived() {
    assert_compiler("legacy_dev_component_event_prop_derived");
}

#[rstest]
fn legacy_dev_component_prop_getter() {
    assert_compiler("legacy_dev_component_prop_getter");
}

#[rstest]
fn auto_softlegacy_simple_template() {
    assert_compiler("auto_softlegacy_simple_template");
}

#[rstest]
fn auto_softlegacy_const_only() {
    assert_compiler("auto_softlegacy_const_only");
}

#[rstest]
fn auto_softlegacy_const_member_mutation() {
    assert_compiler("auto_softlegacy_const_member_mutation");
}

#[rstest]
fn legacy_state_bind_member_mutate_wrap() {
    assert_compiler("legacy_state_bind_member_mutate_wrap");
}

#[rstest]
fn auto_softlegacy_component_prop_derived() {
    assert_compiler("auto_softlegacy_component_prop_derived");
}

#[rstest]
fn auto_softlegacy_render_arg_memo() {
    assert_compiler("auto_softlegacy_render_arg_memo");
}

#[rstest]
fn auto_softlegacy_template_call_expr() {
    assert_compiler("auto_softlegacy_template_call_expr");
}

#[rstest]
fn needs_context_template_member_in_binary() {
    assert_compiler("needs_context_template_member_in_binary");
}

#[rstest]
fn needs_context_template_member_in_conditional() {
    assert_compiler("needs_context_template_member_in_conditional");
}

#[rstest]
fn needs_context_template_member_root_import() {
    assert_compiler("needs_context_template_member_root_import");
}

#[rstest]
fn needs_context_template_member_ts_props() {
    assert_compiler("needs_context_template_member_ts_props");
}

#[rstest]
fn diagnose_component_bind_group_emits_array() {
    assert_compiler("diagnose_component_bind_group_emits_array");
}

#[rstest]
fn diagnose_html_template_preserves_nbsp() {
    assert_compiler("diagnose_html_template_preserves_nbsp");
}

#[rstest]
fn diagnose_attribute_entity_decoding() {
    assert_compiler("diagnose_attribute_entity_decoding");
}

#[rstest]
fn diagnose_attribute_entity_strict_mode() {
    assert_compiler("diagnose_attribute_entity_strict_mode");
}

#[rstest]
fn diagnose_component_onclick_const_arrow() {
    assert_compiler("diagnose_component_onclick_const_arrow");
}

#[rstest]
fn diagnose_component_spread_call_memo() {
    assert_compiler("diagnose_component_spread_call_memo");
}

#[rstest]
fn diagnose_button_single_dynamic_text() {
    assert_compiler("diagnose_button_single_dynamic_text");
}

#[rstest]
fn diagnose_text_concat_import_uses_template_effect() {
    assert_compiler("diagnose_text_concat_import_uses_template_effect");
}

#[rstest]
fn diagnose_text_concat_sequence_expr_nullish_fallback() {
    assert_compiler("diagnose_text_concat_sequence_expr_nullish_fallback");
}

#[rstest]
fn diagnose_static_text_before_dynamic_in_element() {
    assert_compiler("diagnose_static_text_before_dynamic_in_element");
}

#[rstest]
fn diagnose_snippet_inside_if_consequent() {
    assert_compiler("diagnose_snippet_inside_if_consequent");
}

#[rstest]
fn diagnose_snippet_hoistable_with_script_import() {
    assert_compiler("diagnose_snippet_hoistable_with_script_import");
}

#[rstest]
fn diagnose_snippet_hoistable_param_type_annotation() {
    assert_compiler("diagnose_snippet_hoistable_param_type_annotation");
}

#[rstest]
fn diagnose_snippet_name_with_underscore() {
    assert_compiler("diagnose_snippet_name_with_underscore");
}

#[rstest]
fn diagnose_hoisted_snippet_module_order_with_sibling_template() {
    assert_compiler("diagnose_hoisted_snippet_module_order_with_sibling_template");
}

#[rstest]
fn diagnose_snippet_store_autosub_not_hoistable() {
    assert_compiler("diagnose_snippet_store_autosub_not_hoistable");
}

#[rstest]
fn diagnose_component_on_directive_shorthand_forward() {
    assert_compiler("diagnose_component_on_directive_shorthand_forward");
}

#[rstest]
fn diagnose_legacy_pre_effect_order_after_functions() {
    assert_compiler("diagnose_legacy_pre_effect_order_after_functions");
}

#[rstest]
fn diagnose_legacy_template_effect_prop_call_coarse_wrap() {
    assert_compiler("diagnose_legacy_template_effect_prop_call_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_local_var_not_promoted_to_state() {
    assert_compiler("diagnose_legacy_local_var_not_promoted_to_state");
}

#[rstest]
fn diagnose_legacy_reactive_assignment_promotes_state_via_handler() {
    assert_compiler("diagnose_legacy_reactive_assignment_promotes_state_via_handler");
}

#[rstest]
fn diagnose_legacy_reactive_arrow_value_promotes_state() {
    assert_compiler("diagnose_legacy_reactive_arrow_value_promotes_state");
}

#[rstest]
fn diagnose_legacy_each_const_destructure_coarse_wrap() {
    assert_compiler("diagnose_legacy_each_const_destructure_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_each_if_condition_coarse_wrap() {
    assert_compiler("diagnose_legacy_each_if_condition_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_each_html_member_coarse_wrap() {
    assert_compiler("diagnose_legacy_each_html_member_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_class_attribute_prop_member_coarse_wrap() {
    assert_compiler("diagnose_legacy_class_attribute_prop_member_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_each_bind_this_indexed_reactive() {
    assert_compiler("diagnose_legacy_each_bind_this_indexed_reactive");
}

#[rstest]
fn diagnose_legacy_each_bind_this_indexed_by_index_variable() {
    assert_compiler("diagnose_legacy_each_bind_this_indexed_by_index_variable");
}

#[rstest]
fn diagnose_legacy_each_store_bind_value_item_member() {
    assert_compiler("diagnose_legacy_each_store_bind_value_item_member");
}

#[rstest]
fn diagnose_legacy_each_store_bind_checked_item_member() {
    assert_compiler("diagnose_legacy_each_store_bind_checked_item_member");
}

#[rstest]
fn diagnose_legacy_each_store_bind_group_item_member() {
    assert_compiler("diagnose_legacy_each_store_bind_group_item_member");
}

#[rstest]
fn diagnose_legacy_each_store_bind_value_element_item_member() {
    assert_compiler("diagnose_legacy_each_store_bind_value_element_item_member");
}

#[rstest]
fn diagnose_legacy_each_store_bind_checked_element_item_member() {
    assert_compiler("diagnose_legacy_each_store_bind_checked_element_item_member");
}

#[rstest]
fn diagnose_video_muted_static_attribute_lowers_to_property() {
    assert_compiler("diagnose_video_muted_static_attribute_lowers_to_property");
}

#[rstest]
fn diagnose_video_muted_concatenation_attribute_lowers_to_property() {
    assert_compiler("diagnose_video_muted_concatenation_attribute_lowers_to_property");
}

#[rstest]
fn diagnose_legacy_if_store_short_circuit_coarse_wrap() {
    assert_compiler("diagnose_legacy_if_store_short_circuit_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_each_index_component_prop_plain() {
    assert_compiler("diagnose_legacy_each_index_component_prop_plain");
}

#[rstest]
fn diagnose_legacy_each_css_props_member_coarse_wrap() {
    assert_compiler("diagnose_legacy_each_css_props_member_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_each_css_props_concat_member_coarse_wrap() {
    assert_compiler("diagnose_legacy_each_css_props_concat_member_coarse_wrap");
}

#[rstest]
fn diagnose_each_css_wrapper_root_order() {
    assert_compiler("diagnose_each_css_wrapper_root_order");
}

#[rstest]
fn diagnose_component_bind_store_derived_base() {
    assert_compiler("diagnose_component_bind_store_derived_base");
}

#[rstest]
fn diagnose_nested_delegated_transition_order() {
    assert_compiler("diagnose_nested_delegated_transition_order");
}

#[rstest]
fn diagnose_text_entity_leading_ws_before_if_block() {
    assert_compiler("diagnose_text_entity_leading_ws_before_if_block");
}

#[rstest]
fn diagnose_td_sibling_if_blocks_whitespace() {
    assert_compiler("diagnose_td_sibling_if_blocks_whitespace");
}

#[rstest]
fn diagnose_legacy_component_prop_const_call_safe_equal_untrack() {
    assert_compiler("diagnose_legacy_component_prop_const_call_safe_equal_untrack");
}

#[rstest]
fn diagnose_legacy_component_prop_call_with_prop_arg_coarse_wrap() {
    assert_compiler("diagnose_legacy_component_prop_call_with_prop_arg_coarse_wrap");
}

#[rstest]
fn diagnose_legacy_component_prop_ternary_arrow_no_coarse_wrap() {
    assert_compiler("diagnose_legacy_component_prop_ternary_arrow_no_coarse_wrap");
}

#[rstest]
fn diagnose_style_directive_literal_skips_template_effect_grouping() {
    assert_compiler("diagnose_style_directive_literal_skips_template_effect_grouping");
}

#[rstest]
fn diagnose_style_directive_quoted_literal_interp_static() {
    assert_compiler("diagnose_style_directive_quoted_literal_interp_static");
}

#[rstest]
fn diagnose_style_directive_template_literal_const_ident_fold() {
    assert_compiler("diagnose_style_directive_template_literal_const_ident_fold");
}

#[rstest]
fn diagnose_style_directive_const_ident_fold() {
    assert_compiler("diagnose_style_directive_const_ident_fold");
}

#[rstest]
fn diagnose_style_directive_const_number_ident_fold() {
    assert_compiler("diagnose_style_directive_const_number_ident_fold");
}

#[rstest]
fn diagnose_style_directive_const_binary_init_ident_fold() {
    assert_compiler("diagnose_style_directive_const_binary_init_ident_fold");
}

#[rstest]
fn diagnose_style_directive_multiple_const_idents_fold() {
    assert_compiler("diagnose_style_directive_multiple_const_idents_fold");
}

#[rstest]
fn diagnose_style_directive_const_ident_fold_runes() {
    assert_compiler("diagnose_style_directive_const_ident_fold_runes");
}

#[rstest]
fn diagnose_style_directive_const_with_state_keeps_reactive() {
    assert_compiler("diagnose_style_directive_const_with_state_keeps_reactive");
}

#[rstest]
fn diagnose_style_directive_legacy_let_no_fold() {
    assert_compiler("diagnose_style_directive_legacy_let_no_fold");
}

#[rstest]
fn diagnose_style_directive_complex_expression_hoists_to_memo_deps() {
    assert_compiler("diagnose_style_directive_complex_expression_hoists_to_memo_deps");
}

#[rstest]
fn diagnose_style_attr_call_groups_in_template_effect_memo() {
    assert_compiler("diagnose_style_attr_call_groups_in_template_effect_memo");
}

#[rstest]
fn diagnose_legacy_template_effect_merge_memo_attr_with_simple_attr() {
    assert_compiler("diagnose_legacy_template_effect_merge_memo_attr_with_simple_attr");
}

#[rstest]
fn attr_call_value_hoists_to_template_effect_memo() {
    assert_compiler("attr_call_value_hoists_to_template_effect_memo");
}

#[rstest]
fn diagnose_attr_call_no_references_init() {
    assert_compiler("diagnose_attr_call_no_references_init");
}

#[rstest]
fn diagnose_style_attr_call_no_references_with_directive_init() {
    assert_compiler("diagnose_style_attr_call_no_references_with_directive_init");
}

#[rstest]
fn diagnose_style_attr_dynamic_with_style_directive_merges_set_style() {
    assert_compiler("diagnose_style_attr_dynamic_with_style_directive_merges_set_style");
}

#[rstest]
fn diagnose_style_attr_with_directive_source_order() {
    assert_compiler("diagnose_style_attr_with_directive_source_order");
}

#[rstest]
fn diagnose_style_directive_with_static_style_attribute() {
    assert_compiler("diagnose_style_directive_with_static_style_attribute");
}

#[rstest]
fn noscript_root_in_if_block() {
    assert_compiler("noscript_root_in_if_block");
}

#[rstest]
fn diagnose_legacy_pre_effect_dep_order_switch_externals() {
    assert_compiler("diagnose_legacy_pre_effect_dep_order_switch_externals");
}

#[rstest]
fn diagnose_legacy_pre_effect_dep_order_lhs_write_before_prop_read() {
    assert_compiler("diagnose_legacy_pre_effect_dep_order_lhs_write_before_prop_read");
}

#[rstest]
fn diagnose_legacy_iife_read_nested_write_promotes_state() {
    assert_compiler("diagnose_legacy_iife_read_nested_write_promotes_state");
}

#[rstest]
fn diagnose_legacy_pre_effect_store_write_read_topological_order() {
    assert_compiler("diagnose_legacy_pre_effect_store_write_read_topological_order");
}

#[rstest]
fn diagnose_css_slot_fallback_descendant_scope() {
    assert_compiler("diagnose_css_slot_fallback_descendant_scope");
}

#[rstest]
fn ts_init_impure_type_assertion() {
    assert_compiler("ts_init_impure_type_assertion");
}

#[rstest]
fn ts_class_field_rune_paren() {
    assert_compiler("ts_class_field_rune_paren");
}

#[rstest]
fn ts_class_field_rune_as() {
    assert_compiler("ts_class_field_rune_as");
}

#[rstest]
fn ts_constructor_this_rune_assignment_ts() {
    assert_compiler("ts_constructor_this_rune_assignment_ts");
}

#[rstest]
fn ts_bindable_default_as() {
    assert_compiler("ts_bindable_default_as");
}

#[rstest]
fn ts_bindable_default_identifier_paren() {
    assert_compiler("ts_bindable_default_identifier_paren");
}

#[rstest]
fn ts_derived_async_await_paren() {
    assert_compiler("ts_derived_async_await_paren");
}

#[rstest]
fn ts_state_proxyable_paren_arrow() {
    assert_compiler("ts_state_proxyable_paren_arrow");
}

#[rstest]
fn ts_simple_expression_paren_default() {
    assert_compiler("ts_simple_expression_paren_default");
}

#[rstest]
fn ts_each_key_is_index_paren() {
    assert_compiler("ts_each_key_is_index_paren");
}

#[rstest]
fn ts_each_key_is_item_as() {
    assert_compiler("ts_each_key_is_item_as");
}

#[rstest]
fn ts_legacy_reactive_assign_satisfies() {
    assert_compiler("ts_legacy_reactive_assign_satisfies");
}

#[rstest]
fn legacy_let_reassigned_unread_stays_plain() {
    assert_compiler("legacy_let_reassigned_unread_stays_plain");
}

#[rstest]
fn legacy_export_const_emits_bind_prop() {
    assert_compiler("legacy_export_const_emits_bind_prop");
}

#[rstest]
fn component_name_ignores_ts_namespace_type() {
    assert_compiler("component_name_ignores_ts_namespace_type");
}

#[rstest]
fn diagnose_component_on_directive_legacy_reactive_handler() {
    assert_compiler("diagnose_component_on_directive_legacy_reactive_handler");
}

#[rstest]
fn diagnose_legacy_reactive_store_value_passed_as_bare_prop() {
    assert_compiler("diagnose_legacy_reactive_store_value_passed_as_bare_prop");
}

#[rstest]
fn diagnose_legacy_let_writable_store_only_assign() {
    assert_compiler("diagnose_legacy_let_writable_store_only_assign");
}

#[rstest]
fn diagnose_legacy_bind_store_member_keeps_writable_plain() {
    assert_compiler("diagnose_legacy_bind_store_member_keeps_writable_plain");
}

#[rstest]
fn diagnose_legacy_component_bind_base_store_unsub() {
    assert_compiler("diagnose_legacy_component_bind_base_store_unsub");
}

#[rstest]
fn diagnose_component_prop_call_fold_global_inlined() {
    assert_compiler("diagnose_component_prop_call_fold_global_inlined");
}

#[rstest]
fn diagnose_ts_type_param_node_skews_anchor_idents() {
    assert_compiler("diagnose_ts_type_param_node_skews_anchor_idents");
}

#[rstest]
fn diagnose_component_prop_const_init_identifier_inline() {
    assert_compiler("diagnose_component_prop_const_init_identifier_inline");
}

#[rstest]
fn diagnose_slot_attribute_const_arrow_shorthand() {
    assert_compiler("diagnose_slot_attribute_const_arrow_shorthand");
}

#[rstest]
fn diagnose_component_prop_object_literal_arrow_value_inline() {
    assert_compiler("diagnose_component_prop_object_literal_arrow_value_inline");
}

#[rstest]
fn diagnose_component_prop_object_literal_shorthand_inline() {
    assert_compiler("diagnose_component_prop_object_literal_shorthand_inline");
}

#[rstest]
fn diagnose_component_name_collides_with_slot_let_binding() {
    assert_compiler("diagnose_component_name_collides_with_slot_let_binding");
}

#[rstest]
fn diagnose_component_name_collides_with_each_alias_binding() {
    assert_compiler("diagnose_component_name_collides_with_each_alias_binding");
}

#[rstest]
fn diagnose_component_name_collides_with_snippet_param_binding() {
    assert_compiler("diagnose_component_name_collides_with_snippet_param_binding");
}

#[rstest]
fn diagnose_legacy_each_collection_store_member_optional_wraps() {
    assert_compiler("diagnose_legacy_each_collection_store_member_optional_wraps");
}

#[rstest]
fn diagnose_soft_legacy_each_store_member_emits_boot_scaffolding() {
    assert_compiler("diagnose_soft_legacy_each_store_member_emits_boot_scaffolding");
}

#[rstest]
fn diagnose_soft_legacy_each_store_item_reactive_read() {
    assert_compiler("diagnose_soft_legacy_each_store_item_reactive_read");
}

#[rstest]
fn diagnose_legacy_each_collection_store_member_local_const_no_wrap() {
    assert_compiler("diagnose_legacy_each_collection_store_member_local_const_no_wrap");
}

#[rstest]
fn legacy_each_collection_logical_nullish() {
    assert_compiler("legacy_each_collection_logical_nullish");
}

#[rstest]
fn legacy_each_collection_conditional() {
    assert_compiler("legacy_each_collection_conditional");
}

#[rstest]
fn legacy_each_collection_logical_or() {
    assert_compiler("legacy_each_collection_logical_or");
}

#[rstest]
fn legacy_each_collection_sequence() {
    assert_compiler("legacy_each_collection_sequence");
}

#[rstest]
fn legacy_each_collection_parenthesized() {
    assert_compiler("legacy_each_collection_parenthesized");
}

#[rstest]
fn each_collection_source_prop_direct() {
    assert_compiler("each_collection_source_prop_direct");
}

#[rstest]
fn each_collection_source_prop_parenthesized() {
    assert_compiler("each_collection_source_prop_parenthesized");
}

#[rstest]
fn each_collection_source_local_member_chain() {
    assert_compiler("each_collection_source_local_member_chain");
}

#[rstest]
fn render_tag_optional_chain() {
    assert_compiler("render_tag_optional_chain");
}

#[rstest]
fn render_tag_conditional_callee() {
    assert_compiler("render_tag_conditional_callee");
}

#[rstest]
fn render_tag_logical_callee() {
    assert_compiler("render_tag_logical_callee");
}

#[rstest]
fn render_tag_args_with_reactive_state() {
    assert_compiler("render_tag_args_with_reactive_state");
}

#[rstest]
fn legacy_await_member_chain_promise() {
    assert_compiler("legacy_await_member_chain_promise");
}

#[rstest]
fn legacy_await_call_with_state_arg() {
    assert_compiler("legacy_await_call_with_state_arg");
}

#[rstest]
fn diagnose_css_custom_prop_component_concat_literal_const_fold() {
    assert_compiler("diagnose_css_custom_prop_component_concat_literal_const_fold");
}

#[rstest]
fn diagnose_slot_element_let_directive_alias_in_named_slot_fill() {
    assert_compiler("diagnose_slot_element_let_directive_alias_in_named_slot_fill");
}

#[rstest]
fn diagnose_on_directive_optional_chain_handler_wrap() {
    assert_compiler("diagnose_on_directive_optional_chain_handler_wrap");
}

#[rstest]
fn diagnose_legacy_slot_let_const_tag_ordering() {
    assert_compiler("diagnose_legacy_slot_let_const_tag_ordering");
}

#[rstest]
fn legacy_bindable_export_with_api_export() {
    assert_compiler("legacy_bindable_export_with_api_export");
}

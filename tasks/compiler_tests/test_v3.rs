use std::{
    fs::{File, read_to_string},
    io::Write,
};

use compiler_tests::cases::{load_v3_case, load_v3_module_case, v3_case_dir};
use compiler_tests::sourcemap_invariants::assert_sourcemap_invariants;
use compiler_tests::{compiler_case, compiler_module_case};
use pretty_assertions::assert_eq;
use svelte_compiler::{GenerateMode, compile, compile_module};
use test_support::strip_reference_only_css_markers;

fn normalize_css(s: &str) -> String {
    let stripped = strip_reference_only_css_markers(s);
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn assert_compiler_prod(case: &str) {
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

fn assert_compiler_dev(case: &str) {
    let dir = v3_case_dir(case);
    let (input, opts) = load_v3_case(case);
    let mut dev_opts = opts.clone();
    dev_opts.dev = true;
    let dev_js = compile(&input, &dev_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] dev compile produced no JS"))
        .code;
    let expected_dev_js = read_to_string(dir.join("case-svelte.dev.js")).expect("test invariant");
    File::create(dir.join("case-rust.dev.js"))
        .expect("test invariant")
        .write_all(dev_js.as_bytes())
        .expect("test invariant");
    assert_eq!(dev_js, expected_dev_js, "[{case}] dev JS mismatch");
}

fn assert_compiler_ssr(case: &str) {
    let dir = v3_case_dir(case);
    let (input, opts) = load_v3_case(case);
    let mut server_opts = opts.clone();
    server_opts.generate = GenerateMode::Server;
    let server_js = compile(&input, &server_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] server compile produced no JS"))
        .code;
    let expected_server_js =
        read_to_string(dir.join("case-svelte.server.js")).expect("test invariant");
    File::create(dir.join("case-rust.server.js"))
        .expect("test invariant")
        .write_all(server_js.as_bytes())
        .expect("test invariant");
    assert_eq!(server_js, expected_server_js, "[{case}] server JS mismatch");
}

fn assert_compiler_ssr_dev(case: &str) {
    let dir = v3_case_dir(case);
    let (input, opts) = load_v3_case(case);
    let mut server_opts = opts.clone();
    server_opts.generate = GenerateMode::Server;
    server_opts.dev = true;
    let server_js = compile(&input, &server_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] server dev compile produced no JS"))
        .code;
    let expected_server_js =
        read_to_string(dir.join("case-svelte.server.dev.js")).expect("test invariant");
    File::create(dir.join("case-rust.server.dev.js"))
        .expect("test invariant")
        .write_all(server_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        server_js, expected_server_js,
        "[{case}] server dev JS mismatch"
    );
}

compiler_case!(legacy_const_each_bind_member_chain);

compiler_case!(diagnose_js_object_method_shorthand);

compiler_case!(diagnose_props_default_identifier_prop_reference);

compiler_case!(diagnose_props_default_identifier_non_reactive);

compiler_case!(css_scope_class_in_snippet);

compiler_case!(css_scope_svelte_element_class);

compiler_case!(css_scope_class_object);

compiler_case!(css_scope_class_array_no_directive);

compiler_case!(css_scope_class_array_with_state);

compiler_case!(css_scope_spread_attribute);

compiler_case!(css_unused_external);

compiler_case!(css_unused_injected, [prod, dev_todo, ssr, ssr_dev_todo]);

compiler_case!(css_nested_style);

compiler_case!(css_nested_pseudo_element_no_scope_class);

compiler_case!(css_nested_amp_compound_no_scope_class);

compiler_case!(diagnose_css_nested_sibling_amp_scope_class);

compiler_case!(diagnose_css_animation_vendor_prefix);

compiler_case!(diagnose_css_vendor_keyframes_rename);

compiler_case!(css_scoped_id_selector);

compiler_case!(css_scoped_attr_presence);

compiler_case!(css_scoped_attr_value_selector);

compiler_case!(css_scoped_attr_matcher_operators);

compiler_case!(css_scoped_attr_name_casefolding);

compiler_case!(css_pseudo_compound_unused_but_scoped);

compiler_case!(css_scope_child_combinator_bare_pseudo);

compiler_case!(css_scope_child_combinator_global_pseudo_unscoped);

compiler_case!(css_snippet_descendant_scope_boundary);

compiler_case!(css_snippet_sibling_boundary);

compiler_case!(diagnose_css_recursive_snippet_sibling_overflow);

compiler_case!(css_component_snippet_descendant_boundary);

compiler_case!(css_pseudo_has);

compiler_case!(css_pseudo_not_scoped);

compiler_case!(css_nesting_selector_scoped);

compiler_case!(css_root_has_scoped);

compiler_case!(css_escaped_selector_scoped);

compiler_case!(css_dynamic_attr_selector_match);

compiler_case!(css_comments_preserved);

compiler_case!(script_module_exports_ordering_with_snippets);

compiler_case!(script_jsdoc_preserve);

compiler_case!(state_raw_dev_ce_with_props_rest);

compiler_case!(warn_attr_avoid_is);

compiler_case!(warn_attr_illegal_colon);

compiler_case!(warn_attr_invalid_prop_name);

compiler_case!(warn_slot_deprecated);

compiler_case!(slot_named_fallback);

compiler_case!(legacy_slot_dev_mixed);

compiler_case!(slot_props_default);

compiler_case!(slot_props_spread);

compiler_case!(slot_props_dynamic_state);

compiler_case!(slot_props_dynamic_call);

compiler_case!(diagnose_legacy_slot_props_store_member);

compiler_case!(diagnose_legacy_slot_prop_conditional_no_derived_wrap);

compiler_case!(diagnose_legacy_slot_prop_non_simple_stateful_shapes);

compiler_case!(warn_script_context_deprecated);

compiler_case!(head_with_special_elements);

compiler_case!(head_with_snippets);

compiler_case!(head_with_if_body);

compiler_case!(head_with_render_and_component);

compiler_case!(diagnose_head_script_in_if);

compiler_case!(diagnose_head_inline_script_template_literal);

compiler_case!(head_nested_if_with_body_if_id_order);

compiler_case!(head_title_then_meta_effect_order);

compiler_case!(push_binding_group_order);

compiler_case!(bind_group_order_with_stores);

compiler_case!(bind_group_order_with_legacy_reactive);

compiler_case!(component_bind_group_multiple_targets);

compiler_case!(bind_group_value_defined);

compiler_case!(bind_member_expression_no_runes);

compiler_case!(legacy_const_destructured_member_bind);

compiler_case!(diagnose_legacy_const_destructure_keeps_siblings);

compiler_case!(legacy_const_member_mutation_through_ts_non_null);

compiler_case!(
    css_injected_append_styles_with_stores_order,
    [prod, dev_todo, ssr, ssr_dev_todo]
);

compiler_case!(css_scoped_basic);

compiler_case!(css_injected, [prod, dev_todo, ssr, ssr_dev_todo]);

compiler_case!(css_global_basic);

compiler_case!(css_global_block);

compiler_case!(css_global_compound);

compiler_case!(css_global_in_pseudo);

compiler_case!(css_global_with_combinators);

compiler_case!(diagnose_css_global_leading_combinator);

compiler_case!(css_keyframes_scoped);

compiler_case!(css_keyframes_percentage_scopes_all);

compiler_case!(bind_this_with_children_and_class_directive);

compiler_case!(head_position_with_body);

compiler_case!(special_elements_all);

compiler_case!(empty);

compiler_case!(simple, "hello_state");

compiler_case!(single_text_node);

compiler_case!(single_element);

compiler_case!(single_interpolation);

compiler_case!(text_entity_decoding);

compiler_case!(text_entity_decoding_root);

compiler_case!(title_entity_decoding);

compiler_case!(single_if_block);

compiler_case!(single_if_else_block);

compiler_case!(if_call_condition);

compiler_case!(if_block_empty_consequent);

compiler_case!(if_block_empty_alternate);

compiler_case!(element_attributes);

compiler_case!(element_autofocus);

compiler_case!(textarea_child_value_dynamic);

compiler_case!(option_expr_child_value);

compiler_case!(option_expr_value);

compiler_case!(option_concat_value);

compiler_case!(option_expr_value_multi);

compiler_case!(option_expr_value_defined);

compiler_case!(option_expr_value_use_action_cache_var_order);

compiler_case!(bind_value_dev_named_fns);

compiler_case!(bind_component_prop_dev_ownership);

compiler_case!(bind_component_plain_prop_dev_ownership);

compiler_case!(bind_dynamic_component_dev_ownership);

compiler_case!(bind_component_dev_ownership_ignore);

compiler_case!(bind_component_explicit_source);

compiler_case!(customizable_select_option_el);

compiler_case!(customizable_select_select_div);

compiler_case!(selectedcontent_basic);

compiler_case!(state_runes);

compiler_case!(state_raw);

compiler_case!(state_eager_basic);

compiler_case!(state_eager_reactive);

compiler_case!(state_eager_template);

compiler_case!(state_snapshot_basic);

compiler_case!(state_snapshot_expression);

compiler_case!(state_snapshot_reactive);

compiler_case!(each_block);

compiler_case!(each_index_underscore_identifier, [prod, dev, ssr, ssr_dev]);

compiler_case!(
    store_mutate_server_shadowed_dollar_param,
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(slot_svg_namespace_whitespace, [prod, dev, ssr, ssr_dev]);

compiler_case!(
    slot_element_svg_namespace_whitespace,
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(each_inner_shadow);

compiler_case!(each_nested_array_destructure_no_inner_shadow);

compiler_case!(each_legacy_shadow_with_script_array_assign);

compiler_case!(bind_directives);

compiler_case!(nested_elements);

compiler_case!(nested_resets);

compiler_case!(single_concatenation);

compiler_case!(elements_childs);

compiler_case!(generic_root_sequence);

compiler_case!(spread_attribute);

compiler_case!(attribute_effect_shorthand_prop_unwrap);

compiler_case!(img_spread_replay_events);

compiler_case!(embed_spread_replay_events);

compiler_case!(object_spread_replay_events);

compiler_case!(source_spread_in_each_no_replay_events);

compiler_case!(spread_class_directive);

compiler_case!(spread_style_directive);

compiler_case!(utf8);

compiler_case!(smoke);

compiler_case!(class_directive);

compiler_case!(diagnose_class_directive_name_with_underscore);

compiler_case!(diagnose_class_directive_named_like_slot_attribute_no_placement_error);

compiler_case!(diagnose_class_directive_call_in_template_effect);

compiler_case!(diagnose_class_directive_slots_member_legacy);

compiler_case!(diagnose_attribute_effect_spread_call_memo);

compiler_case!(diagnose_attribute_effect_concat_call_part_memo);

compiler_case!(diagnose_attribute_effect_hydration_ignored_eighth_arg);

compiler_case!(diagnose_set_attribute_hydration_ignored_fourth_arg);

compiler_case!(diagnose_set_xlink_attribute_hydration_ignored_fourth_arg);

compiler_case!(class_concat);

compiler_case!(empty_class_attribute_static_elided);

compiler_case!(diagnose_class_directive_on_svg_is_html_flag);

compiler_case!(class_concat_literal_fold);

compiler_case!(attribute_concat_literal_fold);

compiler_case!(component_prop_concat_literal_fold);

compiler_case!(rune_update);

compiler_case!(assign_in_template);

compiler_case!(only_script);

compiler_case!(hoist_imports);

compiler_case!(bind_directives_extended);

compiler_case!(mutated_state_rune);

compiler_case!(static_interpolation);

compiler_case!(props_basic);

compiler_case!(props_rest);

compiler_case!(props_renamed);

compiler_case!(props_const_destructured_with_default);

compiler_case!(template_effect_merge_class_state_with_memo_text);

compiler_case!(props_renamed_bindable);

compiler_case!(props_bindable);

compiler_case!(props_lazy_default);

compiler_case!(props_mutated);

compiler_case!(props_member_mutation_computed);

compiler_case!(props_renamed_member_update_computed);

compiler_case!(props_mixed);

compiler_case!(exports);

compiler_case!(snippet_basic);

compiler_case!(component_basic);

compiler_case!(svelte_component_basic);

compiler_case!(svelte_component_children);

compiler_case!(svelte_component_if_child);

compiler_case!(svelte_component_each_child);

compiler_case!(svelte_component_slot_legacy_store_reactivity);

compiler_case!(svelte_component_slot_legacy_if_store_untrack);

compiler_case!(component_non_self_closing);

compiler_case!(component_in_element);

compiler_case!(component_mixed);

compiler_case!(component_props);

compiler_case!(component_children);

compiler_case!(diagnose_component_slot_node_naming);

compiler_case!(diagnose_sibling_after_deep_nested_elements);

compiler_case!(legacy_slot_fallback_if_sibling_node_naming);

compiler_case!(component_events);

compiler_case!(component_events_dev_apply);

compiler_case!(component_element_children);

compiler_case!(component_named_slot);

compiler_case!(legacy_slot_forward_named_into_child_component);

compiler_case!(component_default_slot_let);

compiler_case!(diagnose_component_callee_from_slot_let);

compiler_case!(diagnose_component_callee_member_legacy_reactive_root);

compiler_case!(diagnose_component_callee_from_slot_let_shadowed_by_script_binding);

compiler_case!(component_let_directive_name_starts_with_on);

compiler_case!(diagnose_component_events_slot_let_handler);

compiler_case!(diagnose_component_attr_on_prefix_false_positive);

compiler_case!(component_default_slot_let_alias);

compiler_case!(component_named_slot_let_element);

compiler_case!(component_named_slot_let_element_destructure);

compiler_case!(component_named_slot_let_element_multiple);

compiler_case!(component_child_slot_attribute);

compiler_case!(smoke_all);

compiler_case!(derived_basic);

compiler_case!(diagnose_derived_rune_with_ts_as_cast);

compiler_case!(derived_by);

compiler_case!(derived_dynamic);

compiler_case!(derived_write_assignment);

compiler_case!(unmutated_state_optimization);

compiler_case!(effect_runes);

compiler_case!(effect_root_basic);

compiler_case!(effect_root_cleanup);

compiler_case!(effect_tracking);

compiler_case!(effect_pending);

compiler_case!(effect_pending_script_init);

compiler_case!(effect_pending_script_derived);

compiler_case!(host_basic);

compiler_case!(host_props_rest);

compiler_case!(custom_element_props);

compiler_case!(custom_element_props_config);

compiler_case!(custom_element_boolean_default);

compiler_case!(custom_element_exports);

compiler_case!(custom_element_shadow_none);

compiler_case!(custom_element_object_full);

compiler_case!(custom_element_shadow_open);

compiler_case!(custom_element_extend);

compiler_case!(custom_element_no_tag);

compiler_case!(custom_element_prop_alias);

compiler_case!(custom_element_compile_option_default);

compiler_case!(custom_element_dev_exports_legacy_api);

compiler_case!(custom_element_slots);

compiler_case!(legacy_props_basic);

compiler_case!(legacy_reactivity_let_basic);

compiler_case!(legacy_reactivity_var_basic);

compiler_case!(legacy_reactivity_member_mutation);

compiler_case!(legacy_reactivity_array_self_assign);

compiler_case!(legacy_reactivity_destructure);

compiler_case!(diagnose_legacy_array_destructure_mixed_targets);

compiler_case!(legacy_reactive_assignment_basic);

compiler_case!(diagnose_legacy_pre_effect_reset_with_module_script);

compiler_case!(legacy_reactive_assignment_declared_dependency);

compiler_case!(legacy_reactive_assignment_block_destructure);

compiler_case!(legacy_reactive_assignment_coarse_deps);

compiler_case!(legacy_reactive_assignment_import_topology);

compiler_case!(legacy_rest_props_basic);

compiler_case!(legacy_slots_if);

compiler_case!(legacy_slots_script_basic);

compiler_case!(legacy_before_after_update_basic);

compiler_case!(legacy_before_after_update_alias);

compiler_case!(
    custom_element_css_default_injected,
    [prod, dev_todo, ssr, ssr_dev]
);

compiler_case!(custom_element_shadow_object);

compiler_case!(html_tag);

compiler_case!(html_tag_mathml);

compiler_case!(svg_foreignobject_fragment_html);

compiler_case!(mathml_root_html_fragment);

compiler_case!(mathml_annotation_xml_fragment_html);

compiler_case!(key_block);

compiler_case!(key_block_nested);

compiler_case!(style_directive);

compiler_case!(css_custom_prop_component);

compiler_case!(component_css_prop_boolean);

compiler_case!(css_custom_prop_component_svg);

compiler_case!(css_custom_prop_component_nested);

compiler_case!(css_custom_prop_component_string_value);

compiler_case!(css_custom_prop_component_concat_value);

compiler_case!(css_custom_prop_component_with_memoized_prop);

compiler_case!(css_custom_prop_component_slot_fill);

compiler_case!(style_directive_important);

compiler_case!(style_directive_string);

compiler_case!(style_directive_concat);

compiler_case!(on_directive);

compiler_case!(on_directive_modifiers);

compiler_case!(on_directive_with_use);

compiler_case!(on_directive_with_use_bind_this_order);

compiler_case!(on_directive_nonpassive);

compiler_case!(on_directive_dev_apply);

compiler_case!(use_action_basic);

compiler_case!(use_action_expression);

compiler_case!(use_action_reactive);

compiler_case!(use_action_dotted);

compiler_case!(use_action_dotted_hyphen);

compiler_case!(use_action_multiple);

compiler_case!(use_action_in_if);

compiler_case!(use_action_in_each);

compiler_case!(use_action_with_children);

compiler_case!(void_elements);

compiler_case!(non_void_self_closing);

compiler_case!(mixed_html_elements);

compiler_case!(store_basic);

compiler_case!(store_legacy_let_synthetic_reassign);

compiler_case!(store_legacy_each_invalidate);

compiler_case!(store_legacy_each_member_iterable);

compiler_case!(store_bind_value_thunk_arrow);

compiler_case!(store_legacy_var_basic);

compiler_case!(store_legacy_member_mutation);

compiler_case!(store_runes_id_assign_ops);

compiler_case!(store_runes_id_ops_template);

compiler_case!(store_runes_member_ops_script);

compiler_case!(store_runes_member_ops_template);

compiler_case!(store_runes_computed_member);

compiler_case!(store_runes_dev_smoke);

compiler_case!(store_runes_each_member_mutation);

compiler_case!(store_runes_prop_thunk);

compiler_case!(store_runes_synthetic_thunk_derived_base);

compiler_case!(store_runes_prop_assign_bind);

compiler_case!(diagnose_bindable_prop_store_only);

compiler_case!(store_runes_component_bind_prop_store);

compiler_case!(component_bind_ref_state_flag);

compiler_case!(component_bind_prop_order);

compiler_case!(component_snippet_node_ident_ordering);

compiler_case!(store_legacy_id_assign_ops);

compiler_case!(store_legacy_id_ops_template);

compiler_case!(store_legacy_member_ops_script);

compiler_case!(store_legacy_dev_smoke);

compiler_case!(store_legacy_bind_value);

compiler_case!(diagnose_legacy_bind_value_writable_store_shadow);

compiler_case!(legacy_dev_synthetic_store_thunk);

compiler_case!(legacy_dev_writable_no_mutable_source);

compiler_case!(legacy_dev_store_thunk_call_read);

compiler_case!(legacy_dev_store_set_assignment);

compiler_case!(legacy_dev_bind_store_unsub);

compiler_case!(legacy_dev_bind_this_promotes_state);

compiler_case!(legacy_dev_reactive_text_only_element);

compiler_case!(legacy_dev_deferred_template_effect);

compiler_case!(diagnose_legacy_head_title_store_deps_coarse_wrap);

compiler_case!(store_if_block_condition);

compiler_case!(store_key_block_expression);

compiler_case!(store_await_block_promise);

compiler_case!(store_render_tag_snippet);

compiler_case!(store_bind_this_element_ref);

compiler_case!(store_bare_identifier_method_call);

compiler_case!(store_write);

compiler_case!(store_validate_dev);

compiler_case!(store_reassign_unsub);

compiler_case!(store_each_invalidate);

compiler_case!(store_mark_binding);

compiler_case!(const_tag);

compiler_case!(const_tag_destructured);

compiler_case!(const_tag_destructured_multi);

compiler_case!(const_tag_destructured_if);

compiler_case!(const_tag_destructured_single_key);

compiler_case!(const_tag_destructured_default);

compiler_case!(diagnose_const_tag_legacy_destructure_into_component);

compiler_case!(const_tag_key_block);

compiler_case!(const_tag_await);

compiler_case!(const_tag_component);

compiler_case!(class_array);

compiler_case!(class_object);

compiler_case!(class_variable);

compiler_case!(class_expr_with_directives);

compiler_case!(class_concat_logical_and_string);

compiler_case!(bind_select_value);

compiler_case!(diagnose_select_value_attribute_emits_special_value_triple);

compiler_case!(bind_files);

compiler_case!(bind_property);

compiler_case!(bind_content_editable);

compiler_case!(bind_element_size);

compiler_case!(bind_omit_in_ssr_spread, [prod, dev, ssr, ssr_dev]);

compiler_case!(bind_natural_size_omit_ssr_spread, [ssr, ssr_dev]);

compiler_case!(bind_element_size_bindable_prop_source);

compiler_case!(bind_resize_observer);

compiler_case!(bind_resize_observer_border_box_size);

compiler_case!(bind_resize_observer_device_pixel_content_box_size);

compiler_case!(bind_textarea_value);

compiler_case!(bind_media_rw);

compiler_case!(bind_media_ro);

compiler_case!(bind_media_property);

compiler_case!(bind_img);

compiler_case!(bind_this);

compiler_case!(component_bind_this);

compiler_case!(component_bind_this_variants);

compiler_case!(svelte_self_if);

compiler_case!(svelte_self_if_sibling);

compiler_case!(svelte_self_if_else_sibling);

compiler_case!(svelte_self_each_sibling);

compiler_case!(svelte_self_element_sibling);

compiler_case!(svelte_self_css_props);

compiler_case!(svelte_self_each);

compiler_case!(svelte_self_snippet);

compiler_case!(svelte_self_slot);

compiler_case!(svelte_self_props);

compiler_case!(svelte_self_bind_this);

compiler_case!(bind_focused);

// ---------------------------------------------------------------------------
// Transition tests
// ---------------------------------------------------------------------------

compiler_case!(transition_basic);

compiler_case!(transition_params);

compiler_case!(transition_in);

compiler_case!(transition_out);

compiler_case!(transition_in_out_separate);

compiler_case!(transition_local);

compiler_case!(transition_global);

compiler_case!(transition_dotted_name);

compiler_case!(transition_in_if);

compiler_case!(transition_reactive_params);

compiler_case!(transition_elseif_local);

compiler_case!(transition_after_delegated);

compiler_case!(transition_after_delegated_descendant);

compiler_case!(transition_before_lifecycle_events);

// ---------------------------------------------------------------------------
// Animate directive tests
// ---------------------------------------------------------------------------

compiler_case!(animate_basic);

compiler_case!(animate_params);

compiler_case!(animate_dotted_name);

compiler_case!(animate_reactive_params);

compiler_case!(animate_svelte_element);

compiler_case!(animate_with_const_tag);

// ---------------------------------------------------------------------------
// Attach tag tests
// ---------------------------------------------------------------------------

compiler_case!(attach_basic);

compiler_case!(attach_inline_arrow);

compiler_case!(attach_conditional);

compiler_case!(attach_multiple);

compiler_case!(attach_with_directives);

compiler_case!(attach_in_if);

compiler_case!(attach_in_each);

// ---------------------------------------------------------------------------
// $state/$state.raw destructuring
// ---------------------------------------------------------------------------

compiler_case!(state_destructure);

compiler_case!(state_raw_destructure_object);

compiler_case!(state_raw_destructure_array);

// ---------------------------------------------------------------------------
// $state/$state.raw class fields
// ---------------------------------------------------------------------------

compiler_case!(state_class_field);

compiler_case!(state_raw_class_field);

compiler_case!(state_private_class_field);

compiler_case!(state_class_constructor);

compiler_case!(state_class_multiple);

compiler_case!(state_constructor_private_read);

compiler_case!(state_constructor_read_v);

compiler_case!(state_constructor_read_derived);

compiler_case!(state_class_raw_field);

compiler_case!(state_no_init);

compiler_case!(state_snapshot_in_template);

compiler_case!(state_snapshot_ignored);

compiler_case!(state_snapshot_not_ignored);

compiler_case!(state_snapshot_ignored_return);

compiler_case!(for_await_ignored);

compiler_case!(await_reactivity_ignored);

compiler_case!(state_class_constructor_proxy);

compiler_case!(derived_class_field);

compiler_case!(derived_by_class_fields);

compiler_case!(derived_by_class_constructor_only);

compiler_case!(derived_by_class_placeholder_preserves_plain_fields);

compiler_case!(state_class_field_constructor_assign);

compiler_case!(svg_inner_whitespace_trimming);

compiler_case!(svg_inner_template_from_svg);

compiler_case!(template_effect_call_deps);

compiler_case!(svg_text_preserves_whitespace);

compiler_case!(template_effect_multiple_call_deps);

compiler_case!(template_effect_attr_before_text_order);

compiler_case!(component_local_underscored_bind_this);

compiler_case!(component_dynamic_dotted_identifier_root);

compiler_case!(component_dynamic_props_access);

compiler_case!(component_dynamic_dotted_props_root);

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

fn assert_compiler_module_prod(case: &str) {
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

fn assert_compiler_module_dev(case: &str) {
    let dir = v3_case_dir(case);
    let (input, opts) = load_v3_module_case(case);

    let mut dev_opts = opts.clone();
    dev_opts.dev = true;
    let dev_js = compile_module(&input, &dev_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] dev compile_module produced no JS"))
        .code;
    let expected_dev = read_to_string(dir.join("case-svelte.dev.js")).expect("test invariant");
    File::create(dir.join("case-rust.dev.js"))
        .expect("test invariant")
        .write_all(dev_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_leading_block_comments(&dev_js),
        strip_leading_block_comments(&expected_dev),
        "[{case}] dev JS mismatch"
    );
}

fn assert_compiler_module_ssr(case: &str) {
    let dir = v3_case_dir(case);
    let (input, opts) = load_v3_module_case(case);

    let mut server_opts = opts.clone();
    server_opts.generate = GenerateMode::Server;
    let server_js = compile_module(&input, &server_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] server compile_module produced no JS"))
        .code;
    let expected_server =
        read_to_string(dir.join("case-svelte.server.js")).expect("test invariant");
    File::create(dir.join("case-rust.server.js"))
        .expect("test invariant")
        .write_all(server_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_leading_block_comments(&server_js),
        strip_leading_block_comments(&expected_server),
        "[{case}] server JS mismatch"
    );
}

fn assert_compiler_module_ssr_dev(case: &str) {
    let dir = v3_case_dir(case);
    let (input, opts) = load_v3_module_case(case);

    let mut server_dev_opts = opts.clone();
    server_dev_opts.generate = GenerateMode::Server;
    server_dev_opts.dev = true;
    let server_dev_js = compile_module(&input, &server_dev_opts)
        .js
        .unwrap_or_else(|| panic!("[{case}] server dev compile_module produced no JS"))
        .code;
    let expected_server_dev =
        read_to_string(dir.join("case-svelte.server.dev.js")).expect("test invariant");
    File::create(dir.join("case-rust.server.dev.js"))
        .expect("test invariant")
        .write_all(server_dev_js.as_bytes())
        .expect("test invariant");
    assert_eq!(
        strip_leading_block_comments(&server_dev_js),
        strip_leading_block_comments(&expected_server_dev),
        "[{case}] server dev JS mismatch"
    );
}

compiler_module_case!(
    state_class_field_proxy_init,
    "state_class_field_proxy_init",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_store_dollar_param_no_subscription,
    "module_store_dollar_param_no_subscription",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_derived_arrow_wrap_with_state_deps,
    "module_derived_arrow_wrap_with_state_deps",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_state_destructure,
    "module_state_destructure",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_compilation,
    "module_compilation",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    state_raw_class_constructor_object_ts,
    "state_raw_class_constructor_object_ts",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_derived_arrow_wrap_in_class_method,
    "module_derived_arrow_wrap_in_class_method",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_ts_strip,
    "module_ts_strip",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_derived_arrow_wrap_no_state_deps,
    "module_derived_arrow_wrap_no_state_deps",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    diagnose_state_private_constructor_init_from_decl_only,
    "diagnose_state_private_constructor_init_from_decl_only",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    state_private_class_field_array_proxy,
    "state_private_class_field_array_proxy",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    diagnose_module_leading_jsdoc_dropped,
    "diagnose_module_leading_jsdoc_dropped",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    ts_strip_non_null_chain,
    "ts_strip_non_null_chain",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_dev_state_tag,
    "module_dev_state_tag",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_dev_derived_tag,
    "module_dev_derived_tag",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    module_dev_console_log_wrap,
    "module_dev_console_log_wrap",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    diagnose_module_import_between_top_level_statements,
    "diagnose_module_import_between_top_level_statements",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(script_module_exports);

compiler_case!(script_module_export_specifiers);

compiler_case!(script_module_imports);

compiler_case!(script_module_empty);

compiler_case!(script_module_runes);

compiler_case!(script_module_instance_ref);

compiler_case!(script_module_only);

compiler_case!(script_module_with_instance);

compiler_case!(svelte_options_basic);

compiler_case!(svelte_options_runes_false_override);

compiler_case!(svelte_options_accessors_legacy);

compiler_case!(svelte_options_immutable_legacy);

compiler_case!(immutable_init_ctx_mutable);

compiler_case!(immutable_init_noctx_immutable);

compiler_case!(immutable_init_noctx_mutable);

compiler_case!(immutable_source_init_immutable);

compiler_case!(immutable_source_noinit_immutable);

compiler_case!(immutable_source_noinit_mutable);

compiler_case!(immutable_reactive_immutable);

compiler_case!(legacy_export_let_required);

compiler_case!(legacy_export_var_basic);

compiler_case!(legacy_export_specifier);

compiler_case!(legacy_export_specifier_alias);

compiler_case!(legacy_export_destructure);

compiler_case!(legacy_export_let_typed);

compiler_case!(legacy_export_let_member_mutation);

compiler_case!(legacy_export_let_bind_to_inner);

compiler_case!(diagnose_legacy_export_let_element_bind_this);

compiler_case!(diagnose_legacy_bind_group_radio_export_let_prop);

compiler_case!(diagnose_legacy_bind_group_radio_attr_update_order);

compiler_case!(diagnose_legacy_bind_group_value_use_action_cache_var_order);

compiler_case!(diagnose_legacy_bind_group_radio_value_shorthand_spread);

compiler_case!(diagnose_legacy_each_bind_group_input_value_merges_with_text);

compiler_case!(diagnose_legacy_each_bind_group_radio_value_index_no_cache);

compiler_case!(diagnose_legacy_input_bind_spread_omits_remove_defaults);

compiler_case!(diagnose_input_spread_only_should_remove_defaults);

compiler_case!(diagnose_input_boolean_value_emits_remove_defaults);

compiler_case!(diagnose_input_default_value_attribute_omits_remove_defaults);

compiler_case!(diagnose_spread_bind_this_action_order);

compiler_case!(diagnose_legacy_attr_expression_restprops_member_coarse_wrap);

compiler_case!(diagnose_legacy_attribute_effect_expression_restprops_coarse_wrap);

compiler_case!(diagnose_legacy_restprops_template_only_function_param);

compiler_case!(diagnose_legacy_child_bind_after_parent_spread_event_order);

compiler_case!(diagnose_legacy_bind_value_on_implicit_reactive_declaration);

compiler_case!(diagnose_script_let_multi_declarator_splits);

compiler_case!(diagnose_script_empty_named_import_normalized_to_bare);

compiler_case!(diagnose_script_ts_empty_named_import_normalized_to_bare);

compiler_case!(diagnose_legacy_bind_value_textarea_export_let_prop);

compiler_case!(diagnose_legacy_bind_files_export_let_prop);

compiler_case!(diagnose_textarea_bind_value_with_spread_init_order);

compiler_case!(diagnose_textarea_spread_only_emits_remove_child);

compiler_case!(diagnose_textarea_dynamic_value_attribute_emits_remove_child);

compiler_case!(diagnose_legacy_export_let_store_prop_subscription);

compiler_case!(diagnose_legacy_export_let_store_prop_writes);

compiler_case!(diagnose_legacy_export_let_leaks_into_exports);

compiler_case!(diagnose_runes_prop_export_specifier);

compiler_case!(runes_prop_export_specifier_alias);

compiler_case!(runes_prop_export_specifier_dev);

compiler_case!(runes_state_export_specifier);

compiler_case!(runes_raw_state_export_specifier);

compiler_case!(runes_local_let_export_specifier);

compiler_case!(runes_local_const_export_specifier);

compiler_case!(runes_derived_export_specifier);

compiler_case!(diagnose_legacy_export_const_init_order);

compiler_case!(diagnose_legacy_props_spread_no_init);

compiler_case!(diagnose_legacy_props_spread_only_no_push);

compiler_case!(diagnose_legacy_export_const_excluded_from_rest_props);

compiler_case!(diagnose_legacy_export_const_bind_prop_after_template_render);

compiler_case!(diagnose_css_attribute_selector_quote_style);

compiler_case!(diagnose_css_adjacent_sibling_across_if_block);

compiler_case!(diagnose_css_attribute_selector_unquoted_value);

compiler_case!(diagnose_css_declaration_value_comment);

compiler_case!(diagnose_css_type_class_selector_on_svelte_element);

compiler_case!(diagnose_legacy_component_bind_store_prop);

compiler_case!(diagnose_legacy_component_bind_store_reactive_destructured_base);

compiler_case!(legacy_export_let_compound_assign_prop);

compiler_case!(legacy_export_let_update_prop_in_template);

compiler_case!(legacy_export_let_assign_prop_in_template);

compiler_case!(legacy_state_member_update_in_template);

compiler_case!(legacy_state_member_compound_in_template);

compiler_case!(legacy_export_let_member_update_in_template);

compiler_case!(diagnose_legacy_export_let_member_assign_computed_member_key);

compiler_case!(legacy_export_let_default_typed_cast_arrow);

compiler_case!(diagnose_legacy_export_let_default_prop_reference);

compiler_case!(diagnose_legacy_export_let_default_prop_reference_in_conditional);

compiler_case!(diagnose_legacy_export_let_closure_capture_needs_push_init_pop);

compiler_case!(diagnose_legacy_each_key_prop_call_needs_push_init_pop);

compiler_case!(svelte_component_legacy_derived_hoist_block);

compiler_case!(legacy_export_let_key_block_member_coarse_wrap);

compiler_case!(legacy_pre_effect_store_subscription_dep);

compiler_case!(diagnose_legacy_reactive_ts_type_position_dep);

compiler_case!(diagnose_legacy_reactive_array_destructure_with_store);

compiler_case!(legacy_reactive_assignment_object_destructure_with_store);

compiler_case!(legacy_reactive_object_destructure_single_store_leaf);

compiler_case!(legacy_reactive_assignment_mixed_destructure);

compiler_case!(diagnose_legacy_reactive_destructure_identifier_rhs);

compiler_case!(runes_prop_member_update_in_template);

compiler_case!(runes_prop_member_compound_in_template);

compiler_case!(smoke_legacy_reactive_mutations_all);

compiler_case!(smoke_runes_reactive_mutations_all);

compiler_case!(smoke_legacy_contextual_mutations_all);

compiler_case!(smoke_legacy_rune_fallback_all);

compiler_case!(derived_non_runes_invalid_usage);

compiler_case!(smoke_runes_declarator_gaps_all);

compiler_case!(smoke_runes_state_eager_panic);

compiler_case!(smoke_ts_non_null_assertion_mutations);

compiler_case!(text_expression_binary_no_nullish_fallback);

compiler_case!(text_expression_conditional_memoized_needs_nullish_fallback);

compiler_case!(svelte_options_preserve_whitespace);

compiler_case!(preserve_whitespace_compile_option_true);

compiler_case!(preserve_whitespace_pre_first_newline);

compiler_case!(preserve_whitespace_inner_trailing_text);

compiler_case!(preserve_whitespace_script_element);

compiler_case!(diagnose_svg_fragment_root_ws_between_siblings);

compiler_case!(diagnose_svg_block_fragment_ws_between_siblings);

compiler_case!(diagnose_svg_component_slot_ws_between_siblings);

compiler_case!(diagnose_svg_text_block_ws_preserved);

compiler_case!(diagnose_svg_snippet_body_ws_between_siblings);

compiler_case!(diagnose_svg_legacy_slot_ws_between_siblings);

compiler_case!(diagnose_svg_root_html_tag_strategy);

compiler_case!(diagnose_svg_root_block_only_html_tag);

compiler_case!(preserve_comments_basic);

compiler_case!(preserve_comments_only_child);

compiler_case!(preserve_comments_between_elements);

compiler_case!(preserve_comments_in_block);

compiler_case!(preserve_comments_svelte_ignore);

compiler_case!(preserve_comments_only_in_block);

compiler_case!(preserve_comments_consecutive);

compiler_case!(diagnose_consecutive_comments_between_components);

compiler_case!(preserve_comments_empty);

compiler_case!(preserve_comments_in_each);

// ---------------------------------------------------------------------------
// svelte:head tests
// ---------------------------------------------------------------------------

compiler_case!(svelte_head_basic);

compiler_case!(svelte_head_reactive);

compiler_case!(svelte_head_with_content);

// <title> in <svelte:head> tests
compiler_case!(title_variants);

compiler_case!(async_title_basic);

// svelte:window tests
compiler_case!(svelte_window_event_legacy);

compiler_case!(svelte_window_event_legacy_with_if);

compiler_case!(diagnose_svelte_window_on_directive_legacy_prop_handler_wraps);

compiler_case!(diagnose_svelte_document_on_directive_legacy_prop_handler_wraps);

compiler_case!(diagnose_svelte_body_on_directive_legacy_prop_handler_wraps);

compiler_case!(svelte_window_action);

compiler_case!(svelte_document_action);

compiler_case!(svelte_window_event_attr);

compiler_case!(svelte_window_event_attr_props_handler);

compiler_case!(svelte_document_event_attr_props_handler);

compiler_case!(svelte_body_event_attr_props_handler);

compiler_case!(svelte_window_bind_scroll);

compiler_case!(svelte_window_bind_size);

compiler_case!(svelte_window_bind_size_legacy);

compiler_case!(svelte_window_bind_size_with_template);

compiler_case!(svelte_window_bind_online);

compiler_case!(svelte_window_combined);

compiler_case!(svelte_window_reactive);

compiler_case!(svelte_document_bindings);

compiler_case!(svelte_document_events);

compiler_case!(svelte_document_bubble);

compiler_case!(svelte_document_combined);

compiler_case!(svelte_element_basic);

compiler_case!(svelte_element_self_closing);

compiler_case!(svelte_fragment_named_slot);

compiler_case!(svelte_fragment_named_slot_inside_svelte_component);

compiler_case!(svelte_fragment_named_slot_component_expr_attr);

compiler_case!(component_named_slot_let_fragment);

compiler_case!(component_named_slot_let_fragment_destructure);

compiler_case!(svelte_fragment_default_slot_wrapper);

compiler_case!(svelte_fragment_default_slot_let);

compiler_case!(svelte_fragment_explicit_default_slot_attribute_lowers_to_children_prop);

compiler_case!(svelte_fragment_named_slot_with_const_tag);

compiler_case!(svelte_element_static_tag);

compiler_case!(svelte_element_attributes);

compiler_case!(svelte_element_spread);

compiler_case!(diagnose_svelte_element_spread_with_class_directive);

compiler_case!(diagnose_svelte_element_class_directive_with_dynamic_attr);

compiler_case!(diagnose_svelte_element_spread_style_directive);

compiler_case!(diagnose_svelte_element_spread_class_with_bind);

compiler_case!(diagnose_svelte_element_static_class_with_directive);

compiler_case!(diagnose_svelte_element_static_style_with_directive);

compiler_case!(diagnose_svelte_element_dynamic_class_attr_with_directive);

compiler_case!(diagnose_svelte_element_style_directive_with_dynamic_attr);

compiler_case!(diagnose_svelte_element_class_and_style_directives);

compiler_case!(audit_svelte_element_scoped_no_class);

compiler_case!(audit_svelte_element_scoped_class_directive);

compiler_case!(audit_svelte_element_scoped_static_class_with_directive);

compiler_case!(audit_svelte_element_scoped_dynamic_class_with_directive);

compiler_case!(audit_svelte_element_class_dynamic_alone);

compiler_case!(audit_svelte_element_class_concat_alone);

compiler_case!(audit_svelte_element_class_concat_with_directive);

compiler_case!(audit_svelte_element_class_array_alone);

compiler_case!(audit_svelte_element_class_array_with_directive);

compiler_case!(audit_svelte_element_style_dynamic_alone);

compiler_case!(audit_svelte_element_style_concat_alone);

compiler_case!(audit_svelte_element_style_dynamic_with_directive);

compiler_case!(audit_svelte_element_scoped_style_directive);

compiler_case!(audit_svelte_element_class_directive_with_bind);

compiler_case!(audit_svelte_element_class_directive_with_use);

compiler_case!(audit_svelte_element_dynamic_attr_with_both_directives);

compiler_case!(audit_svelte_element_xmlns_with_class_directive);

compiler_case!(audit_svelte_element_static_class_with_style_directive);

compiler_case!(audit_svelte_element_static_class_with_both_directives);

compiler_case!(audit_svelte_element_on_legacy_with_class_directive);

compiler_case!(audit_svelte_element_transition_only);

compiler_case!(audit_svelte_element_spread_with_use);

compiler_case!(diagnose_svelte_element_attribute_effect_call_value_memo);

compiler_case!(diagnose_svelte_element_use_action_with_on_event_legacy);

compiler_case!(audit_svelte_element_style_directive_with_bind);

compiler_case!(audit_svelte_element_class_directive_expression);

compiler_case!(audit_svelte_element_style_directive_important);

compiler_case!(audit_svelte_element_style_directive_custom_prop);

compiler_case!(audit_svelte_element_animate_in_each);

compiler_case!(audit_svelte_element_bind_this_store);

compiler_case!(audit_svelte_element_attach_tag);

compiler_case!(audit_svelte_element_multiple_class_directives);

compiler_case!(audit_svelte_element_multiple_style_directives);

compiler_case!(audit_svelte_element_bind_this_member);

compiler_case!(audit_svelte_element_multiple_spreads);

compiler_case!(audit_svelte_element_transition_with_params);

compiler_case!(audit_svelte_element_transition_local);

compiler_case!(audit_svelte_element_in_out_separate);

compiler_case!(audit_svelte_element_use_action_with_params);

compiler_case!(audit_svelte_element_modern_event_handler);

compiler_case!(
    audit_svelte_element_async_tag_with_class_directive,
    [prod, dev_todo, ssr, ssr_dev]
);

compiler_case!(
    audit_svelte_element_async_tag_with_spread,
    [prod, dev_todo, ssr, ssr_dev]
);

compiler_case!(audit_svelte_element_scoped_with_spread);

compiler_case!(audit_svelte_element_class_directive_with_call);

compiler_case!(audit_svelte_element_dev_spread);

compiler_case!(audit_svelte_element_dev_class_directive);

compiler_case!(audit_svelte_element_transition_in_with_params);

compiler_case!(audit_svelte_element_transition_global);

compiler_case!(audit_svelte_element_class_directive_with_each);

compiler_case!(audit_svelte_element_dev_static_class_with_directive);

compiler_case!(svelte_element_onclick);

compiler_case!(svelte_element_bind);

compiler_case!(svelte_element_null_tag);

compiler_case!(svelte_element_xmlns);

compiler_case!(svelte_element_dynamic_xmlns);

compiler_case!(svelte_element_children_expr);

compiler_case!(svelte_element_body_multi_child_fragment);

compiler_case!(svelte_body_event_attr);

compiler_case!(svelte_body_event_legacy);

compiler_case!(svelte_body_action);

compiler_case!(svelte_body_combined);

compiler_case!(boundary_basic);

compiler_case!(boundary_failed_snippet);

compiler_case!(boundary_onerror);

compiler_case!(boundary_pending_snippet);

compiler_case!(boundary_failed_onerror);

compiler_case!(boundary_failed_attribute);

compiler_case!(boundary_all_three);

compiler_case!(boundary_reactive_onerror);

compiler_case!(boundary_nested);

compiler_case!(boundary_const_tag);

compiler_case!(boundary_in_if);

compiler_case!(boundary_other_snippets);

compiler_case!(boundary_pending_attribute);

compiler_case!(boundary_pending_imported);

compiler_case!(boundary_failed_attribute_override);

compiler_case!(boundary_pending_attribute_override);

compiler_case!(await_basic);

compiler_case!(await_short_then);

compiler_case!(await_short_catch);

compiler_case!(await_then_catch);

compiler_case!(await_no_bindings);

compiler_case!(await_pending_only);

compiler_case!(await_destructured);

compiler_case!(await_in_if);

compiler_case!(await_in_each);

compiler_case!(await_reactive);

compiler_case!(await_nested_content);

// ---------------------------------------------------------------------------
// Event attribute tests (Svelte 5)
// ---------------------------------------------------------------------------

compiler_case!(event_attr_non_delegatable);

compiler_case!(event_attr_delegated_after_non_delegated_order);

compiler_case!(event_attr_capture);

compiler_case!(event_attr_capture_non_deleg);

compiler_case!(event_attr_gotpointercapture);

compiler_case!(event_attr_passive);

compiler_case!(event_attr_passive_window);

compiler_case!(event_attr_import_handler);

compiler_case!(event_attr_member_handler);

compiler_case!(event_attr_const_tag_destructure);

compiler_case!(event_attr_props_handler);

compiler_case!(event_attr_has_call);

compiler_case!(event_attr_dev_apply);

// ---------------------------------------------------------------------------
// Expression memoization tests
// ---------------------------------------------------------------------------

compiler_case!(component_prop_has_call);

compiler_case!(component_prop_has_call_multi);

compiler_case!(component_prop_has_call_mixed);

compiler_case!(component_prop_concat_call_memo);

compiler_case!(diagnose_component_prop_concat_legacy_ternary);

compiler_case!(component_prop_concat_call_with_literal);

compiler_case!(component_prop_concat_import_identifier);

compiler_case!(component_prop_concat_import_and_call);

compiler_case!(html_concat_call_with_literal);

compiler_case!(html_class_concat_call_with_literal);

compiler_case!(component_dynamic_dotted);

compiler_case!(component_prop_memo_state);

compiler_case!(component_prop_const_call_init_getter);

compiler_case!(component_prop_const_member_init_getter);

compiler_case!(component_prop_imported_direct_getter);

compiler_case!(component_prop_import_meta_getter);

compiler_case!(element_attr_import_meta_template_effect);

compiler_case!(render_tag_arg_has_call);

compiler_case!(render_tag_arg_has_call_multi);

compiler_case!(render_tag_arg_mixed);

compiler_case!(render_tag_dynamic_prop);

compiler_case!(render_tag_dynamic_state);

compiler_case!(render_tag_dynamic_snippet_param);

compiler_case!(render_tag_optional);

compiler_case!(render_tag_optional_dynamic);

// ---------------------------------------------------------------------------
// $inspect rune tests
// ---------------------------------------------------------------------------

compiler_case!(inspect_basic);

compiler_case!(inspect_with_callback);

compiler_case!(inspect_prod_strip);

// ---------------------------------------------------------------------------
// $inspect.trace() rune tests
// ---------------------------------------------------------------------------

compiler_case!(inspect_trace_basic);

compiler_case!(inspect_trace_contexts);

compiler_case!(inspect_trace_prod_strip);

compiler_case!(inspect_trace_reactive_contexts);

// ---------------------------------------------------------------------------
// $props.id() rune tests
// ---------------------------------------------------------------------------

compiler_case!(props_id_basic);

compiler_case!(props_id_with_props);

// ---------------------------------------------------------------------------
// {@debug} tests
// ---------------------------------------------------------------------------

compiler_case!(debug_basic);

compiler_case!(debug_in_blocks);

// ---------------------------------------------------------------------------
// TypeScript stripping tests
// ---------------------------------------------------------------------------

compiler_case!(ts_strip_expression_tag);

compiler_case!(ts_strip_satisfies);

compiler_case!(ts_strip_non_null);

compiler_case!(ts_strip_const_tag);

compiler_case!(ts_strip_attribute);

compiler_case!(ts_strip_script_types);

compiler_case!(ts_strip_handler_param_annotation);

compiler_case!(ts_strip_as_paren_optional_chain);

compiler_case!(ts_strip_catch_empty_comment_orphan);

compiler_case!(diagnose_ts_cast_const_init_marks_attr_dynamic);

compiler_case!(namespace_svg);

compiler_case!(namespace_mathml);

compiler_case!(svg_fragment_ambiguous_a);

compiler_case!(svg_fragment_ambiguous_title);

compiler_case!(svelte_element_in_if);

compiler_case!(svelte_element_class_directive);

compiler_case!(svelte_element_style_directive);

compiler_case!(svelte_element_dev_invalid_tag);

compiler_case!(svelte_element_dev_void_children);

compiler_case!(boundary_const_in_snippet);

compiler_case!(boundary_imported_handler);

compiler_case!(bind_this_sequence);

// ---------------------------------------------------------------------------
// Tier 2b — Template Tags
// ---------------------------------------------------------------------------

compiler_case!(await_array_destructured);

compiler_case!(html_tag_controlled);

compiler_case!(html_tag_svg);

compiler_case!(html_tag_nested_svg);

compiler_case!(html_tag_nested_mathml);

compiler_case!(html_tag_hydration_ignore);

compiler_case!(const_tag_dev);

compiler_case!(rune_compound_template);

compiler_case!(store_assign_template);

compiler_case!(store_compound_template);

compiler_case!(store_update_template);

compiler_case!(store_deep_mutation);

compiler_case!(store_deep_update);

// ---------------------------------------------------------------------------
// Tier 2c — Bind Directive Edge Cases
// ---------------------------------------------------------------------------

compiler_case!(bind_function_value);

compiler_case!(bind_function_checked);

compiler_case!(bind_use_deferral);

compiler_case!(bind_contenteditable_flag);

compiler_case!(bind_group_each);

compiler_case!(bind_group_keyed_each);

compiler_case!(bind_group_nested_each);

compiler_case!(bind_group_value_attr);

compiler_case!(bind_group_value_attr_before_bind);

compiler_case!(bind_group_each_legacy_item_member_untrack);

compiler_case!(bind_group_each_var);

compiler_case!(bind_group_each_var_keyed);

compiler_case!(each_fallback);

compiler_case!(each_keyed_index);

compiler_case!(diagnose_each_key_legacy_prop_no_rewrite);

compiler_case!(each_keyed_index_plain_in_body);

compiler_case!(each_key_is_index_literal_diagnose);

compiler_case!(each_key_uses_index);

compiler_case!(each_key_is_item);

compiler_case!(each_destructured_obj);

compiler_case!(each_destructured_default);

compiler_case!(each_destructured_array);

compiler_case!(diagnose_each_rest_only_pattern_binding);

compiler_case!(each_destructured_obj_with_rest);

compiler_case!(each_destructured_array_with_rest);

compiler_case!(each_destructured_array_rest_only);

compiler_case!(diagnose_legacy_each_component_css_prop_hoist_derived);

compiler_case!(style_attr_object);

compiler_case!(style_attr_dynamic);

compiler_case!(script_jsdoc_comment);

compiler_case!(svelte_head_title_meta);

compiler_case!(snippet_ident_conflict_with_script);

compiler_case!(debug_non_dev);

compiler_case!(debug_non_runes_untrack);

compiler_case!(non_runes_simple_snapshot);

compiler_case!(animate_with_spread);

compiler_case!(svelte_element_static_class_attr);

compiler_case!(root_with_special_elements);

compiler_case!(needs_context_method_chain);

compiler_case!(event_handler_derived_with_class_directives);

compiler_case!(event_handler_derived_with_class_object);

compiler_case!(diagnose_class_directive_legacy_event_handler_derived_order);

compiler_case!(diagnose_class_directive_named_slot_let_classes_order_legacy);

compiler_case!(derived_inside_function);

compiler_case!(derived_nested_getter);

compiler_case!(derived_shorthand_property);

compiler_case!(state_inside_function);

compiler_case!(derived_by_inside_function);

compiler_case!(component_snippet_prop);

compiler_case!(component_snippet_with_children);

compiler_case!(component_multiple_snippets);

compiler_case!(component_snippet_only);

// ---------------------------------------------------------------------------
// Diagnose: TypeScript import + spread + bind:prop tests
// ---------------------------------------------------------------------------

compiler_case!(ts_type_import_comment);

compiler_case!(rest_props_member_access);

compiler_case!(component_spread_props);

compiler_case!(component_bind_prop_forward);

compiler_case!(component_bind_member_path);

compiler_case!(component_bind_member_path_bindable_root, [prod, dev]);

compiler_case!(component_bind_member_path_dev);

compiler_case!(component_bind_function);

compiler_case!(component_bind_function_anchor_order);

compiler_case!(diagnose_component_bind_function_props_position);

compiler_case!(diagnose_component_bind_derived_target_no_proxy_flag);

compiler_case!(diagnose_fragment_id_after_component_with_snippet);

compiler_case!(diagnose_fragment_id_in_snippet_used_as_expression);

compiler_case!(diagnose_fragment_id_in_sibling_named_slot_after_component);

compiler_case!(component_prop_const_tag_member);

compiler_case!(diagnose_component_prop_const_tag_member_init);

compiler_case!(diagnose_component_prop_const_tag_destructured_shorthand);

compiler_case!(diagnose_each_const_tag_shorthand_prop_to_component);

compiler_case!(diagnose_const_tag_legacy_dependency_destructure);

compiler_case!(diagnose_const_tag_legacy_dep_read_spread_of_derived);

compiler_case!(diagnose_legacy_slot_let_array_destructure_dep_read);

compiler_case!(diagnose_svelte_fragment_let_inside_named_slot_component);

compiler_case!(
    diagnose_svelte_component_css_custom_prop_wrapper,
    [prod, dev_todo]
);

compiler_case!(diagnose_component_prop_computed_member_getter);

compiler_case!(diagnose_component_prop_hyphenated_key_derived);

// ---------------------------------------------------------------------------
// Diagnose: svelte import patterns
// ---------------------------------------------------------------------------

compiler_case!(needs_context_nested_fn);

compiler_case!(member_expr_dynamic_local);

compiler_case!(import_type_mixed);

compiler_case!(derived_in_nested_function);

compiler_case!(derived_local_signal_get);

compiler_case!(svelte_element_duplicate_naming);

compiler_case!(each_block_no_item);

compiler_case!(each_block_no_item_multi);

compiler_case!(each_block_no_item_with_index);

compiler_case!(each_collection_call_reads_state);

compiler_case!(legacy_each_collection_member_const_wraps_untrack);

compiler_case!(legacy_each_collection_member_reactive_let_wraps_with_read);

compiler_case!(diagnose_legacy_each_collection_member_nullish_fallback_wraps_with_read);

compiler_case!(diagnose_legacy_each_collection_member_outer_each_item_wraps);

compiler_case!(diagnose_legacy_each_collection_member_export_let_prop_wraps);

compiler_case!(diagnose_legacy_each_collection_member_imported_wraps_with_read);

compiler_case!(diagnose_legacy_each_call_imported_wraps);

compiler_case!(diagnose_legacy_each_call_imported_args_wrap);

compiler_case!(diagnose_legacy_each_call_local_fn_wraps);

compiler_case!(diagnose_legacy_each_collection_new_expression_wraps);

compiler_case!(diagnose_legacy_each_collection_new_expression_prop_arg_wraps);

compiler_case!(async_if_basic);

compiler_case!(async_if_else_if_condition);

compiler_case!(if_elseif_new_blockers);

compiler_case!(async_each_basic);

compiler_case!(async_html_basic);

compiler_case!(async_key_basic);

compiler_case!(async_await_has_await);

compiler_case!(async_flag_import);

compiler_case!(async_blockers_basic);

compiler_case!(async_bind_basic);

compiler_case!(action_blockers);

compiler_case!(attach_blockers);

compiler_case!(transition_blockers);

compiler_case!(animate_blockers);

compiler_case!(async_svelte_element, [prod, dev_todo, ssr, ssr_dev]);

compiler_case!(async_const_tag);

compiler_case!(async_derived_basic);

compiler_case!(async_derived_destructured);

compiler_case!(async_derived_dev);

compiler_case!(async_derived_dev_ignored);

compiler_case!(async_derived_dev_ignored_destructured);

compiler_case!(async_derived_nested_function);

compiler_case!(async_derived_nested_function_destructured);

compiler_case!(async_for_await_dev);

compiler_case!(orthogonality_heavy_call);

compiler_case!(orthogonality_async_await);

compiler_case!(orthogonality_heavy_async_await_call);

compiler_case!(inline_await_basic);

compiler_case!(inline_await_global_callee);

compiler_case!(inline_await_text_concat);

compiler_case!(inline_await_attr);

compiler_case!(async_pickled_await_template);

compiler_case!(async_render_tag);

compiler_case!(async_render_tag_complex_args);

compiler_case!(async_boundary_const);

compiler_case!(async_const_derived_chain);

compiler_case!(attach_on_component);

compiler_case!(attach_on_component_dynamic);

compiler_case!(attach_on_document);

compiler_case!(each_keyed_destructure);

compiler_case!(await_then_text_before_element);

compiler_case!(await_thunk_optimization);

compiler_case!(await_each_nested);

compiler_case!(await_pending_then);

compiler_case!(await_pending_catch);

compiler_case!(await_short_catch_no_binding);

compiler_case!(await_nested_await);

compiler_case!(fragment_counter_with_nested_if);

compiler_case!(bind_group_radio_basic);

compiler_case!(bind_multiple_on_element);

compiler_case!(if_else_chain_with_const);

compiler_case!(event_mixed_delegation);

compiler_case!(props_identifier_basic);

compiler_case!(props_identifier_await_expression);

compiler_case!(call_expr_local_method_dynamic);

compiler_case!(call_expr_nested_fn_dynamic);

compiler_case!(effect_cleanup_return);

compiler_case!(tag_derived_basic);

compiler_case!(tag_derived_by);

compiler_case!(derived_destructured_object);

compiler_case!(derived_destructured_object_prop_source);

compiler_case!(derived_destructured_props_whole_source);

compiler_case!(derived_destructured_array);

compiler_case!(derived_destructured_by);

compiler_case!(tag_state_unmutated);

compiler_case!(tag_state_unmutated_no_proxy);

compiler_case!(tag_bindable_proxy);

compiler_case!(tag_class_field_public);

compiler_case!(tag_class_field_private);

compiler_case!(tag_class_constructor);

compiler_case!(tag_snippet_dev);

compiler_case!(tag_render_dev);

compiler_case!(snippet_destructure_dev);

compiler_case!(snippet_object_destructure);

compiler_case!(diagnose_legacy_snippet_param_member_to_component_prop);

compiler_case!(diagnose_legacy_component_prop_import_meta_getter_in_if);

compiler_case!(diagnose_snippet_inside_each_callback);

compiler_case!(snippet_array_destructure);

compiler_case!(snippet_mixed_params);

compiler_case!(snippet_nested_destructure);

compiler_case!(snippet_computed_key_destructure);

compiler_case!(snippet_body_single_component);

compiler_case!(tag_state_destructured_array);

compiler_case!(tag_state_destructured_object);

compiler_case!(state_var_safe_get);

compiler_case!(state_assign_dev);

compiler_case!(css_scoped_class_selector);

compiler_case!(bind_select_static_option_value);

compiler_case!(input_dynamic_special_attrs);

compiler_case!(svg_dynamic_special_attrs);

compiler_case!(each_index_text_no_coalesce);

compiler_case!(snippet_destructure_default_state_ref);

compiler_case!(snippet_destructure_default_mutated_state_ref);

compiler_case!(diagnose_props_bindable_icon_component);

compiler_case!(diagnose_props_identifier_in_snippet_body);

compiler_case!(diagnose_filename_name_collides_with_ts_type_import);

compiler_case!(props_bindable_checkbox_disabled_shorthand_ts);

compiler_case!(diagnose_component_onclick_state);

compiler_case!(diagnose_svg_city_icon, [prod, dev, ssr_todo, ssr_dev_todo]);

compiler_case!(clock_svg_derived_onmount);

compiler_case!(diagnose_component_default_and_named_slot_expr);

compiler_case!(diagnose_component_named_slot_child_with_expression_prop);

compiler_case!(diagnose_component_named_slot_child_inflates_template_root_ids);

compiler_case!(diagnose_component_named_slot_empty_element_child_kept);

compiler_case!(diagnose_component_named_slot_svelte_component_child_kept);

compiler_case!(diagnose_svelte_component_named_slot_expression_binds);

compiler_case!(diagnose_legacy_slot_forward_inflates_template_root_ids);

compiler_case!(diagnose_legacy_dev_benchmark);

compiler_case!(component_dev_default_children_wrap_snippet);

compiler_case!(component_dev_named_slot_no_wrap_snippet);

compiler_case!(diagnose_dev_benchmark);

compiler_case!(dev_binary_equals_wrap);

compiler_case!(dev_filename_relative_to_root_dir);

compiler_case!(dev_filename_root_dir_no_match);

compiler_case!(add_locations_svelte_head_skips_hoisted_title);

compiler_case!(add_locations_named_slot_wrapper);

compiler_case!(auto_softlegacy_member_read);

compiler_case!(auto_hardlegacy_member_read_explicit);

compiler_case!(auto_hardlegacy_store_autosub_shadows_rune);

compiler_case!(auto_hardlegacy_import_call_coarse_wrap);

compiler_case!(legacy_dev_inspect_fallback);

compiler_case!(legacy_dev_const_deep_read_wrap);

compiler_case!(legacy_dev_each_const_deep_read);

compiler_case!(legacy_dev_attribute_effect_grouping);

compiler_case!(legacy_dev_component_event_prop_derived);

compiler_case!(legacy_dev_component_prop_getter);

compiler_case!(auto_softlegacy_simple_template);

compiler_case!(auto_softlegacy_const_only);

compiler_case!(auto_softlegacy_const_member_mutation);

compiler_case!(legacy_state_bind_member_mutate_wrap);

compiler_case!(auto_softlegacy_component_prop_derived);

compiler_case!(auto_softlegacy_render_arg_memo);

compiler_case!(auto_softlegacy_template_call_expr);

compiler_case!(needs_context_template_member_in_binary);

compiler_case!(needs_context_template_member_in_conditional);

compiler_case!(needs_context_template_member_root_import);

compiler_case!(needs_context_template_member_ts_props);

compiler_case!(diagnose_component_bind_group_emits_array);

compiler_case!(diagnose_html_template_preserves_nbsp);

compiler_case!(diagnose_attribute_entity_decoding);

compiler_case!(diagnose_attribute_entity_strict_mode);

compiler_case!(diagnose_component_onclick_const_arrow);

compiler_case!(diagnose_component_spread_call_memo);

compiler_case!(diagnose_button_single_dynamic_text);

compiler_case!(diagnose_text_concat_import_uses_template_effect);

compiler_case!(diagnose_text_concat_sequence_expr_nullish_fallback);

compiler_case!(diagnose_static_text_before_dynamic_in_element);

compiler_case!(diagnose_snippet_inside_if_consequent);

compiler_case!(diagnose_snippet_hoistable_with_script_import);

compiler_case!(diagnose_snippet_hoistable_param_type_annotation);

compiler_case!(diagnose_snippet_name_with_underscore);

compiler_case!(diagnose_hoisted_snippet_module_order_with_sibling_template);

compiler_case!(diagnose_snippet_store_autosub_not_hoistable);

compiler_case!(diagnose_component_on_directive_shorthand_forward);

compiler_case!(diagnose_legacy_pre_effect_order_after_functions);

compiler_case!(diagnose_legacy_template_effect_prop_call_coarse_wrap);

compiler_case!(diagnose_legacy_local_var_not_promoted_to_state);

compiler_case!(diagnose_legacy_reactive_assignment_promotes_state_via_handler);

compiler_case!(diagnose_legacy_reactive_arrow_value_promotes_state);

compiler_case!(diagnose_legacy_each_const_destructure_coarse_wrap);

compiler_case!(diagnose_legacy_each_if_condition_coarse_wrap);

compiler_case!(diagnose_legacy_each_html_member_coarse_wrap);

compiler_case!(diagnose_legacy_class_attribute_prop_member_coarse_wrap);

compiler_case!(diagnose_legacy_each_bind_this_indexed_reactive);

compiler_case!(diagnose_legacy_each_bind_this_indexed_by_index_variable);

compiler_case!(diagnose_legacy_each_store_bind_value_item_member);

compiler_case!(diagnose_legacy_each_store_bind_checked_item_member);

compiler_case!(diagnose_legacy_each_store_bind_group_item_member);

compiler_case!(diagnose_legacy_each_store_bind_value_element_item_member);

compiler_case!(diagnose_legacy_each_store_bind_checked_element_item_member);

compiler_case!(diagnose_video_muted_static_attribute_lowers_to_property);

compiler_case!(diagnose_video_muted_concatenation_attribute_lowers_to_property);

compiler_case!(diagnose_legacy_if_store_short_circuit_coarse_wrap);

compiler_case!(diagnose_legacy_each_index_component_prop_plain);

compiler_case!(diagnose_legacy_each_css_props_member_coarse_wrap);

compiler_case!(diagnose_legacy_each_css_props_concat_member_coarse_wrap);

compiler_case!(diagnose_each_css_wrapper_root_order);

compiler_case!(diagnose_component_bind_store_derived_base);

compiler_case!(diagnose_nested_delegated_transition_order);

compiler_case!(diagnose_text_entity_leading_ws_before_if_block);

compiler_case!(diagnose_td_sibling_if_blocks_whitespace);

compiler_case!(diagnose_legacy_component_prop_const_call_safe_equal_untrack);

compiler_case!(diagnose_legacy_component_prop_call_with_prop_arg_coarse_wrap);

compiler_case!(diagnose_legacy_component_prop_ternary_arrow_no_coarse_wrap);

compiler_case!(diagnose_style_directive_literal_skips_template_effect_grouping);

compiler_case!(diagnose_style_directive_quoted_literal_interp_static);

compiler_case!(diagnose_style_directive_template_literal_const_ident_fold);

compiler_case!(diagnose_style_directive_const_ident_fold);

compiler_case!(diagnose_style_directive_const_number_ident_fold);

compiler_case!(diagnose_style_directive_const_binary_init_ident_fold);

compiler_case!(diagnose_style_directive_multiple_const_idents_fold);

compiler_case!(diagnose_style_directive_const_ident_fold_runes);

compiler_case!(diagnose_style_directive_const_with_state_keeps_reactive);

compiler_case!(diagnose_style_directive_legacy_let_no_fold);

compiler_case!(diagnose_style_directive_complex_expression_hoists_to_memo_deps);

compiler_case!(diagnose_style_attr_call_groups_in_template_effect_memo);

compiler_case!(diagnose_legacy_template_effect_merge_memo_attr_with_simple_attr);

compiler_case!(attr_call_value_hoists_to_template_effect_memo);

compiler_case!(diagnose_attr_call_no_references_init);

compiler_case!(diagnose_style_attr_call_no_references_with_directive_init);

compiler_case!(diagnose_style_attr_dynamic_with_style_directive_merges_set_style);

compiler_case!(diagnose_style_attr_with_directive_source_order);

compiler_case!(diagnose_style_directive_with_static_style_attribute);

compiler_case!(noscript_root_in_if_block);

compiler_case!(diagnose_legacy_pre_effect_dep_order_switch_externals);

compiler_case!(diagnose_legacy_pre_effect_dep_order_lhs_write_before_prop_read);

compiler_case!(diagnose_legacy_iife_read_nested_write_promotes_state);

compiler_case!(diagnose_legacy_pre_effect_store_write_read_topological_order);

compiler_case!(diagnose_css_slot_fallback_descendant_scope);

compiler_case!(ts_init_impure_type_assertion);

compiler_case!(ts_class_field_rune_paren);

compiler_case!(ts_class_field_rune_as);

compiler_case!(ts_constructor_this_rune_assignment_ts);

compiler_case!(ts_bindable_default_as);

compiler_case!(ts_bindable_default_identifier_paren);

compiler_case!(ts_derived_async_await_paren);

compiler_case!(ts_state_proxyable_paren_arrow);

compiler_case!(ts_simple_expression_paren_default);

compiler_case!(ts_each_key_is_index_paren);

compiler_case!(ts_each_key_is_item_as);

compiler_case!(ts_legacy_reactive_assign_satisfies);

compiler_case!(legacy_let_reassigned_unread_stays_plain);

compiler_case!(legacy_export_const_emits_bind_prop);

compiler_case!(component_name_ignores_ts_namespace_type);

compiler_case!(diagnose_component_on_directive_legacy_reactive_handler);

compiler_case!(diagnose_legacy_reactive_store_value_passed_as_bare_prop);

compiler_case!(diagnose_legacy_let_writable_store_only_assign);

compiler_case!(diagnose_legacy_bind_store_member_keeps_writable_plain);

compiler_case!(diagnose_legacy_component_bind_base_store_unsub);

compiler_case!(diagnose_component_prop_call_fold_global_inlined);

compiler_case!(diagnose_ts_type_param_node_skews_anchor_idents);

compiler_case!(diagnose_component_prop_const_init_identifier_inline);

compiler_case!(diagnose_slot_attribute_const_arrow_shorthand);

compiler_case!(diagnose_component_prop_object_literal_arrow_value_inline);

compiler_case!(diagnose_component_prop_object_literal_shorthand_inline);

compiler_case!(diagnose_component_name_collides_with_slot_let_binding);

compiler_case!(diagnose_component_name_collides_with_each_alias_binding);

compiler_case!(diagnose_component_name_collides_with_snippet_param_binding);

compiler_case!(diagnose_legacy_each_collection_store_member_optional_wraps);

compiler_case!(diagnose_soft_legacy_each_store_member_emits_boot_scaffolding);

compiler_case!(diagnose_soft_legacy_each_store_item_reactive_read);

compiler_case!(diagnose_legacy_each_collection_store_member_local_const_no_wrap);

compiler_case!(legacy_each_collection_logical_nullish);

compiler_case!(legacy_each_collection_conditional);

compiler_case!(legacy_each_collection_logical_or);

compiler_case!(legacy_each_collection_sequence);

compiler_case!(legacy_each_collection_parenthesized);

compiler_case!(each_collection_source_prop_direct);

compiler_case!(each_collection_source_prop_parenthesized);

compiler_case!(each_collection_source_local_member_chain);

compiler_case!(render_tag_optional_chain);

compiler_case!(render_tag_conditional_callee);

compiler_case!(render_tag_logical_callee);

compiler_case!(render_tag_args_with_reactive_state);

compiler_case!(legacy_await_member_chain_promise);

compiler_case!(legacy_await_call_with_state_arg);

compiler_case!(diagnose_css_custom_prop_component_concat_literal_const_fold);

compiler_case!(diagnose_slot_element_let_directive_alias_in_named_slot_fill);

compiler_case!(diagnose_on_directive_optional_chain_handler_wrap);

compiler_case!(diagnose_legacy_slot_let_const_tag_ordering);

compiler_case!(legacy_bindable_export_with_api_export);

compiler_case!(css_has_leading_plus_global);

compiler_case!(css_has_leading_plus_global_flat);

compiler_case!(css_has_leading_child_global);

compiler_case!(css_has_leading_plus_class);

compiler_case!(css_has_descendant_global);

compiler_case!(css_has_leading_tilde_global);

compiler_case!(css_nth_child_leading_plus_sign);

compiler_case!(css_nth_child_of_selector_list);

compiler_case!(css_nth_child_of_combinator);

compiler_case!(css_nth_child_uppercase_name);

use compiler_tests::harness::{
    assert_compiler_dev, assert_compiler_module_dev, assert_compiler_module_prod,
    assert_compiler_module_ssr, assert_compiler_module_ssr_dev, assert_compiler_prod,
    assert_compiler_ssr, assert_compiler_ssr_dev,
};
use compiler_tests::{compiler_case, compiler_module_case};

#[path = "clusters/value_evaluation.rs"]
mod value_evaluation;

#[path = "clusters/comments.rs"]
mod comments;

#[path = "clusters/empty_statement.rs"]
mod empty_statement;

#[path = "clusters/paren_strip.rs"]
mod paren_strip;

#[path = "clusters/prop_default_store.rs"]
mod prop_default_store;

#[path = "clusters/attribute_template_literal.rs"]
mod attribute_template_literal;

#[path = "clusters/bind_store_each.rs"]
mod bind_store_each;

#[path = "clusters/destructure_fallback.rs"]
mod destructure_fallback;

#[path = "clusters/store_module.rs"]
mod store_module;

#[path = "clusters/module_reactive.rs"]
mod module_reactive;

#[path = "clusters/dynamic_component_ssr.rs"]
mod dynamic_component_ssr;

#[path = "clusters/ident_gen.rs"]
mod ident_gen;

#[path = "clusters/bind_this_function_pair.rs"]
mod bind_this_function_pair;

#[path = "clusters/custom_element_var.rs"]
mod custom_element_var;

#[path = "clusters/assign_async.rs"]
mod assign_async;

#[path = "clusters/destructure_async.rs"]
mod destructure_async;

#[path = "clusters/snapshot_ignore_ssr.rs"]
mod snapshot_ignore_ssr;

#[path = "clusters/events.rs"]
mod events;

#[path = "clusters/state_snapshot.rs"]
mod state_snapshot;

#[path = "clusters/inspect_trace.rs"]
mod inspect_trace;

#[path = "clusters/void_element.rs"]
mod void_element;

#[path = "clusters/element_name_casing.rs"]
mod element_name_casing;

#[path = "clusters/dev_element_locations.rs"]
mod dev_element_locations;

#[path = "clusters/set_class_scope_hash.rs"]
mod set_class_scope_hash;

#[path = "clusters/whitespace_trim.rs"]
mod whitespace_trim;

#[path = "clusters/css_global_combinator.rs"]
mod css_global_combinator;

#[path = "clusters/hmr.rs"]
mod hmr;

#[path = "clusters/disclose_version.rs"]
mod disclose_version;

#[path = "clusters/fragment_static_text.rs"]
mod fragment_static_text;

#[path = "clusters/spread_class.rs"]
mod spread_class;
#[path = "clusters/spread_default_value.rs"]
mod spread_default_value;

#[path = "clusters/head_title.rs"]
mod head_title;

#[path = "clusters/legacy_slots.rs"]
mod legacy_slots;

#[path = "clusters/special_element_order.rs"]
mod special_element_order;

#[path = "clusters/console_log.rs"]
mod console_log;

#[path = "clusters/component_props.rs"]
mod component_props;

#[path = "clusters/member_assign.rs"]
mod member_assign;

#[path = "clusters/component_dynamic_name.rs"]
mod component_dynamic_name;

#[path = "clusters/attach_component.rs"]
mod attach_component;

#[path = "clusters/snippet_hoist.rs"]
mod snippet_hoist;

#[path = "clusters/snippet_placement.rs"]
mod snippet_placement;

#[path = "clusters/css_scope_svelte_fragment.rs"]
mod css_scope_svelte_fragment;

#[path = "clusters/css_sibling_slot.rs"]
mod css_sibling_slot;

#[path = "clusters/css_global_nesting.rs"]
mod css_global_nesting;

#[path = "clusters/css_is_has_prune.rs"]
mod css_is_has_prune;

#[path = "clusters/css_scope_placement_global.rs"]
mod css_scope_placement_global;

#[path = "clusters/css_root.rs"]
mod css_root;

#[path = "clusters/css_scope_position_global.rs"]
mod css_scope_position_global;

#[path = "clusters/css_global_local_compound.rs"]
mod css_global_local_compound;

#[path = "clusters/css_global_pseudo_args.rs"]
mod css_global_pseudo_args;

#[path = "clusters/css_global_block_animation.rs"]
mod css_global_block_animation;

#[path = "clusters/css_has_global_parent.rs"]
mod css_has_global_parent;

#[path = "clusters/css_host.rs"]
mod css_host;

#[path = "clusters/css_escaped_identifier.rs"]
mod css_escaped_identifier;

#[path = "clusters/css_brace_whitespace.rs"]
mod css_brace_whitespace;

#[path = "clusters/css_declaration_no_colon.rs"]
mod css_declaration_no_colon;

#[path = "clusters/css_injected.rs"]
mod css_injected;

#[path = "clusters/text_reactivity.rs"]
mod text_reactivity;

#[path = "clusters/text_node_marker.rs"]
mod text_node_marker;

#[path = "clusters/style_directive.rs"]
mod style_directive;

#[path = "clusters/customizable_select.rs"]
mod customizable_select;

#[path = "clusters/element_namespace.rs"]
mod element_namespace;

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

#[path = "clusters/directive_member_name.rs"]
mod directive_member_name;

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

#[path = "clusters/typescript.rs"]
mod typescript;

#[path = "clusters/attribute_memo.rs"]
mod attribute_memo;

#[path = "clusters/if_else.rs"]
mod if_else;

#[path = "clusters/attribute_single_expr.rs"]
mod attribute_single_expr;

#[path = "clusters/attribute_ws_around_equals.rs"]
mod attribute_ws_around_equals;

#[path = "clusters/attribute_unquoted_value.rs"]
mod attribute_unquoted_value;

#[path = "clusters/attribute_img_loading.rs"]
mod attribute_img_loading;

#[path = "clusters/custom_element.rs"]
mod custom_element;

#[path = "clusters/attribute_autofocus.rs"]
mod attribute_autofocus;

#[path = "clusters/attribute_svelte_element.rs"]
mod attribute_svelte_element;

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

#[path = "clusters/bind_prop_accessor.rs"]
mod bind_prop_accessor;

#[path = "clusters/template_element.rs"]
mod template_element;

#[path = "clusters/textarea_content.rs"]
mod textarea_content;

#[path = "clusters/attribute_entities.rs"]
mod attribute_entities;

#[path = "clusters/element_reset.rs"]
mod element_reset;

#[path = "clusters/closing_tag.rs"]
mod closing_tag;

#[path = "clusters/template_runes.rs"]
mod template_runes;

#[path = "clusters/boundary.rs"]
mod boundary;

#[path = "clusters/render_tag_arg.rs"]
mod render_tag_arg;

#[path = "clusters/css_prune.rs"]
mod css_prune;

#[path = "clusters/declaration_tag.rs"]
mod declaration_tag;

#[path = "clusters/element_block_scope.rs"]
mod element_block_scope;

#[path = "clusters/script_element.rs"]
mod script_element;

#[path = "clusters/module_props.rs"]
mod module_props;

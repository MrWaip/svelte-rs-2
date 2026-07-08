use super::*;

compiler_case!(
    legacy_dev_arrow_handler_return,
    "events/legacy_dev_arrow_handler_return",
    ignore = "divergence: legacy dev arrow handler expression body loses implicit return — on:click={() => n++} emits $.update(n) instead of return $.update(n); reproduces unquoted too, unrelated to quoted directives"
);
compiler_case!(
    snippet_param,
    "events/snippet_param",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    store_handler_onclick,
    "events/store_handler_onclick",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    store_handler_legacy_on,
    "events/store_handler_legacy_on",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    state_handler,
    "events/state_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    derived_handler,
    "events/derived_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_state_handler,
    "events/legacy_reactive_state_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(await_value, "events/await_value", [prod, dev, ssr, ssr_dev]);
compiler_case!(await_error, "events/await_error", [prod, dev, ssr, ssr_dev]);
compiler_case!(each_item, "events/each_item", [prod, dev, ssr, ssr_dev]);
compiler_case!(each_index, "events/each_index", [prod, dev, ssr, ssr_dev]);

compiler_case!(
    shorthand_event_delegatable,
    "events/shorthand_event_delegatable",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    shorthand_event_non_delegatable,
    "events/shorthand_event_non_delegatable",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    shorthand_event_bare_on,
    "events/shorthand_event_bare_on",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    forward_bare_on_regular,
    "events/forward_bare_on_regular",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    forward_bare_on_svelte_body,
    "events/forward_bare_on_svelte_body",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    forward_bare_on_svelte_element,
    "events/forward_bare_on_svelte_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    forward_bare_on_nested_if,
    "events/forward_bare_on_nested_if",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    forward_bare_on_handler_present,
    "events/forward_bare_on_handler_present",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    forward_bare_on_window,
    "events/forward_bare_on_window",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    shorthand_non_event,
    "events/shorthand_non_event",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    explicit_event_delegatable,
    "events/explicit_event_delegatable",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    shorthand_event_on_component,
    "events/shorthand_event_on_component",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    spread_order_one_handler,
    "events/spread_order_one_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_two_handlers,
    "events/spread_order_two_handlers",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_plain_handler,
    "events/spread_order_plain_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_no_handler,
    "events/spread_order_no_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_no_spread,
    "events/spread_order_no_spread",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_runes_onclick,
    "events/spread_order_runes_onclick"
);
compiler_case!(
    spread_order_runes_onclick_no_spread,
    "events/spread_order_runes_onclick_no_spread",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_use_directive,
    "events/spread_order_use_directive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_bind,
    "events/spread_order_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_transition,
    "events/spread_order_transition",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    spread_order_memo_with_use,
    "events/spread_order_memo_with_use",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    input_defaults_memo_on,
    "events/input_defaults_memo_on",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    textarea_remove_child_memo_on,
    "events/textarea_remove_child_memo_on"
);
compiler_case!(
    input_defaults_plain_on,
    "events/input_defaults_plain_on",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    input_defaults_no_on,
    "events/input_defaults_no_on",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    input_defaults_static_attr,
    "events/input_defaults_static_attr",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    input_no_defaults_memo_on,
    "events/input_no_defaults_memo_on",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    nospread_on_then_transition,
    "events/nospread_on_then_transition",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    nospread_transition_then_on,
    "events/nospread_transition_then_on",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    nospread_memo_on_with_use,
    "events/nospread_memo_on_with_use",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    window_quoted_handler,
    "events/window_quoted_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    document_quoted_handler,
    "events/document_quoted_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    body_quoted_handler,
    "events/body_quoted_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_quoted_handler,
    "events/element_quoted_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    window_unquoted_handler_guard,
    "events/window_unquoted_handler_guard",
    [prod, dev, ssr, ssr_dev]
);

use super::*;

compiler_case!(snippet_param, "events/snippet_param");
compiler_case!(await_value, "events/await_value");
compiler_case!(await_error, "events/await_error");
compiler_case!(each_item, "events/each_item");
compiler_case!(each_index, "events/each_index");

compiler_case!(
    shorthand_event_delegatable,
    "events/shorthand_event_delegatable"
);
compiler_case!(
    shorthand_event_non_delegatable,
    "events/shorthand_event_non_delegatable"
);
compiler_case!(shorthand_event_bare_on, "events/shorthand_event_bare_on");
compiler_case!(shorthand_non_event, "events/shorthand_non_event");
compiler_case!(
    explicit_event_delegatable,
    "events/explicit_event_delegatable"
);
compiler_case!(
    shorthand_event_on_component,
    "events/shorthand_event_on_component"
);

compiler_case!(spread_order_one_handler, "events/spread_order_one_handler");
compiler_case!(
    spread_order_two_handlers,
    "events/spread_order_two_handlers"
);
compiler_case!(
    spread_order_plain_handler,
    "events/spread_order_plain_handler"
);
compiler_case!(spread_order_no_handler, "events/spread_order_no_handler");
compiler_case!(spread_order_no_spread, "events/spread_order_no_spread");
compiler_case!(
    spread_order_runes_onclick,
    "events/spread_order_runes_onclick"
);
compiler_case!(
    spread_order_runes_onclick_no_spread,
    "events/spread_order_runes_onclick_no_spread"
);
compiler_case!(
    spread_order_use_directive,
    "events/spread_order_use_directive"
);
compiler_case!(spread_order_bind, "events/spread_order_bind");
compiler_case!(spread_order_transition, "events/spread_order_transition");
compiler_case!(
    spread_order_memo_with_use,
    "events/spread_order_memo_with_use"
);

compiler_case!(input_defaults_memo_on, "events/input_defaults_memo_on");
compiler_case!(
    textarea_remove_child_memo_on,
    "events/textarea_remove_child_memo_on"
);
compiler_case!(input_defaults_plain_on, "events/input_defaults_plain_on");
compiler_case!(input_defaults_no_on, "events/input_defaults_no_on");
compiler_case!(
    input_defaults_static_attr,
    "events/input_defaults_static_attr"
);
compiler_case!(
    input_no_defaults_memo_on,
    "events/input_no_defaults_memo_on"
);
compiler_case!(
    nospread_on_then_transition,
    "events/nospread_on_then_transition"
);
compiler_case!(
    nospread_transition_then_on,
    "events/nospread_transition_then_on"
);
compiler_case!(
    nospread_memo_on_with_use,
    "events/nospread_memo_on_with_use"
);

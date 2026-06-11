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

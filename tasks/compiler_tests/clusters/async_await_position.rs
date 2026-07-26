use super::*;

compiler_case!(
    member_static_object,
    "async_await_position/member_static_object"
);
compiler_case!(
    member_computed_property,
    "async_await_position/member_computed_property"
);
compiler_case!(unary_operand, "async_await_position/unary_operand");
compiler_case!(optional_member, "async_await_position/optional_member");
compiler_case!(spread_element, "async_await_position/spread_element");
compiler_case!(
    property_computed_key,
    "async_await_position/property_computed_key"
);

compiler_case!(
    member_computed_object_guard,
    "async_await_position/member_computed_object_guard"
);
compiler_case!(
    array_last_element_guard,
    "async_await_position/array_last_element_guard"
);
compiler_case!(
    tail_positions_guard,
    "async_await_position/tail_positions_guard"
);
compiler_case!(const_tag_guard, "async_await_position/const_tag_guard");

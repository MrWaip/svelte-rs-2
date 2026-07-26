use super::*;

compiler_case!(
    assignment_write,
    "async_instance_blockers/assignment_write",
    ignore = "instance-body writes after an await do not blocker the reading expression"
);
compiler_case!(
    update_write,
    "async_instance_blockers/update_write",
    ignore = "instance-body writes after an await do not blocker the reading expression"
);
compiler_case!(
    call_touch,
    "async_instance_blockers/call_touch",
    ignore = "bindings reached through a call after an await are not blockered"
);
compiler_case!(
    function_declaration,
    "async_instance_blockers/function_declaration",
    ignore = "function declarations get no deferred blocker"
);
compiler_case!(
    arrow_declarator,
    "async_instance_blockers/arrow_declarator",
    ignore = "arrow declarators get no deferred blocker"
);
compiler_case!(
    sync_group_slot,
    "async_instance_blockers/sync_group_slot",
    ignore = "consecutive non-await statements are split into separate promise slots"
);
compiler_case!(
    props_id_declarator,
    "async_instance_blockers/props_id_declarator",
    ignore = "$props.id declarator is not skipped when grouping the instance body"
);

compiler_case!(
    before_await_guard,
    "async_instance_blockers/before_await_guard"
);
compiler_case!(
    effect_not_traced,
    "async_instance_blockers/effect_not_traced",
    ignore =
        "single expression statement in a promise group is wrapped as a block, not `void expr`"
);
compiler_case!(
    statement_thunk_shape,
    "async_instance_blockers/statement_thunk_shape",
    ignore =
        "single expression statement in a promise group is wrapped as a block, not `void expr`"
);
compiler_case!(
    closure_reference_guard,
    "async_instance_blockers/closure_reference_guard"
);

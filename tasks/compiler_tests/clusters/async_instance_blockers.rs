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
compiler_case!(
    await_statement_collapse,
    "async_instance_blockers/await_statement_collapse",
    ignore =
        "a bare await statement keeps its async arrow instead of collapsing to `() => argument`"
);
compiler_case!(
    await_call_unthunk,
    "async_instance_blockers/await_call_unthunk",
    ignore = "`await load()` is not collapsed to the callee `load`"
);
compiler_case!(
    nested_await_no_collapse,
    "async_instance_blockers/nested_await_no_collapse",
    ignore = "an await with a nested await keeps a block body instead of an expression body"
);
compiler_case!(
    class_declaration_assignment,
    "async_instance_blockers/class_declaration_assignment",
    ignore = "a class declaration in a promise group is not rewritten to an assignment of a class expression"
);
compiler_case!(
    destructured_await_assignment,
    "async_instance_blockers/destructured_await_assignment",
    ignore = "a destructured awaited declarator keeps its declaration instead of assigning the hoisted bindings"
);
compiler_case!(
    removed_statement_slot,
    "async_instance_blockers/removed_statement_slot",
    ignore = "a statement erased by the transform panics instead of holding its slot with `() => void 0`"
);
compiler_case!(
    multi_declarator_slots,
    "async_instance_blockers/multi_declarator_slots",
    ignore =
        "a multi-declarator declaration in an async instance body panics on statement metadata"
);

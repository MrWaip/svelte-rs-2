use super::*;

compiler_case!(
    derived_destructured_after_await,
    "async_instance_blockers/derived_destructured_after_await",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    derived_destructured_carrier_guard,
    "async_instance_blockers/derived_destructured_carrier_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    boundary_nullish_pending_literal,
    "async_instance_blockers/boundary_nullish_pending_literal",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_write,
    "async_instance_blockers/assignment_write",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    update_write,
    "async_instance_blockers/update_write",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    call_touch,
    "async_instance_blockers/call_touch",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    function_declaration,
    "async_instance_blockers/function_declaration",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    arrow_declarator,
    "async_instance_blockers/arrow_declarator",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    sync_group_slot,
    "async_instance_blockers/sync_group_slot",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    props_id_declarator,
    "async_instance_blockers/props_id_declarator",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    before_await_guard,
    "async_instance_blockers/before_await_guard"
);
compiler_case!(
    effect_not_traced,
    "async_instance_blockers/effect_not_traced",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    statement_thunk_shape,
    "async_instance_blockers/statement_thunk_shape",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    closure_reference_guard,
    "async_instance_blockers/closure_reference_guard"
);
compiler_case!(
    await_statement_collapse,
    "async_instance_blockers/await_statement_collapse",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    await_call_unthunk,
    "async_instance_blockers/await_call_unthunk",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    nested_await_no_collapse,
    "async_instance_blockers/nested_await_no_collapse",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    class_declaration_assignment,
    "async_instance_blockers/class_declaration_assignment",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    destructured_await_assignment,
    "async_instance_blockers/destructured_await_assignment",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    removed_statement_slot,
    "async_instance_blockers/removed_statement_slot",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    multi_declarator_slots,
    "async_instance_blockers/multi_declarator_slots",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    parenthesized_await_collapse,
    "async_instance_blockers/parenthesized_await_collapse",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    export_specifier_slot,
    "async_instance_blockers/export_specifier_slot",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    block_statement_entry,
    "async_instance_blockers/block_statement_entry",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    erased_statement_slot_shape,
    "async_instance_blockers/erased_statement_slot_shape",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    props_erased_before_await,
    "async_instance_blockers/props_erased_before_await",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    function_declaration_after_erased_props,
    "async_instance_blockers/function_declaration_after_erased_props",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    derived_nested_await_thunk,
    "async_instance_blockers/derived_nested_await_thunk",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    destructured_props_assignment,
    "async_instance_blockers/destructured_props_assignment",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    effect_slot_void,
    "async_instance_blockers/effect_slot_void",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    bindable_prop_after_await,
    "async_instance_blockers/bindable_prop_after_await",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    rest_prop_after_await,
    "async_instance_blockers/rest_prop_after_await",
    [prod, dev, ssr, ssr_dev]
);

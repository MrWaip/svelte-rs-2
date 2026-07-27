use super::*;

compiler_case!(non_await_body, "async_block_thunk/non_await_body");
compiler_case!(
    await_body_nested_await,
    "async_block_thunk/await_body_nested_await"
);
compiler_case!(html_tag, "async_block_thunk/html_tag");
compiler_case!(
    render_tag_await_arg,
    "async_block_thunk/render_tag_await_arg"
);
compiler_case!(svelte_element_tag, "async_block_thunk/svelte_element_tag");

compiler_case!(
    await_body_simple_guard,
    "async_block_thunk/await_body_simple_guard"
);
compiler_case!(
    await_body_await_in_function_guard,
    "async_block_thunk/await_body_await_in_function_guard"
);

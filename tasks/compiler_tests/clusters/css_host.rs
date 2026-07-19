use super::*;

compiler_case!(host_descendant, "css_host/host_descendant");
compiler_case!(host_child, "css_host/host_child");
compiler_case!(host_child_universal, "css_host/host_child_universal");
compiler_case!(host_deep, "css_host/host_deep");
compiler_case!(host_args, "css_host/host_args");

compiler_case!(
    host_not_direct_child_guard,
    "css_host/host_not_direct_child_guard"
);
compiler_case!(host_bare_guard, "css_host/host_bare_guard");
compiler_case!(root_descendant_guard, "css_host/root_descendant_guard");
compiler_case!(plain_descendant_guard, "css_host/plain_descendant_guard");

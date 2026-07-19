use super::*;

compiler_case!(class_hex, "css_escaped_identifier/class_hex");
compiler_case!(class_hex_multi, "css_escaped_identifier/class_hex_multi");
compiler_case!(id_hex_space, "css_escaped_identifier/id_hex_space");
compiler_case!(class_hex_astral, "css_escaped_identifier/class_hex_astral");
compiler_case!(
    descendant_escaped,
    "css_escaped_identifier/descendant_escaped"
);

compiler_case!(
    class_plain_guard,
    "css_escaped_identifier/class_plain_guard"
);
compiler_case!(
    class_hex_descendant_unused_guard,
    "css_escaped_identifier/class_hex_descendant_unused_guard"
);

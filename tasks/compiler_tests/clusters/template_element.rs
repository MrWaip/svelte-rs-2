use super::*;

compiler_case!(static_content, "template_element/static");
compiler_case!(element_child, "template_element/element_child");
compiler_case!(html_tag, "template_element/html_tag");

compiler_case!(
    shadowrootmode_slot_native,
    "template_element/shadowrootmode_slot_native"
);
compiler_case!(
    shadowrootmode_closed_nested_slot_native,
    "template_element/shadowrootmode_closed_nested_slot_native"
);
compiler_case!(
    content_descend_child_element,
    "template_element/content_descend_child_element"
);
compiler_case!(
    content_descend_compiled_slot,
    "template_element/content_descend_compiled_slot"
);
compiler_case!(
    content_descend_if_block,
    "template_element/content_descend_if_block"
);
compiler_case!(
    shadowrootmode_expression_content,
    "template_element/shadowrootmode_expression_content"
);
compiler_case!(html_tag_sole_child, "template_element/html_tag_sole_child");

compiler_case!(
    slot_outside_shadowrootmode_guard,
    "template_element/slot_outside_shadowrootmode_guard"
);

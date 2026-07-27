use super::*;

compiler_module_case!(dynamic_import_comment, "comments/dynamic_import_comment");
compiler_module_case!(
    dynamic_import_no_comment_guard,
    "comments/dynamic_import_no_comment_guard"
);

compiler_case!(
    element_tag_line_comment,
    "comments/element_tag_line_comment"
);
compiler_case!(
    element_tag_block_comment,
    "comments/element_tag_block_comment"
);
compiler_case!(
    element_tag_block_multiline,
    "comments/element_tag_block_multiline"
);
compiler_case!(
    element_tag_comment_after_name,
    "comments/element_tag_comment_after_name"
);
compiler_case!(
    element_tag_comment_before_close,
    "comments/element_tag_comment_before_close"
);
compiler_case!(
    element_tag_comment_self_close,
    "comments/element_tag_comment_self_close"
);
compiler_case!(
    element_tag_multiple_comments,
    "comments/element_tag_multiple_comments"
);
compiler_case!(
    element_tag_comment_component,
    "comments/element_tag_comment_component"
);

compiler_case!(
    element_tag_mustache_comment_guard,
    "comments/element_tag_mustache_comment_guard"
);

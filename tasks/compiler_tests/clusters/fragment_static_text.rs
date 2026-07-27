use super::*;

compiler_case!(
    texts_split_by_comment,
    "fragment_static_text/texts_split_by_comment"
);
compiler_case!(
    slot_default_texts_split_by_slot,
    "fragment_static_text/slot_default_texts_split_by_slot"
);
compiler_case!(
    block_texts_split_by_comment,
    "fragment_static_text/block_texts_split_by_comment"
);

compiler_case!(single_text_guard, "fragment_static_text/single_text_guard");
compiler_case!(
    slot_default_contiguous_text_guard,
    "fragment_static_text/slot_default_contiguous_text_guard"
);
compiler_case!(
    text_expression_space_template_guard,
    "fragment_static_text/text_expression_space_template_guard"
);
compiler_case!(
    single_comment_guard,
    "fragment_static_text/single_comment_guard"
);
compiler_case!(
    slot_default_expressions_split_by_slot_guard,
    "fragment_static_text/slot_default_expressions_split_by_slot_guard"
);

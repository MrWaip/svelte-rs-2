use super::*;

compiler_case!(
    reactive_break_break_in_if,
    "legacy/reactive_break/break_in_if",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_break_break_in_for,
    "legacy/reactive_break/break_in_for",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_break_plain_break_in_for,
    "legacy/reactive_break/plain_break_in_for",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_break_labeled_break_other,
    "legacy/reactive_break/labeled_break_other",
    [prod, dev, ssr, ssr_dev]
);

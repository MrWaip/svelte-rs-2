use super::*;

compiler_case!(declaration_child_alias_weird_sample, "legacy/props/declaration/child_alias_weird_sample");
compiler_case!(declaration_gibberish_key_default_source, "legacy/props/declaration/gibberish_key_default_source");
compiler_case!(declaration_gibberish_key_nonsource, "legacy/props/declaration/gibberish_key_nonsource");
compiler_case!(declaration_identifier_key_nonsource_guard, "legacy/props/declaration/identifier_key_nonsource_guard");
compiler_case!(declaration_main_alias_weird_sample, "legacy/props/declaration/main_alias_weird_sample", ignore = "binding-pattern-routing slice 5 (legacy state/props declaration, transform) · tag binding-pattern-routing#5");
compiler_case!(declaration_nested_array_props, "legacy/props/declaration/nested_array_props");
compiler_case!(declaration_numeric_key_default_source, "legacy/props/declaration/numeric_key_default_source");
compiler_case!(declaration_numeric_key_nonsource, "legacy/props/declaration/numeric_key_nonsource");
compiler_case!(declaration_numeric_key_rest_after, "legacy/props/declaration/numeric_key_rest_after");
compiler_case!(declaration_shorthand_nonsource_guard, "legacy/props/declaration/shorthand_nonsource_guard");
compiler_case!(declaration_string_valid_ident_nonsource_guard, "legacy/props/declaration/string_valid_ident_nonsource_guard");

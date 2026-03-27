//! Utilities for extracting names from OXC BindingPattern.

use oxc_ast::ast::BindingPattern;

/// Collect all binding identifier names from a BindingPattern.
pub(crate) fn collect_binding_names(pat: &BindingPattern<'_>, out: &mut Vec<String>) {
    match pat {
        BindingPattern::BindingIdentifier(id) => out.push(id.name.to_string()),
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties { collect_binding_names(&prop.value, out); }
            if let Some(rest) = &obj.rest { collect_binding_names(&rest.argument, out); }
        }
        BindingPattern::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() { collect_binding_names(el, out); }
            if let Some(rest) = &arr.rest { collect_binding_names(&rest.argument, out); }
        }
        BindingPattern::AssignmentPattern(assign) => collect_binding_names(&assign.left, out),
    }
}

/// Collect `(name, has_default)` pairs from a destructuring BindingPattern.
/// `has_default` is true when the binding is inside an AssignmentPattern.
pub(crate) fn collect_destructure_bindings(pat: &BindingPattern<'_>, out: &mut Vec<(String, bool)>) {
    match pat {
        BindingPattern::BindingIdentifier(id) => out.push((id.name.to_string(), false)),
        BindingPattern::AssignmentPattern(assign) => {
            let mut names = Vec::new();
            collect_binding_names(&assign.left, &mut names);
            for name in names { out.push((name, true)); }
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties { collect_destructure_bindings(&prop.value, out); }
            if let Some(rest) = &obj.rest { collect_destructure_bindings(&rest.argument, out); }
        }
        BindingPattern::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() { collect_destructure_bindings(el, out); }
            if let Some(rest) = &arr.rest { collect_destructure_bindings(&rest.argument, out); }
        }
    }
}

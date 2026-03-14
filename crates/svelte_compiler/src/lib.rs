use svelte_diagnostics::{Diagnostic, Severity};

pub struct CompileResult {
    pub js: String,
}

/// Compile a Svelte source file to client-side JavaScript.
pub fn compile(source: &str) -> Result<CompileResult, Diagnostic> {
    let (component, parse_diagnostics) = svelte_parser::Parser::new(source).parse();

    // Treat parse errors as fatal.
    if let Some(diag) = parse_diagnostics.into_iter().find(|d| d.severity == Severity::Error) {
        return Err(diag);
    }

    let (analysis, diags) = svelte_analyze::analyze(&component);

    // Treat analysis errors as fatal.
    if let Some(diag) = diags.into_iter().find(|d| d.severity == Severity::Error) {
        return Err(diag);
    }

    let js = svelte_codegen_client::generate(&component, &analysis);

    Ok(CompileResult { js })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn check(source: &str, expected: &str) {
        let result = compile(source).unwrap_or_else(|e| panic!("compile failed: {e:?}"));
        assert_eq!(result.js, expected);
    }

    #[test]
    fn empty_component() {
        check(
            "",
            r#"import * as $ from "svelte/internal/client";
export default function App($$anchor) {}
"#,
        );
    }

    #[test]
    fn only_script() {
        check(
            r#"<script>
    let i = 10;
    i++;
</script>"#,
            r#"import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let i = 10;
	i++;
}
"#,
        );
    }

    #[test]
    fn single_interpolation_rune() {
        // Unmutated $state — treated as static (no template_effect)
        check(
            r#"<script>
    let name = $state();
</script>{name}"#,
            r#"import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let name = void 0;
	$.next();
	var text = $.text();
	text.nodeValue = name;
	$.append($$anchor, text);
}
"#,
        );
    }
}

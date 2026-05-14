import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	let status = $.derived(() => $$props.error ? "error" : $$props.fallback);
	var span = root();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(status)));
	$.append($$anchor, span);
}

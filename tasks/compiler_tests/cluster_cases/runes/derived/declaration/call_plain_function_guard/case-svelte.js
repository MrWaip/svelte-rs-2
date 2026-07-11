import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	function makeValue() {
		return 42;
	}
	const value = $.derived(makeValue);
	var span = root();
	var text = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.append($$anchor, span);
}

import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let count = $.mutable_source(0);
	function bump() {
		$.set(count, $.get(count) + 1);
	}
	var $$exports = { bump };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.append($$anchor, p);
	$.bind_prop($$props, "bump", bump);
	return $.pop($$exports);
}

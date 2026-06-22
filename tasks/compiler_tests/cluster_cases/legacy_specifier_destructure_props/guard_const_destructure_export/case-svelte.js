import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const { i, j } = {
		i: 1,
		j: 2
	};
	var $$exports = { i };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${i ?? ""}${j ?? ""}`));
	$.append($$anchor, p);
	$.bind_prop($$props, "i", i);
	return $.pop($$exports);
}

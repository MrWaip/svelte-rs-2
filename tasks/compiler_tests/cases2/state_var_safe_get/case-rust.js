import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var count = $.state(0);
	var name = "hello";
	$.set(count, $.safe_get(count) + 1);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.safe_get(count) ?? ""} hello`));
	$.append($$anchor, p);
}

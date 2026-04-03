import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let localVal = $.prop($$props, "value", 11, "fallback");
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, localVal()));
	$.append($$anchor, p);
	$.pop();
}

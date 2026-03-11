import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []), config = $.prop($$props, "config", 19, getDefault), label = $.prop($$props, "label", 3, "hello");
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, label()));
	$.append($$anchor, p);
}

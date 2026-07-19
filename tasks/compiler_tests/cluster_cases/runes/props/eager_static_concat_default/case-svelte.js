import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let title = $.prop($$props, "title", 3, "a" + "b" + "c");
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, title()));
	$.append($$anchor, p);
}

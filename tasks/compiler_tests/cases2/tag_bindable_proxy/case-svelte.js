import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let items = $.prop($$props, "items", 27, () => $.proxy([
		1,
		2,
		3
	]));
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, items()));
	$.append($$anchor, p);
	$.pop();
}

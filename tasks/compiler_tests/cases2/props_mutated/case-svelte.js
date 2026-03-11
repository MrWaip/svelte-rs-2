import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let count = $.prop($$props, "count", 7, 0);
	function increment() {
		$.update_prop(count);
	}
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, count()));
	$.append($$anchor, p);
}

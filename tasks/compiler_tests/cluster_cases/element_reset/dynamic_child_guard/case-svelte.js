import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><span> </span></div>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	var div = root();
	var span = $.child(div);
	var text = $.child(span, true);
	$.reset(span);
	$.reset(div);
	$.template_effect(() => $.set_text(text, foo()));
	$.append($$anchor, div);
}

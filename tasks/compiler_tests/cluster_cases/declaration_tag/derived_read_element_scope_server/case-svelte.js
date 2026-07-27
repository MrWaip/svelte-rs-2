import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div> <p> </p>`, 1);
export default function App($$anchor) {
	let outer = $.derived(() => Date.now());
	var fragment = root();
	var div = $.first_child(fragment);
	{
		let inner = $.derived(() => Date.now());
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(inner)));
	}
	var p = $.sibling(div, 2);
	var text_1 = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text_1, $.get(outer)));
	$.append($$anchor, fragment);
}

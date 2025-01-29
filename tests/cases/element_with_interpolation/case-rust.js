import * as $ from "svelte/internal/client";
var root = $.template(`<div> </div>`);
export default function App($$anchor) {
	let title = 10;
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, title));
	$.append($$anchor, div);
}

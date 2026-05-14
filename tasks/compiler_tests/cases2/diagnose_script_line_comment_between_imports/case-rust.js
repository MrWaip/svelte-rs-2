import * as $ from "svelte/internal/client";
import { a } from "mod-a";
// keep-this-comment
import { b } from "mod-b";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	const x = a + b;
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, x));
	$.append($$anchor, div);
}

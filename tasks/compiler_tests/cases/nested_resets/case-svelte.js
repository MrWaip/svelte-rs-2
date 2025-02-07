import * as $ from "svelte/internal/client";
var root = $.template(`<div><div><div> </div></div></div>`);
export default function App($$anchor) {
	let name = "world";
	var div = root();
	var div_1 = $.child(div);
	var div_2 = $.child(div_1);
	var text = $.child(div_2, true);
	$.reset(div_2);
	$.reset(div_1);
	$.reset(div);
	$.template_effect(() => $.set_text(text, name));
	$.append($$anchor, div);
}

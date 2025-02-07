import * as $ from "svelte/internal/client";
var root = $.template(`<div><div><div><div></div></div></div></div>`);
export default function App($$anchor) {
	let name = "world";
	var div = root();
	var div_1 = $.child(div);
	$.reset(div_2);
	$.reset(div_1);
	$.reset(div);
	$.append($$anchor, div);
}

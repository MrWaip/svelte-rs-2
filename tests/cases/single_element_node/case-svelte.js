import * as $ from "svelte/internal/client";
var root = $.template(`<div><div></div></div>`);
export default function App($$anchor) {
	let name = "";
	var div = root();
	var div_1 = $.child(div);
	$.reset(div);
	$.template_effect(() => {
		$.set_attribute(div, "name", name);
		$.set_attribute(div_1, "name", name);
	});
	$.append($$anchor, div);
}

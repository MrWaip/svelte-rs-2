import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let props = $.proxy({
		id: "a",
		style: "border-color: blue;"
	});
	let color = "red";
	var div = root();
	$.attribute_effect(div, () => ({
		...props,
		[$.STYLE]: { color }
	}));
	$.append($$anchor, div);
}

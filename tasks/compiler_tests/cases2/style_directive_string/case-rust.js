import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>String value</div>`);
export default function App($$anchor) {
	let size = $.state("16px");
	$.set(size, "20px");
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		color: "red",
		"font-size": $.get(size)
	}));
	$.append($$anchor, div);
}

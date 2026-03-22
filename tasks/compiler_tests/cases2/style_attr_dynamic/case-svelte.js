import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let color = $.state("red");
	function toggle() {
		$.set(color, "blue");
	}
	var div = root();
	$.template_effect(() => $.set_style(div, `color: ${$.get(color) ?? ""}; font-size: 14px`));
	$.append($$anchor, div);
}

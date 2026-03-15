import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>Concat value</div>`);
export default function App($$anchor) {
	let shade = $.state("500");
	$.set(shade, "600");
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, "", styles, { color: `red-${$.get(shade) ?? ""}` }));
	$.append($$anchor, div);
}

import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let onmouseenter = $.prop($$props, "onmouseenter", 8, undefined);
	var div = root();
	$.event("mouseenter", div, function(...$$args) {
		onmouseenter()?.apply(this, $$args);
	});
	$.append($$anchor, div);
}

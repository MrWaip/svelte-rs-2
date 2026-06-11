import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let on = $.prop($$props, "on", 8, undefined);
	var div = root();
	$.event("", div, function(...$$args) {
		on()?.apply(this, $$args);
	});
	$.append($$anchor, div);
}

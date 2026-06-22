import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let active = true;
	var div = root();
	$.set_class(div, 1, "", null, {}, { active });
	$.append($$anchor, div);
}

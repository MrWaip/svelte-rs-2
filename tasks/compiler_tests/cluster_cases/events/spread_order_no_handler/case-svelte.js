import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let rest = $.prop($$props, "rest", 24, () => ({}));
	var div = root();
	$.attribute_effect(div, () => ({ ...rest() }));
	$.append($$anchor, div);
}

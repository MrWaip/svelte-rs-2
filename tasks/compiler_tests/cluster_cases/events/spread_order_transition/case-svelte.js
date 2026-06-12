import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { slide } from "svelte/transition";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let rest = $.prop($$props, "rest", 24, () => ({}));
	var div = root();
	$.attribute_effect(div, () => ({ ...rest() }));
	$.transition(3, div, () => slide);
	$.append($$anchor, div);
}

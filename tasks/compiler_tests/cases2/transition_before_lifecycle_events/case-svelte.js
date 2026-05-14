import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let animated = $.state(false);
	var div = root();
	$.transition(2, div, () => fade);
	$.event("introstart", div, () => $.set(animated, true));
	$.event("outroend", div, () => $.set(animated, false));
	$.append($$anchor, div);
}

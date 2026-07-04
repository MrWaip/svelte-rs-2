import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	let n = $.mutable_source(0);
	var button = root();
	$.event("click", button, () => $.update(n));
	$.append($$anchor, button);
}

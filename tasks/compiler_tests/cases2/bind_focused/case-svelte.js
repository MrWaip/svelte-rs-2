import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	let focused = $.state(false);
	var button = root();
	$.bind_focused(button, ($$value) => $.set(focused, $$value));
	$.append($$anchor, button);
}

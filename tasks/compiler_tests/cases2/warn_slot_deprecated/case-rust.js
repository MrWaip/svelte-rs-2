import * as $ from "svelte/internal/client";
var root = $.from_html(`<slot></slot>`);
export default function App($$anchor) {
	var slot = root();
	$.append($$anchor, slot);
}

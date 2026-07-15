import * as $ from "svelte/internal/client";
var root = $.from_html(`<div id="x" class="svelte-1ghvvfz">a</div>`);
export default function App($$anchor) {
	var div = root();
	$.set_style(div, "", {}, { color: "red" });
	$.append($$anchor, div);
}

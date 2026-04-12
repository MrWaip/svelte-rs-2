import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="foo svelte-70hmnn">hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}

import * as $ from "svelte/internal/client";
var root = $.from_html(`<div is="native">hello</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}

import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="x svelte-bukzi4"><span>hi</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}

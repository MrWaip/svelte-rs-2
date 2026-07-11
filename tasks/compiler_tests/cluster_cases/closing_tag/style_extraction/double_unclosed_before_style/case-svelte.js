import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="x svelte-bukzi4"><span><b>hi</b></span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}

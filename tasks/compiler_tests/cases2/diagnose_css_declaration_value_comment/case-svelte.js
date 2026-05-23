import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box svelte-b8u45r">x</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}

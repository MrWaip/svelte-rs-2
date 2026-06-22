import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-1ic36ag">x</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}

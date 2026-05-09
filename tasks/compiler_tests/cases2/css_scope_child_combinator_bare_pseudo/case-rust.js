import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="benefit svelte-1vvtp4y"><span class="svelte-1vvtp4y">icon</span> <span class="svelte-1vvtp4y">text</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}

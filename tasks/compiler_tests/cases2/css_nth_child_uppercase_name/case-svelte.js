import * as $ from "svelte/internal/client";
var root = $.from_html(`<ul><li class="a svelte-ym30ly">x</li></ul>`);
export default function App($$anchor) {
	var ul = root();
	$.append($$anchor, ul);
}

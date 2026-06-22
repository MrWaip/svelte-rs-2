import * as $ from "svelte/internal/client";
var root = $.from_html(`<ul><li class="a svelte-18ywyf0">x</li></ul>`);
export default function App($$anchor) {
	var ul = root();
	$.append($$anchor, ul);
}

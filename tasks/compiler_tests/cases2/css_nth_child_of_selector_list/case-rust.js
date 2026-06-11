import * as $ from "svelte/internal/client";
var root = $.from_html(`<ul><li class="a svelte-1poaahi">x</li><li class="b">y</li></ul>`);
export default function App($$anchor) {
	var ul = root();
	$.append($$anchor, ul);
}

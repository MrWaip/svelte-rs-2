import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root_2 = $.from_html(`<p class="svelte-5iy3wu">hi</p>`);
var root = $.from_html(`<div class="wrap svelte-5iy3wu"><!></div>`);
export default function App($$anchor) {
	var div = root();
	var node = $.child(div);
	Child(node, { $$slots: { x: ($$anchor, $$slotProps) => {
		var p = root_2();
		$.append($$anchor, p);
	} } });
	$.reset(div);
	$.append($$anchor, div);
}

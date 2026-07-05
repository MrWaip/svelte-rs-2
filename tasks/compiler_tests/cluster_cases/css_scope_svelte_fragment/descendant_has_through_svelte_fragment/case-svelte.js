import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<p class="svelte-1ktsoc1">hi</p>`);
var root_1 = $.from_html(`<div class="wrap svelte-1ktsoc1"><!></div>`);
export default function App($$anchor) {
	var div = root_1();
	var node = $.child(div);
	Child(node, { $$slots: { x: ($$anchor, $$slotProps) => {
		var p = root();
		$.append($$anchor, p);
	} } });
	$.reset(div);
	$.append($$anchor, div);
}

import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root_2 = $.from_html(`<p class="svelte-1u3aiak">hi</p>`);
var root = $.from_html(`<div class="wrap svelte-1u3aiak"><!></div>`);
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

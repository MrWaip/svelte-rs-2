import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root_2 = $.from_html(`<div class="mid svelte-1pc7tr6"><p class="svelte-1pc7tr6">hi</p></div>`);
var root = $.from_html(`<div class="wrap svelte-1pc7tr6"><!></div>`);
export default function App($$anchor) {
	var div = root();
	var node = $.child(div);
	Child(node, { $$slots: { x: ($$anchor, $$slotProps) => {
		var div_1 = root_2();
		$.append($$anchor, div_1);
	} } });
	$.reset(div);
	$.append($$anchor, div);
}

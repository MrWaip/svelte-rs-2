import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<p class="svelte-16b3ya8">hi</p>`);
export default function App($$anchor) {
	Child($$anchor, { $$slots: { x: ($$anchor, $$slotProps) => {
		var p = root();
		$.append($$anchor, p);
	} } });
}

import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root_2 = $.from_html(`<p class="svelte-16b3ya8">hi</p>`);
export default function App($$anchor) {
	Child($$anchor, { $$slots: { x: ($$anchor, $$slotProps) => {
		var p = root_2();
		$.append($$anchor, p);
	} } });
}

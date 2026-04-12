import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root_1 = $.from_html(`<p slot="item" let:item=""></p>`);
export default function App($$anchor) {
	List($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		var p = root_1();
		p.textContent = item.text;
		$.append($$anchor, p);
	} } });
}

import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root_2 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	List($$anchor, { $$slots: { item: ($$anchor, $$slotProps) => {
		var p = root_2();
		p.textContent = item.text;
		$.append($$anchor, p);
	} } });
}

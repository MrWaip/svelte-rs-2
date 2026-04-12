import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import List from "./List.svelte";
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	List($$anchor, {
		let:item: true,
		children: ($$anchor, $$slotProps) => {
			var p = root_1();
			p.textContent = item;
			$.append($$anchor, p);
		},
		$$slots: { default: true }
	});
}

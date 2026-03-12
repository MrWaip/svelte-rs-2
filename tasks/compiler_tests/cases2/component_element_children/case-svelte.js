import * as $ from "svelte/internal/client";
import Card from "./Card.svelte";
var root_1 = $.from_html(`<p>Hello world</p>`);
export default function App($$anchor) {
	Card($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var p = root_1();
			$.append($$anchor, p);
		},
		$$slots: { default: true }
	});
}

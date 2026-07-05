import * as $ from "svelte/internal/client";
import Card from "./Card.svelte";
var root = $.from_html(`<p>Hello world</p>`);
export default function App($$anchor) {
	Card($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var p = root();
			$.append($$anchor, p);
		},
		$$slots: { default: true }
	});
}

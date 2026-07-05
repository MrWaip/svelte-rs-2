import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let value = 5;
	Widget($$anchor, {
		children: ($$anchor, $$slotProps) => {
			const doubled = $.derived(() => value * 2);
			var p = root();
			p.textContent = $.get(doubled);
			$.append($$anchor, p);
		},
		$$slots: { default: true }
	});
}

import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let name = "world";
	Card($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var p = root_1();
			p.textContent = "Content world";
			$.append($$anchor, p);
		},
		$$slots: { default: true }
	});
}

import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<h2>Hello</h2>`);
var root_2 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let name = "world";
	{
		const title = ($$anchor) => {
			var h2 = root_1();
			$.append($$anchor, h2);
		};
		Card($$anchor, {
			title,
			children: ($$anchor, $$slotProps) => {
				var p = root_2();
				p.textContent = "Content world";
				$.append($$anchor, p);
			},
			$$slots: {
				title: true,
				default: true
			}
		});
	}
}

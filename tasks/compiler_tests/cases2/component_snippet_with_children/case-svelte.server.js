import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let name = "world";
	{
		function title($$renderer) {
			$$renderer.push(`<h2>Hello</h2>`);
		}
		Card($$renderer, {
			title,
			children: ($$renderer) => {
				$$renderer.push(`<p>Content world</p>`);
			},
			$$slots: {
				title: true,
				default: true
			}
		});
	}
}

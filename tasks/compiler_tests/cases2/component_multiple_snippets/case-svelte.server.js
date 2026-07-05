import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		function header($$renderer) {
			$$renderer.push(`<h1>Header</h1>`);
		}
		function footer($$renderer) {
			$$renderer.push(`<p>Footer</p>`);
		}
		Card($$renderer, {
			header,
			footer,
			$$slots: {
				header: true,
				footer: true
			}
		});
	}
}

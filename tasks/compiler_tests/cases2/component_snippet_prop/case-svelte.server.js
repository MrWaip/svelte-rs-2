import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	{
		function header($$renderer) {
			$$renderer.push(`<h2>Title 0</h2>`);
		}
		Dialog($$renderer, {
			header,
			$$slots: { header: true }
		});
	}
}

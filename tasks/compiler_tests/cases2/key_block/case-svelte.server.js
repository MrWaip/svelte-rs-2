import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	count = 1;
	$$renderer.push(`<!---->`);
	{
		$$renderer.push(`<div>${$.escape(count)}</div>`);
	}
	$$renderer.push(`<!---->`);
}

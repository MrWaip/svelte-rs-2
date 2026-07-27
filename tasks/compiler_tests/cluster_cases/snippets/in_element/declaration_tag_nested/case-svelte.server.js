import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div>`);
	{
		const x = 1;
		$$renderer.push(`<span></span>`);
	}
	$$renderer.push(`</div>`);
}

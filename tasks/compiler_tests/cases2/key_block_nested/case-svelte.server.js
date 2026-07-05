import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	count = 1;
	$$renderer.push(`<div>before <!---->`);
	{
		$$renderer.push(`<span>${$.escape(count)}</span>`);
	}
	$$renderer.push(`<!----> after</div>`);
}

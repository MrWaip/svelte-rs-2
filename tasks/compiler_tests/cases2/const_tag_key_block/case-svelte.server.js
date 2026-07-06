import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 1;
	$$renderer.push(`<!---->`);
	{
		const doubled = count * 2;
		$$renderer.push(`<p>2</p>`);
	}
	$$renderer.push(`<!---->`);
}

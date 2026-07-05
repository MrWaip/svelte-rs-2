import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = ["a", "b"];
	$$renderer.push(`<!--[-->`);
	{
		const count = items.length;
		$$renderer.push(`<p>Count: ${$.escape(count)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
}

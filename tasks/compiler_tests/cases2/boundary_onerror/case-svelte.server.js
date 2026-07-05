import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handleError(error) {
		console.error(error);
	}
	$$renderer.push(`<!--[-->`);
	{
		$$renderer.push(`<p>content</p>`);
	}
	$$renderer.push(`<!--]-->`);
}

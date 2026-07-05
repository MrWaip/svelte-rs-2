import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handleError(e) {
		console.error(e);
	}
	$$renderer.push(`<!--[-->`);
	{
		$$renderer.push(`<!---->x`);
	}
	$$renderer.push(`<!--]-->`);
}

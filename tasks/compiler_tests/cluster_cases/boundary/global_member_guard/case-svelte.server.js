import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!--[-->`);
	{
		$$renderer.push(`<!---->x`);
	}
	$$renderer.push(`<!--]-->`);
}

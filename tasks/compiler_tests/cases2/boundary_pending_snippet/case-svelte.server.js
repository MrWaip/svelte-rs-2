import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!--[!-->`);
	{
		$$renderer.push(`<p>loading...</p>`);
	}
	$$renderer.push(`<!--]-->`);
}

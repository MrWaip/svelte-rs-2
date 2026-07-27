import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let show = true;
	$$renderer.push(`<button>toggle</button> `);
	if (show) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<!doctype html=""/>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	if (count) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>Active</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

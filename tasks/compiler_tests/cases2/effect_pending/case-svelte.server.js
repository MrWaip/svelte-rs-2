import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<p>${$.escape(0)}</p> `);
	if (0) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>Loading 0</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<p>Done</p>`);
	}
	$$renderer.push(`<!--]-->`);
}

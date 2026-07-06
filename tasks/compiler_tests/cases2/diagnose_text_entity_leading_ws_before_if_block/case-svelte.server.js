import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div><a href="/x">link</a> text more `);
	if (true) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`x`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></div>`);
}

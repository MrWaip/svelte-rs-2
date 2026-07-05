import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (true) {
		$$renderer.push("<!--[0-->");
		const c = "x";
		const b = c;
		const a = b;
		$$renderer.push(`<p>x</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

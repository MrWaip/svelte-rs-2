import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (true) {
		$$renderer.push("<!--[0-->");
		const a = "x";
		const b = "y";
		$$renderer.push(`<p>xy</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

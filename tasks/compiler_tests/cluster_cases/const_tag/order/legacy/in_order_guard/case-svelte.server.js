import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	if (true) {
		$$renderer.push("<!--[0-->");
		const bar = "world";
		const foo = bar;
		$$renderer.push(`<p>world</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

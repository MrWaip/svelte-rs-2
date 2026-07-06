import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = "world";
	if (n) {
		$$renderer.push("<!--[0-->");
		const bar = n;
		const foo = bar;
		$$renderer.push(`<p>world</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

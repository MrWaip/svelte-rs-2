import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = [{ a: 1 }, { b: 2 }];
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const [{ a }, { b }] = pairs;
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

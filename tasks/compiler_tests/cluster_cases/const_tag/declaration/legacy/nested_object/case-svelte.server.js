import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = {
		p: { a: 1 },
		q: { b: 2 }
	};
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const { p: { a }, q: { b } } = pairs;
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

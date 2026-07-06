import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = [[1, 2], [3, 4]];
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const [[a, b], [c, d]] = pairs;
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

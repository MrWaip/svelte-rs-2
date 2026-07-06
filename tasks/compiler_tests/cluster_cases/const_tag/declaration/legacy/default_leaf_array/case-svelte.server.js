import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = [1];
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const [a = 10, b = 20] = pairs;
		$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

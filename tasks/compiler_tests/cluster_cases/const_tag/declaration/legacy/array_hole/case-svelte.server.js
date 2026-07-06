import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = [
		1,
		2,
		3
	];
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const [a, , c] = pairs;
		$$renderer.push(`<button>${$.escape(a)}${$.escape(c)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

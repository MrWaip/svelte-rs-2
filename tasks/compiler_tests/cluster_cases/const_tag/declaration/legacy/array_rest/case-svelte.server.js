import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = [
		1,
		2,
		3
	];
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const [a, ...rest] = pairs;
		$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.length)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const k = "z";
	let pairs = { z: 1 };
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const { [k]: v } = pairs;
		$$renderer.push(`<button>${$.escape(v)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

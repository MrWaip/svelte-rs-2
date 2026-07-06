import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = {};
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const { p: { a } = {} } = pairs;
		$$renderer.push(`<button>${$.escape(a)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

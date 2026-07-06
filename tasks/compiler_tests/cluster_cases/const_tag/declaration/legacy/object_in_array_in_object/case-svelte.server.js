import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = { outer: [{ inner: 1 }] };
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const { outer: [{ inner }] } = pairs;
		$$renderer.push(`<button>${$.escape(inner)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

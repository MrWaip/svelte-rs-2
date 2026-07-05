import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pairs = {
		"a-b": 1,
		"c d": 2
	};
	if (pairs) {
		$$renderer.push("<!--[0-->");
		const { "a-b": ab, "c d": cd } = pairs;
		$$renderer.push(`<button>${$.escape(ab)}${$.escape(cd)}</button>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

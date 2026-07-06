import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function fade(node) {
		return {};
	}
	let show = true;
	if (show) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div></div>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

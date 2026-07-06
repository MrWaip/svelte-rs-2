import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function is_even() {
		return count % 2 === 0;
	}
	if (is_even()) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>even</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<p>odd</p>`);
	}
	$$renderer.push(`<!--]-->`);
}

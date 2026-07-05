import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function tick() {
		count += 1;
		return count;
	}
	let show = true;
	if (show) {
		$$renderer.push("<!--[0-->");
		const value = tick();
		$$renderer.push(`<p>${$.escape(value)}</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}

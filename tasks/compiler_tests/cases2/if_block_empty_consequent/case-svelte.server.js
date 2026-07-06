import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let show = true;
	if (show) {
		$$renderer.push("<!--[0-->");
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
